<script lang="ts">
  let {
    wordCount,
    charCount,
    encoding,
    lineEnding,
  }: {
    wordCount: number;
    charCount: number;
    encoding: string;
    lineEnding: string;
  } = $props();
</script>

<div class="status-bar flex items-center h-6 px-3 text-xs shrink-0 select-none gap-4"
     style="background-color: var(--statusbar-bg); border-top: 1px solid var(--border); color: var(--muted);">
  <span>{wordCount} word{wordCount !== 1 ? 's' : ''}</span>
  <span>{charCount} char{charCount !== 1 ? 's' : ''}</span>
  <span class="ml-auto">{encoding}</span>
  <span>{lineEnding}</span>
</div>
