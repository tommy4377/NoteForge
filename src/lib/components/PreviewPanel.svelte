<script lang="ts">
  import { marked } from 'marked';
  import katex from 'katex';
  import 'katex/dist/katex.min.css';

  let {
    content,
  }: {
    content: string;
  } = $props();

  let renderedHtml = $state('');

  function renderLatex(text: string): string {
    // Block LaTeX: $$...$$
    text = text.replace(/\$\$([\s\S]*?)\$\$/g, (_match, formula) => {
      try {
        return katex.renderToString(formula.trim(), { displayMode: true, throwOnError: false });
      } catch {
        return `<span style="color:red;">LaTeX Error: ${formula}</span>`;
      }
    });

    // Inline LaTeX: $...$  (but not $$)
    text = text.replace(/(?<!\$)\$(?!\$)(.+?)(?<!\$)\$(?!\$)/g, (_match, formula) => {
      try {
        return katex.renderToString(formula.trim(), { displayMode: false, throwOnError: false });
      } catch {
        return `<span style="color:red;">LaTeX Error: ${formula}</span>`;
      }
    });

    return text;
  }

  $effect(() => {
    const raw = content;
    const withLatex = renderLatex(raw);
    try {
      const result = marked.parse(withLatex, { async: false });
      renderedHtml = typeof result === 'string' ? result : '';
    } catch {
      renderedHtml = '<p>Preview error</p>';
    }
  });
</script>

<div class="preview-panel border-l overflow-y-auto h-full"
     style="background-color: var(--bg); color: var(--text); border-color: var(--border); width: 100%;">
  {#if content.trim() === ''}
    <div class="p-8 text-center" style="color: var(--muted);">
      <p>Preview will appear here.</p>
      <p class="text-sm mt-2">Start typing Markdown or LaTeX in the editor.</p>
    </div>
  {:else}
    {@html renderedHtml}
  {/if}
</div>

<style>
  /* KaTeX overrides for theme */
  :global(.katex) {
    color: var(--text) !important;
  }
  :global(.katex-display) {
    margin: 1em 0 !important;
    overflow-x: auto;
  }
</style>
