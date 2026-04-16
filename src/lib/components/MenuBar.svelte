<script lang="ts">
  let {
    onNew,
    onOpen,
    onSave,
    onSaveAs,
    onExportPdf,
    onTogglePreview,
    onToggleFindReplace,
    onToggleWordWrap,
    onToggleTheme,
    onOpenSettings,
    recentFiles,
    onOpenRecent,
    showPreview,
    showFindReplace,
    wordWrap,
    currentTheme,
  }: {
    onNew: () => void;
    onOpen: () => void;
    onSave: () => void;
    onSaveAs: () => void;
    onExportPdf: () => void;
    onTogglePreview: () => void;
    onToggleFindReplace: () => void;
    onToggleWordWrap: () => void;
    onToggleTheme: () => void;
    onOpenSettings: () => void;
    recentFiles: string[];
    onOpenRecent: (path: string) => void;
    showPreview: boolean;
    showFindReplace: boolean;
    wordWrap: boolean;
    currentTheme: string;
  } = $props();

  let activeMenu = $state<string | null>(null);

  function toggleMenu(menu: string) {
    activeMenu = activeMenu === menu ? null : menu;
  }

  function closeMenu() {
    activeMenu = null;
  }

  function handleMenuClick(fn: () => void) {
    fn();
    closeMenu();
  }

  function handleRecentClick(path: string) {
    onOpenRecent(path);
    closeMenu();
  }
</script>

<svelte:window onclick={(e) => { if (!(e.target as HTMLElement).closest('.menu-bar')) closeMenu(); }} />

<nav class="menu-bar flex items-center h-8 px-1 text-sm select-none shrink-0"
     style="background-color: var(--secondary); border-bottom: 1px solid var(--border); color: var(--text);">

  <!-- File Menu -->
  <div class="relative">
    <button class="px-3 py-1 rounded hover:bg-[var(--hover-bg)] {activeMenu === 'file' ? 'bg-[var(--active-bg)]' : ''}"
            onclick={() => toggleMenu('file')}>
      File
    </button>
    {#if activeMenu === 'file'}
      <div class="absolute left-0 top-full mt-0.5 min-w-52 rounded shadow-lg z-50 py-1"
           style="background-color: var(--bg); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]" onclick={() => handleMenuClick(onNew)}>
          New <span class="float-right text-xs opacity-60">Ctrl+N</span>
        </button>
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]" onclick={() => handleMenuClick(onOpen)}>
          Open… <span class="float-right text-xs opacity-60">Ctrl+O</span>
        </button>
        <div class="my-1 border-t" style="border-color: var(--border);"></div>
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]" onclick={() => handleMenuClick(onSave)}>
          Save <span class="float-right text-xs opacity-60">Ctrl+S</span>
        </button>
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]" onclick={() => handleMenuClick(onSaveAs)}>
          Save As… <span class="float-right text-xs opacity-60">Ctrl+Shift+S</span>
        </button>
        <div class="my-1 border-t" style="border-color: var(--border);"></div>
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]" onclick={() => handleMenuClick(onExportPdf)}>
          Export PDF…
        </button>
        <div class="my-1 border-t" style="border-color: var(--border);"></div>
        <!-- Recent Files Submenu -->
        <div class="relative group">
          <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)] flex items-center justify-between">
            Recent Files
            <span class="text-xs opacity-60">▶</span>
          </button>
          {#if recentFiles.length > 0}
            <div class="absolute left-full top-0 min-w-64 rounded shadow-lg z-50 py-1 hidden group-hover:block"
                 style="background-color: var(--bg); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
              {#each recentFiles as path}
                <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)] truncate max-w-64 text-xs"
                        onclick={() => handleRecentClick(path)} title={path}>
                  {path}
                </button>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </div>

  <!-- Edit Menu -->
  <div class="relative">
    <button class="px-3 py-1 rounded hover:bg-[var(--hover-bg)] {activeMenu === 'edit' ? 'bg-[var(--active-bg)]' : ''}"
            onclick={() => toggleMenu('edit')}>
      Edit
    </button>
    {#if activeMenu === 'edit'}
      <div class="absolute left-0 top-full mt-0.5 min-w-52 rounded shadow-lg z-50 py-1"
           style="background-color: var(--bg); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]" onclick={() => handleMenuClick(onToggleFindReplace)}>
          Find & Replace <span class="float-right text-xs opacity-60">Ctrl+H</span>
        </button>
        <div class="my-1 border-t" style="border-color: var(--border);"></div>
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)] flex items-center gap-2"
                onclick={() => handleMenuClick(onToggleWordWrap)}>
          <span class="w-4 inline-block">{wordWrap ? '✓' : ''}</span>
          Word Wrap <span class="float-right text-xs opacity-60">Alt+Z</span>
        </button>
      </div>
    {/if}
  </div>

  <!-- View Menu -->
  <div class="relative">
    <button class="px-3 py-1 rounded hover:bg-[var(--hover-bg)] {activeMenu === 'view' ? 'bg-[var(--active-bg)]' : ''}"
            onclick={() => toggleMenu('view')}>
      View
    </button>
    {#if activeMenu === 'view'}
      <div class="absolute left-0 top-full mt-0.5 min-w-52 rounded shadow-lg z-50 py-1"
           style="background-color: var(--bg); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)] flex items-center gap-2"
                onclick={() => handleMenuClick(onTogglePreview)}>
          <span class="w-4 inline-block">{showPreview ? '✓' : ''}</span>
          Preview <span class="float-right text-xs opacity-60">Ctrl+Shift+M</span>
        </button>
        <button class="w-full text-left px-4 py-1.5 hover:bg-[var(--hover-bg)]"
                onclick={() => handleMenuClick(onToggleTheme)}>
          Toggle Theme <span class="float-right text-xs opacity-60">Ctrl+Shift+T</span>
        </button>
      </div>
    {/if}
  </div>

  <!-- Settings -->
  <div class="relative">
    <button class="px-3 py-1 rounded hover:bg-[var(--hover-bg)]"
            onclick={() => { onOpenSettings(); closeMenu(); }}>
      Settings
    </button>
  </div>
</nav>
