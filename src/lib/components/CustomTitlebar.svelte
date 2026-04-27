<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

  let activeTabId = $state<string | null>(null);
  let tabs: import('$lib/stores/app').TabState[] = $state([]);

  // Subscribe to stores reactively  
  import { appStore } from '$lib/stores/app';

  $effect(() => {
    const unsub1 = appStore.activeTabId.subscribe((id) => { activeTabId = id; });
    const unsub2 = appStore.tabs.subscribe((t) => { tabs = t; });
    return () => { unsub1(); unsub2(); };
  });

  let title = $derived.by(() => {
    const tab = tabs.find((t) => t.id === activeTabId);
    return tab ? `NoteForge — ${tab.fileName}` : 'NoteForge';
  });

  async function handleClose() {
    await appWindow.close();
  }

  async function handleMinimize() {
    await appWindow.minimize();
  }

  async function handleToggleMaximize() {
    await appWindow.toggleMaximize();
  }
</script>

<div
  class="titlebar"
  data-tauri-drag-region
>
  <!-- Left: App icon + Title -->
  <div class="titlebar-left" data-tauri-drag-region>
    <img
      src="/icons/app-icon.png"
      alt="NoteForge"
      class="app-icon"
      data-tauri-drag-region
    />
    <span class="titlebar-title" data-tauri-drag-region>{title}</span>
  </div>

  <!-- Right: Traffic light buttons -->
  <div class="titlebar-right">
    <button
      class="titlebar-btn minimize-btn"
      onclick={handleMinimize}
      aria-label="Minimize"
      title="Minimize"
    >
      <span class="btn-icon">−</span>
    </button>
    <button
      class="titlebar-btn maximize-btn"
      onclick={handleToggleMaximize}
      aria-label="Maximize"
      title="Maximize"
    >
      <span class="btn-icon">⤢</span>
    </button>
    <button
      class="titlebar-btn close-btn"
      onclick={handleClose}
      aria-label="Close"
      title="Close"
    >
      <span class="btn-icon">×</span>
    </button>
  </div>
</div>

<style>
  .titlebar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    height: 32px;
    padding: 0 8px 0 10px;
    background-color: var(--secondary);
    user-select: none;
    flex-shrink: 0;
    -webkit-app-region: drag;
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 0;
    overflow: hidden;
  }

  .app-icon {
    width: 16px;
    height: 16px;
    object-fit: cover;
    flex-shrink: 0;
    border-radius: 2px;
  }

  .titlebar-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .titlebar-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
    -webkit-app-region: no-drag;
  }

  .titlebar-btn {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: none;
    cursor: pointer;
    padding: 0;
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 1;
    -webkit-app-region: no-drag;
  }

  .minimize-btn {
    background-color: #FEBC2E;
  }

  .maximize-btn {
    background-color: #28C840;
  }

  .close-btn {
    background-color: #FF5F57;
  }

  .btn-icon {
    display: none;
    font-size: 8px;
    font-weight: 700;
    color: rgba(0, 0, 0, 0.6);
    line-height: 1;
    font-family: sans-serif;
  }

  .titlebar-btn:hover .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>