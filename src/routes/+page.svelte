<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { get } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';
  import { listen } from '@tauri-apps/api/event';
  import { getCurrentWebview } from '@tauri-apps/api/webview';
  import { Store } from '@tauri-apps/plugin-store';
  import { appStore, createDefaultSettings, zoomLevel } from '$lib/stores/app';
  import type { TabState, AppSettings, Theme } from '$lib/stores/app';

  import MenuBar from '$lib/components/MenuBar.svelte';
  import TabBar from '$lib/components/TabBar.svelte';
  import EditorArea from '$lib/components/EditorArea.svelte';
  import PreviewPanel from '$lib/components/PreviewPanel.svelte';
  import FindReplace from '$lib/components/FindReplace.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import SettingsPanel from '$lib/components/SettingsPanel.svelte';
  import FileChangedNotification from '$lib/components/FileChangedNotification.svelte';
  import GoToLine from '$lib/components/GoToLine.svelte';

  // Reactive state via writable stores (imported with $ syntax)
  let tabs = $state<TabState[]>([]);
  let activeTabId = $state<string | null>(null);
  let settings = $state<AppSettings>(createDefaultSettings());
  let recentFiles = $state<string[]>([]);
  let fileChangedNotif = $state<{ path: string; visible: boolean } | null>(null);

  // Local UI state
  let showSettings = $state(false);
  let showGoToLine = $state(false);
  let editorView = $state<import('@codemirror/view').EditorView | null>(null);
  let editorScrollPercent = $state(0);

  // Derived values from active tab
  let activeTab = $derived(tabs.find((t) => t.id === activeTabId) ?? null);
  let activeContent = $derived(activeTab?.content ?? '');
  let activeLanguage = $derived(activeTab?.language ?? 'markdown');
  let activeEncoding = $derived(activeTab?.encoding ?? 'utf-8');
  let activeLineEnding = $derived(activeTab?.lineEnding ?? 'LF');

  let wordCount = $derived(activeContent.trim() ? activeContent.trim().split(/\s+/).length : 0);

  let charCount = $derived(activeContent.length);

  // Sync local state to stores
  $effect(() => {
    appStore.tabs.set(tabs);
  });
  $effect(() => {
    appStore.activeTabId.set(activeTabId);
  });

  // Dynamic window title
  $effect(() => {
    const title = activeTab
      ? `NoteForge — ${activeTab.fileName}`
      : 'NoteForge';
    invoke('set_window_title', { title }).catch(() => {});
  });

  // Subscribe to stores (for settings, recentFiles, fileChangedNotification)
  onMount(() => {
    const unsubSettings = appStore.settings.subscribe((s) => { settings = { ...s }; });
    const unsubRecent = appStore.recentFiles.subscribe((r) => { recentFiles = r; });
    const unsubNotif = appStore.fileChangedNotification.subscribe((n) => { fileChangedNotif = n; });

    loadRecentFiles();
    restoreTabs();

    // Listen for file-changed events from Rust file watcher
    let unlisten: (() => void) | undefined;
    listen<{ path: string }>('file-changed', (event) => {
      appStore.fileChangedNotification.set({ path: event.payload.path, visible: true });
    }).then((fn) => { unlisten = fn; });

    // Listen for print trigger events from Rust
    let unlistenPrint: (() => void) | undefined;
    listen('trigger-print', () => {
      window.print();
    }).then((fn) => { unlistenPrint = fn; });

    return () => {
      unsubSettings();
      unsubRecent();
      unsubNotif();
      unlisten?.();
      unlistenPrint?.();
    };
  });

  // Autosave timer
  let autosaveTimer: ReturnType<typeof setInterval> | null = null;

  $effect(() => {
    if (autosaveTimer) {
      clearInterval(autosaveTimer);
      autosaveTimer = null;
    }
    if (settings.autosaveInterval > 0) {
      autosaveTimer = setInterval(() => {
        saveDirtyTabs();
      }, settings.autosaveInterval * 1000);
    }
  });

  onDestroy(() => {
    if (autosaveTimer) clearInterval(autosaveTimer);
    persistTabs();
  });

  async function saveDirtyTabs() {
    for (const tab of tabs) {
      if (tab.isDirty && tab.filePath) {
        try {
          await invoke('save_file', {
            path: tab.filePath,
            content: tab.content,
            encoding: tab.encoding,
            lineEnding: tab.lineEnding,
          });
          updateTab(tab.id, { isDirty: false, originalContent: tab.content });
        } catch (e) {
          console.error('Autosave failed:', e);
        }
      }
    }
  }

  async function loadRecentFiles() {
    try {
      const files = await invoke<{ path: string; name: string }[]>('get_recent_files');
      appStore.recentFiles.set(files.map((f) => f.path));
    } catch {
      // command may not be available yet
    }
  }

  // Persist open tabs to store on close, restore on relaunch
  async function persistTabs() {
    try {
      const store = await Store.load('session.json');
      const tabData = tabs
        .filter((t) => t.filePath) // only persist tabs with a file path
        .map((t) => ({ filePath: t.filePath, activeTabId: t.id === activeTabId }));
      await store.set('openTabs', JSON.stringify(tabData));
      await store.save();
    } catch {
      // persistence may fail
    }
  }

  async function restoreTabs() {
    try {
      const store = await Store.load('session.json');
      const tabDataStr = await store.get<string>('openTabs');
      if (!tabDataStr) {
        // Cold start: no session to restore — open one empty Untitled tab
        handleNew();
        return;
      }
      const tabData: { filePath: string; activeTabId: boolean }[] = JSON.parse(tabDataStr);
      
      // Find which tab was active before (by index), since IDs are regenerated
      let activeIndex = -1;
      for (let i = 0; i < tabData.length; i++) {
        if (tabData[i].activeTabId) {
          activeIndex = i;
          break;
        }
      }

      for (const td of tabData) {
        await openFile(td.filePath);
      }

      // Set active tab by index, not by old ID
      if (activeIndex >= 0 && activeIndex < tabs.length) {
        activeTabId = tabs[activeIndex].id;
      }
    } catch {
      // session may not exist on first launch — open empty tab
      handleNew();
    }
  }

  function generateId(): string {
    return Date.now().toString(36) + Math.random().toString(36).slice(2, 7);
  }

  function updateTab(id: string, updates: Partial<TabState>) {
    tabs = tabs.map((t) => (t.id === id ? { ...t, ...updates } : t));
  }

  // Detect language from file extension
  function detectLanguage(fileName: string): string {
    const ext = fileName.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
      md: 'markdown',
      markdown: 'markdown',
      js: 'javascript',
      mjs: 'javascript',
      cjs: 'javascript',
      ts: 'javascript',
      json: 'json',
      py: 'python',
      html: 'html',
      htm: 'html',
      css: 'css',
      txt: 'text',
    };
    return map[ext] || 'text';
  }

  // =====================
  // File Operations
  // =====================

  async function handleNew() {
    const id = generateId();
    const newTab: TabState = {
      id,
      filePath: '',
      fileName: 'Untitled',
      content: '',
      originalContent: '',
      encoding: settings.defaultEncoding,
      lineEnding: 'LF',
      isDirty: false,
      language: 'markdown',
    };
    tabs = [...tabs, newTab];
    activeTabId = id;
  }

  async function handleOpen() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          { name: 'Text Files', extensions: ['txt', 'md', 'json', 'js', 'ts', 'py', 'html', 'css', 'yaml', 'yml'] },
          { name: 'All Files', extensions: ['*'] },
        ],
      });
      if (typeof selected === 'string') {
        await openFile(selected);
      }
    } catch {
      // dialog cancelled
    }
  }

  async function openFile(path: string) {
    // Check if already open
    const existing = tabs.find((t) => t.filePath === path);
    if (existing) {
      activeTabId = existing.id;
      return;
    }

    try {
      const result = await invoke<{ content: string; encoding: string; lineEnding: string; language: string }>('open_file', { path });
      const fileName = path.split(/[/\\]/).pop() ?? 'Untitled';
      const id = generateId();
      const newTab: TabState = {
        id,
        filePath: path,
        fileName,
        content: result.content,
        originalContent: result.content,
        encoding: result.encoding,
        lineEnding: result.lineEnding,
        isDirty: false,
        language: result.language || detectLanguage(fileName),
      };
      tabs = [...tabs, newTab];
      activeTabId = id;

      // Add to recent files
      await invoke('add_recent_file', { path, name: fileName });
      await loadRecentFiles();

      // Start file watcher
      await invoke('start_file_watcher', { path });
    } catch (e) {
      console.error('Failed to open file:', e);
    }
  }

  async function handleSave() {
    if (!activeTab) return;
    if (activeTab.filePath) {
      await saveFileToDisk(activeTab);
    } else {
      await handleSaveAs();
    }
  }

  async function handleSaveAs() {
    if (!activeTab) return;
    try {
      const selected = await save({
        defaultPath: activeTab.fileName,
        filters: [
          { name: 'Text Files', extensions: ['txt', 'md', 'json', 'js', 'ts', 'py', 'html', 'css'] },
          { name: 'All Files', extensions: ['*'] },
        ],
      });
      if (typeof selected === 'string') {
        const fileName = selected.split(/[/\\]/).pop() ?? 'Untitled';
        updateTab(activeTab.id, { filePath: selected, fileName, language: detectLanguage(fileName) });
        await saveFileToDisk({ ...activeTab, filePath: selected });
        await invoke('add_recent_file', { path: selected, name: fileName });
        await loadRecentFiles();
        await invoke('start_file_watcher', { path: selected });
      }
    } catch {
      // dialog cancelled
    }
  }

  async function saveFileToDisk(tab: TabState) {
    if (!tab.filePath) return;
    try {
      await invoke('save_file', {
        path: tab.filePath,
        content: tab.content,
        encoding: tab.encoding,
        lineEnding: tab.lineEnding,
      });
      updateTab(tab.id, { isDirty: false, originalContent: tab.content });
    } catch (e) {
      console.error('Save failed:', e);
    }
  }

  async function handleExportPdf() {
    try {
      await invoke('export_pdf');
    } catch (e) {
      console.error('Export PDF failed:', e);
    }
  }

  function handleCloseTab(id: string) {
    const tab = tabs.find((t) => t.id === id);
    if (tab?.isDirty) {
      const confirmed = confirm(`${tab.fileName} has unsaved changes. Close anyway?`);
      if (!confirmed) return;
    }

    const idx = tabs.findIndex((t) => t.id === id);
    tabs = tabs.filter((t) => t.id !== id);

    if (tabs.length > 0) {
      if (activeTabId === id) {
        // Switch to the nearest tab or the last one available
        const newIdx = Math.min(idx, tabs.length - 1);
        activeTabId = tabs[newIdx]?.id ?? tabs[tabs.length - 1].id;
      }
    } else {
      activeTabId = null;
    }

    invoke('stop_file_watcher').catch(() => {});
    if (activeTab?.filePath) {
      invoke('start_file_watcher', { path: activeTab.filePath }).catch(() => {});
    }
  }

  async function handleOpenRecent(path: string) {
    await openFile(path);
  }

  function handleContentChange(newContent: string) {
    if (!activeTabId) return;
    const tab = tabs.find((t) => t.id === activeTabId);
    if (!tab) return;
    updateTab(activeTabId, {
      content: newContent,
      isDirty: newContent !== tab.originalContent,
    });
  }

  // =====================
  // Toggle Functions
  // =====================

  function togglePreview() {
    settings = { ...settings, showPreview: !settings.showPreview };
    appStore.settings.set(settings);
  }

  function toggleFindReplace() {
    settings = { ...settings, showFindReplace: !settings.showFindReplace };
    appStore.settings.set(settings);
  }

  function toggleWordWrap() {
    settings = { ...settings, wordWrap: !settings.wordWrap };
    appStore.settings.set(settings);
  }

  function cycleTheme() {
    const themes: Theme[] = ['light', 'dark', 'sepia'];
    const idx = themes.indexOf(settings.theme);
    const next = themes[(idx + 1) % themes.length];
    settings = { ...settings, theme: next };
    appStore.settings.set(settings);
  }

  function openSettingsPanel() {
    showSettings = true;
  }

  function zoomIn() {
    const newSize = Math.min(settings.fontSize + 2, 72);
    settings = { ...settings, fontSize: newSize };
    appStore.settings.set(settings);
  }

  function zoomOut() {
    const newSize = Math.max(settings.fontSize - 2, 8);
    settings = { ...settings, fontSize: newSize };
    appStore.settings.set(settings);
  }

  function zoomReset() {
    settings = { ...settings, fontSize: createDefaultSettings().fontSize };
    appStore.settings.set(settings);
  }

  function toggleGoToLine() {
    showGoToLine = !showGoToLine;
  }

  // =====================
  // File Changed Notification
  // =====================

  async function handleReloadFile(path: string) {
    try {
      const result = await invoke<{ content: string; encoding: string; lineEnding: string; language: string }>('open_file', { path });
      const tab = tabs.find((t) => t.filePath === path);
      if (tab) {
        updateTab(tab.id, {
          content: result.content,
          originalContent: result.content,
          isDirty: false,
        });
      }
    } catch (e) {
      console.error('Reload failed:', e);
    }
    appStore.fileChangedNotification.set(null);
  }

  function handleIgnoreFileChange() {
    appStore.fileChangedNotification.set(null);
  }

  // =====================
  // Keyboard Shortcuts
  // =====================

  function handleKeydown(e: KeyboardEvent) {
    const ctrl = e.ctrlKey || e.metaKey;

    // Ctrl+N — New
    if (ctrl && !e.shiftKey && e.key === 'n') {
      e.preventDefault();
      handleNew();
    }
    // Ctrl+O — Open
    if (ctrl && !e.shiftKey && e.key === 'o') {
      e.preventDefault();
      handleOpen();
    }
    // Ctrl+S — Save
    if (ctrl && !e.shiftKey && e.key === 's') {
      e.preventDefault();
      handleSave();
    }
    // Ctrl+Shift+S — Save As
    if (ctrl && e.shiftKey && e.key === 'S') {
      e.preventDefault();
      handleSaveAs();
    }
    // Ctrl+Shift+M — Toggle Preview
    if (ctrl && e.shiftKey && e.key === 'M') {
      e.preventDefault();
      togglePreview();
    }
    // Ctrl+H — Find & Replace
    if (ctrl && !e.shiftKey && e.key === 'h') {
      e.preventDefault();
      toggleFindReplace();
    }
    // Alt+Z — Word Wrap
    if (e.altKey && e.key === 'z') {
      e.preventDefault();
      toggleWordWrap();
    }
    // Ctrl+Shift+T — Cycle Theme
    if (ctrl && e.shiftKey && e.key === 'T') {
      e.preventDefault();
      cycleTheme();
    }
    // Ctrl+W — Close tab
    if (ctrl && !e.shiftKey && e.key === 'w') {
      e.preventDefault();
      if (activeTabId) handleCloseTab(activeTabId);
    }
    // Ctrl+T — New tab
    if (ctrl && !e.shiftKey && e.key === 't') {
      e.preventDefault();
      handleNew();
    }
    // Ctrl+G — Go to line
    if (ctrl && !e.shiftKey && e.key === 'g') {
      e.preventDefault();
      toggleGoToLine();
    }
    // Ctrl++ — Zoom in (increase font size)
    if (ctrl && !e.shiftKey && (e.key === '=' || e.key === '+')) {
      e.preventDefault();
      zoomIn();
    }
    // Ctrl+- — Zoom out (decrease font size)
    if (ctrl && !e.shiftKey && e.key === '-') {
      e.preventDefault();
      zoomOut();
    }
    // Ctrl+0 — Reset zoom and font size
    if (ctrl && !e.shiftKey && e.key === '0') {
      e.preventDefault();
      zoomReset();
      zoomLevel.set(100);
    }
  }

  // =====================
  // Drag & Drop (via Tauri native API)
  // =====================

  let isDragging = $state(false);

  onMount(() => {
    // Use Tauri's native drag-drop event — HTML5 file.path is undefined in modern browsers
    const unlistenDragDrop = getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        isDragging = true;
      } else if (event.payload.type === 'drop') {
        isDragging = false;
        const paths = event.payload.paths;
        if (paths && paths.length > 0) {
          openFile(paths[0]);
        }
      } else {
        // 'cancel'
        isDragging = false;
      }
    });

    return () => {
      unlistenDragDrop.then((fn) => fn());
    };
  });

  // Set editor ref for FindReplace
  function handleEditorViewCreated(view: import('@codemirror/view').EditorView) {
    editorView = view;
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="app-shell flex flex-col h-screen overflow-hidden"
  style="color: var(--text);"
  role="application"
>
  {#if isDragging}
    <div class="fixed inset-0 z-40 flex items-center justify-center pointer-events-none"
         style="background-color: rgba(0, 102, 204, 0.1); border: 3px dashed var(--accent);">
      <span class="text-lg font-semibold" style="color: var(--accent);">Drop file to open</span>
    </div>
  {/if}

  <MenuBar
    onNew={handleNew}
    onOpen={handleOpen}
    onSave={handleSave}
    onSaveAs={handleSaveAs}
    onExportPdf={handleExportPdf}
    onTogglePreview={togglePreview}
    onToggleFindReplace={toggleFindReplace}
    onToggleWordWrap={toggleWordWrap}
    onToggleTheme={cycleTheme}
    onOpenSettings={openSettingsPanel}
    recentFiles={recentFiles}
    onOpenRecent={handleOpenRecent}
    showPreview={settings.showPreview}
    showFindReplace={settings.showFindReplace}
    wordWrap={settings.wordWrap}
    currentTheme={settings.theme}
  />

  <TabBar
    tabs={tabs}
    activeTabId={activeTabId}
    onSwitchTab={(id) => { activeTabId = id; }}
    onCloseTab={handleCloseTab}
    onNewTab={handleNew}
  />

  <div class="flex flex-1 min-h-0">
    {#if activeTab}
      <div class="flex flex-col flex-1 min-w-0">
        <div class="flex flex-1 min-h-0">
          <div class="editor-wrapper flex-1 min-w-0 overflow-hidden" style="background-color: var(--editor-bg);">
            <EditorArea
              content={activeContent}
              language={activeLanguage}
              wordWrap={settings.wordWrap}
              onContentChange={handleContentChange}
              theme={settings.theme}
              onViewReady={handleEditorViewCreated}
              onScrollChange={(pct) => { editorScrollPercent = pct; }}
            />
          </div>
          {#if settings.showPreview}
            <div class="divider-vertical" style="background-color: var(--divider-color);"></div>
            <div class="w-1/2 min-w-0 overflow-hidden" style="background-color: var(--preview-bg);">
              <PreviewPanel content={activeContent} scrollPercent={editorScrollPercent} />
            </div>
          {/if}
        </div>

        <FindReplace
          visible={settings.showFindReplace}
          editorView={editorView}
          onClose={toggleFindReplace}
        />
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center" style="color: var(--muted);">
        <div class="text-center">
          <p class="text-lg mb-2">Welcome to NoteForge</p>
          <p class="text-sm">Open a file or press Ctrl+N to create a new one.</p>
        </div>
      </div>
    {/if}
  </div>

  <StatusBar
    wordCount={wordCount}
    charCount={charCount}
    encoding={activeEncoding}
    lineEnding={activeLineEnding}
  />

  <SettingsPanel
    visible={showSettings}
    onClose={() => { showSettings = false; }}
  />

  <FileChangedNotification
    notification={fileChangedNotif}
    onReload={handleReloadFile}
    onIgnore={handleIgnoreFileChange}
  />

  <GoToLine
    visible={showGoToLine}
    editorView={editorView}
    onClose={toggleGoToLine}
  />
</div>
