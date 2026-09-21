<script lang="ts">
  import type { TabState } from '$lib/stores/app';
  import { invoke } from '@tauri-apps/api/core';

  let {
    tabs,
    activeTabId,
    onSwitchTab,
    onCloseTab,
    onNewTab,
  }: {
    tabs: TabState[];
    activeTabId: string | null;
    onSwitchTab: (id: string) => void;
    onCloseTab: (id: string) => void;
    onNewTab: () => void;
  } = $props();

  // Context menu state
  let contextMenu = $state<{ x: number; y: number; tab: TabState } | null>(null);

  function handleMiddleClick(e: MouseEvent, id: string) {
    if (e.button === 1) {
      e.preventDefault();
      onCloseTab(id);
    }
  }

  function handleContextMenu(e: MouseEvent, tab: TabState) {
    e.preventDefault();
    contextMenu = { x: e.clientX, y: e.clientY, tab };
  }

  function closeContextMenu() {
    contextMenu = null;
  }

  async function copyPath() {
    if (contextMenu?.tab.filePath) {
      try {
        await navigator.clipboard.writeText(contextMenu.tab.filePath);
      } catch {
        // clipboard may not be available
      }
    }
    closeContextMenu();
  }

  async function revealInExplorer() {
    if (contextMenu?.tab.filePath) {
      try {
        await invoke('reveal_in_explorer', { path: contextMenu.tab.filePath });
      } catch {
        // command may not be available on all platforms
      }
    }
    closeContextMenu();
  }

  function closeFromMenu() {
    if (contextMenu) {
      onCloseTab(contextMenu.tab.id);
    }
    closeContextMenu();
  }
</script>

<svelte:window onclick={closeContextMenu} oncontextmenu={closeContextMenu} />

<div class="tab-bar flex items-center h-9 overflow-x-auto shrink-0 select-none"
     style="background-color: var(--tab-inactive-bg); border-bottom: 1px solid var(--border);">
  {#if tabs.length === 0}
    <div class="px-4 text-xs" style="color: var(--muted);">No files open</div>
  {/if}
  {#each tabs as tab (tab.id)}
    <button
      class="tab flex items-center gap-1 px-3 h-full text-xs whitespace-nowrap border-r max-w-48 min-w-0 shrink-0"
      style="
        background-color: {tab.id === activeTabId ? 'var(--tab-active-bg)' : 'var(--tab-inactive-bg)'};
        border-color: var(--border);
        color: var(--text);
        {tab.id === activeTabId ? 'border-bottom: 2px solid var(--accent);' : ''}
      "
      onclick={() => onSwitchTab(tab.id)}
      onmousedown={(e) => handleMiddleClick(e, tab.id)}
      oncontextmenu={(e) => handleContextMenu(e, tab)}
    >
      <span class="truncate">{tab.fileName}{tab.isDirty ? ' *' : ''}</span>
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span
        class="close-btn ml-1 w-4 h-4 flex items-center justify-center rounded hover:bg-[var(--active-bg)] text-xs leading-none"
        role="button"
        tabindex="-1"
        onclick={(e) => { e.stopPropagation(); onCloseTab(tab.id); }}
        onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); onCloseTab(tab.id); } }}
      >
        ×
      </span>
    </button>
  {/each}
  <!-- New tab (+) button -->
  <button
    class="flex items-center justify-center w-9 h-full shrink-0 text-sm font-bold hover:bg-[var(--hover-bg)]"
    style="color: var(--muted);"
    onclick={onNewTab}
    title="New tab (Ctrl+N)"
    aria-label="New tab"
  >
    +
  </button>
</div>

{#if contextMenu}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed z-50 min-w-48 rounded shadow-lg py-1"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px; background-color: var(--bg); border: 1px solid var(--border); box-shadow: var(--shadow-lg);"
    onclick={(e) => e.stopPropagation()}
    oncontextmenu={(e) => e.preventDefault()}
  >
    {#if contextMenu.tab.filePath}
      <button class="w-full text-left px-4 py-1.5 text-xs hover:bg-[var(--hover-bg)]" style="color: var(--text);"
              onclick={copyPath}>
        Copy path
      </button>
      <button class="w-full text-left px-4 py-1.5 text-xs hover:bg-[var(--hover-bg)]" style="color: var(--text);"
              onclick={revealInExplorer}>
        Reveal in Explorer
      </button>
      <div class="my-1 border-t" style="border-color: var(--border);"></div>
    {/if}
    <button class="w-full text-left px-4 py-1.5 text-xs hover:bg-[var(--hover-bg)]" style="color: var(--text);"
            onclick={closeFromMenu}>
      Close
    </button>
  </div>
{/if}
