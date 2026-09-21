<script lang="ts">
  let {
    visible,
    editorView,
    onClose,
  }: {
    visible: boolean;
    editorView: import('@codemirror/view').EditorView | null;
    onClose: () => void;
  } = $props();

  let lineNumber = $state('');
  let inputEl: HTMLInputElement | undefined;

  $effect(() => {
    if (visible) {
      lineNumber = '';
      // Focus input after DOM update
      setTimeout(() => inputEl?.focus(), 0);
    }
  });

  function handleGo() {
    const line = parseInt(lineNumber, 10);
    if (!isNaN(line) && line > 0 && editorView) {
      const pos = editorView.state.doc.line(Math.min(line, editorView.state.doc.lines)).from;
      editorView.dispatch({
        selection: { anchor: pos },
        scrollIntoView: true,
      });
      editorView.focus();
    }
    onClose();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleGo();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    }
  }
</script>

{#if visible}
  <div class="goto-overlay fixed z-50 flex items-center gap-2 px-3 py-2 rounded shadow-lg"
       style="top: 80px; right: 20px; background-color: var(--bg); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
    <label for="goto-line-input" class="text-xs" style="color: var(--muted);">Go to line:</label>
    <input
      id="goto-line-input"
      type="number"
      min="1"
      bind:this={inputEl}
      bind:value={lineNumber}
      onkeydown={handleKeydown}
      class="w-20 px-2 py-1 text-xs rounded border outline-none"
      style="background-color: var(--secondary); color: var(--text); border-color: var(--border);"
      placeholder="1"
    />
    <button
      class="px-2 py-1 text-xs rounded"
      style="background-color: var(--accent); color: white;"
      onclick={handleGo}
    >
      Go
    </button>
  </div>
{/if}
