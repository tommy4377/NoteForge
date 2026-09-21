<script lang="ts">
  let {
    visible,
    onClose,
  }: {
    visible: boolean;
    onClose: () => void;
  } = $props();

  const shortcuts = [
    { key: 'Ctrl+N',          action: 'New File' },
    { key: 'Ctrl+O',          action: 'Open File' },
    { key: 'Ctrl+S',          action: 'Save' },
    { key: 'Ctrl+Shift+S',    action: 'Save As' },
    { key: 'Ctrl+Shift+M',    action: 'Toggle Preview' },
    { key: 'Ctrl+H',          action: 'Find & Replace' },
    { key: 'Alt+Z',           action: 'Toggle Word Wrap' },
    { key: 'Ctrl+Shift+T',    action: 'Cycle Theme' },
    { key: 'Ctrl+W',          action: 'Close Tab' },
    { key: 'Ctrl+T',          action: 'New Tab' },
    { key: 'Ctrl+G',          action: 'Go to Line' },
    { key: 'Ctrl+A',          action: 'Select All' },
    { key: 'Ctrl++',          action: 'Increase Font Size' },
    { key: 'Ctrl+-',          action: 'Decrease Font Size' },
    { key: 'Ctrl+0',          action: 'Reset Zoom & Font' },
    { key: 'Ctrl+Scroll',     action: 'Zoom Workspace (50%-200%)' },
  ];
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="help-overlay fixed inset-0 z-50 flex items-center justify-center"
       style="background-color: rgba(0,0,0,0.4);"
       onclick={(e) => { if ((e.target as HTMLElement).classList.contains('help-overlay')) onClose(); }}
       onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
       role="dialog"
       aria-modal="true"
       tabindex="-1">
    <div class="help-panel w-[480px] max-h-[80vh] overflow-y-auto rounded-lg p-6"
         style="background-color: var(--bg); color: var(--text); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
      <div class="flex items-center justify-between mb-5">
        <h2 class="text-lg font-semibold">Keyboard Shortcuts</h2>
        <button class="w-7 h-7 flex items-center justify-center rounded hover:bg-[var(--hover-bg)]"
                onclick={onClose}>✕</button>
      </div>

      <div class="shortcuts-grid">
        {#each shortcuts as s}
          <div class="shortcut-row">
            <span class="shortcut-action">{s.action}</span>
            <kbd class="shortcut-key">{s.key}</kbd>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .shortcuts-grid {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .shortcut-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 5px 8px;
    border-radius: 4px;
  }

  .shortcut-row:hover {
    background-color: var(--hover-bg);
  }

  .shortcut-action {
    font-size: 13px;
    color: var(--text);
  }

  .shortcut-key {
    font-family: "JetBrains Mono", "Cascadia Code", "Fira Code", "Consolas", monospace;
    font-size: 11px;
    background-color: var(--secondary);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 8px;
    min-width: 80px;
    text-align: center;
  }
</style>
