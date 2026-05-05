<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  let {
    x,
    y,
    visible,
    onClose,
    onTogglePreview,
    activeFilePath,
  }: {
    x: number;
    y: number;
    visible: boolean;
    onClose: () => void;
    onTogglePreview: () => void;
    activeFilePath: string;
  } = $props();

  async function handleCut() {
    try {
      await document.execCommand('cut');
    } catch {
      // fallback: no clipboard access
    }
    onClose();
  }

  async function handleCopy() {
    try {
      await document.execCommand('copy');
    } catch {
      // fallback
    }
    onClose();
  }

  async function handlePaste() {
    try {
      const text = await navigator.clipboard.readText();
      document.execCommand('insertText', false, text);
    } catch {
      // fallback
    }
    onClose();
  }

  function handleSelectAll() {
    const sel = window.getSelection();
    if (sel) {
      sel.selectAllChildren(document.body);
    }
    onClose();
  }

  function handleTogglePreview() {
    onTogglePreview();
    onClose();
  }

  async function handleRevealInExplorer() {
    if (activeFilePath) {
      try {
        await invoke('reveal_in_explorer', { path: activeFilePath });
      } catch (e) {
        console.error('Reveal in Explorer failed:', e);
      }
    }
    onClose();
  }

  // Dismiss on Escape
  $effect(() => {
    if (!visible) return;
    function onKey(e: KeyboardEvent) {
      if (e.key === 'Escape') {
        onClose();
      }
    }
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  });
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="context-menu-overlay fixed inset-0 z-40"
       onclick={onClose}
       oncontextmenu={(e) => { e.preventDefault(); onClose(); }}>
  </div>
  <div
    class="context-menu z-50"
    style="left: {x}px; top: {y}px;"
    role="menu"
  >
    <button class="context-menu-item" onclick={handleCut} role="menuitem">
      <span class="context-menu-icon">✂</span>
      Cut
      <span class="context-menu-shortcut">Ctrl+X</span>
    </button>
    <button class="context-menu-item" onclick={handleCopy} role="menuitem">
      <span class="context-menu-icon">📋</span>
      Copy
      <span class="context-menu-shortcut">Ctrl+C</span>
    </button>
    <button class="context-menu-item" onclick={handlePaste} role="menuitem">
      <span class="context-menu-icon">📄</span>
      Paste
      <span class="context-menu-shortcut">Ctrl+V</span>
    </button>
    <div class="context-menu-divider"></div>
    <button class="context-menu-item" onclick={handleSelectAll} role="menuitem">
      <span class="context-menu-icon">☑</span>
      Select All
      <span class="context-menu-shortcut">Ctrl+A</span>
    </button>
    <div class="context-menu-divider"></div>
    <button class="context-menu-item" onclick={handleTogglePreview} role="menuitem">
      <span class="context-menu-icon">👁</span>
      Toggle Preview
      <span class="context-menu-shortcut">Ctrl+Shift+M</span>
    </button>
    <button class="context-menu-item" onclick={handleRevealInExplorer} role="menuitem"
            disabled={!activeFilePath}>
      <span class="context-menu-icon">📁</span>
      Reveal in Explorer
    </button>
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    min-width: 220px;
    border-radius: 6px;
    padding: 4px 0;
    background-color: var(--bg);
    border: 1px solid var(--border);
    box-shadow: var(--shadow-lg);
    font-family: "JetBrains Mono", "Cascadia Code", "Fira Code", "Consolas", monospace;
    font-size: 12px;
    color: var(--text);
    user-select: none;
  }

  .context-menu-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 6px 12px;
    border: none;
    background: none;
    color: var(--text);
    font-family: inherit;
    font-size: inherit;
    cursor: pointer;
    text-align: left;
    gap: 8px;
  }

  .context-menu-item:hover {
    background-color: var(--hover-bg);
  }

  .context-menu-item:disabled {
    opacity: 0.4;
    cursor: default;
  }

  .context-menu-item:disabled:hover {
    background-color: transparent;
  }

  .context-menu-icon {
    width: 16px;
    text-align: center;
    flex-shrink: 0;
  }

  .context-menu-shortcut {
    margin-left: auto;
    opacity: 0.5;
    font-size: 11px;
  }

  .context-menu-divider {
    height: 1px;
    margin: 4px 0;
    background-color: var(--border);
  }
</style>
