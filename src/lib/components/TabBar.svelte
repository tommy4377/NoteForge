<script lang="ts">
  import type { TabState } from '$lib/stores/app';

  let {
    tabs,
    activeTabId,
    onSwitchTab,
    onCloseTab,
  }: {
    tabs: TabState[];
    activeTabId: string | null;
    onSwitchTab: (id: string) => void;
    onCloseTab: (id: string) => void;
  } = $props();

  function handleMiddleClick(e: MouseEvent, id: string) {
    if (e.button === 1) {
      e.preventDefault();
      onCloseTab(id);
    }
  }
</script>

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
</div>
