<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';

  const appWindow = getCurrentWindow();

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
  class="titlebar flex items-center justify-between h-8 shrink-0 select-none"
  style="background-color: var(--secondary);"
  data-tauri-drag-region
>
  <!-- Left: App title (also draggable) -->
  <div
    class="flex items-center pl-3 text-xs font-medium"
    style="color: var(--muted);"
    data-tauri-drag-region
  >
    NoteForge
  </div>

  <!-- Right: Traffic light buttons -->
  <div class="flex items-center gap-2 pr-3" data-tauri-drag-region>
    <button
      class="titlebar-btn flex items-center justify-center rounded-full"
      style="width: 12px; height: 12px; background-color: #FEBC2E;"
      onclick={handleMinimize}
      aria-label="Minimize"
    >
      <span class="btn-icon">&minus;</span>
    </button>
    <button
      class="titlebar-btn flex items-center justify-center rounded-full"
      style="width: 12px; height: 12px; background-color: #28C840;"
      onclick={handleToggleMaximize}
      aria-label="Maximize"
    >
      <span class="btn-icon">&#10548;</span>
    </button>
    <button
      class="titlebar-btn flex items-center justify-center rounded-full"
      style="width: 12px; height: 12px; background-color: #FF5F57;"
      onclick={handleClose}
      aria-label="Close"
    >
      <span class="btn-icon">&times;</span>
    </button>
  </div>
</div>

<style>
  .titlebar-btn {
    border: none;
    cursor: pointer;
    padding: 0;
    position: relative;
    line-height: 1;
  }

  .btn-icon {
    display: none;
    font-size: 9px;
    font-weight: 700;
    color: rgba(0, 0, 0, 0.5);
    line-height: 1;
  }

  .titlebar-btn:hover .btn-icon {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Prevent drag when clicking buttons */
  .titlebar-btn {
    -webkit-app-region: no-drag;
  }
</style>
