<script lang="ts">
  let {
    notification,
    onReload,
    onIgnore,
  }: {
    notification: { path: string; visible: boolean } | null;
    onReload: (path: string) => void;
    onIgnore: () => void;
  } = $props();
</script>

{#if notification && notification.visible}
  <div class="fixed bottom-10 right-4 z-50 max-w-sm rounded-lg p-4 shadow-lg"
       style="background-color: var(--bg); border: 1px solid var(--border); color: var(--text); box-shadow: var(--shadow-lg);">
    <div class="text-sm font-medium mb-2">File changed externally</div>
    <div class="text-xs mb-3 truncate" style="color: var(--muted);" title={notification.path}>
      {notification.path}
    </div>
    <div class="text-sm mb-3">Do you want to reload it?</div>
    <div class="flex gap-2 justify-end">
      <button class="px-3 py-1 text-xs rounded"
              style="background-color: var(--accent); color: var(--bg);"
              onclick={() => onReload(notification.path)}>
        Reload
      </button>
      <button class="px-3 py-1 text-xs rounded hover:bg-[var(--hover-bg)]"
              style="border: 1px solid var(--border);"
              onclick={onIgnore}>
        Ignore
      </button>
    </div>
  </div>
{/if}
