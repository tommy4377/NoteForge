<script lang="ts">
  import { marked } from 'marked';
  import katex from 'katex';
  import hljs from 'highlight.js';
  import 'katex/dist/katex.min.css';
  import 'highlight.js/styles/github.css';

  let {
    content,
    scrollPercent = 0,
  }: {
    content: string;
    scrollPercent?: number;
  } = $props();

  let renderedHtml = $state('');
  let panelEl: HTMLDivElement | undefined;
  let isSyncingScroll = false;

  // Configure marked with highlight.js for code blocks
  const renderer = new marked.Renderer();

  renderer.code = function ({ text, lang }: { text: string; lang?: string }) {
    const language = lang && hljs.getLanguage(lang) ? lang : 'plaintext';
    const highlighted = hljs.highlight(text, { language }).value;
    return `<pre><code class="hljs language-${language}">${highlighted}</code></pre>`;
  };

  marked.setOptions({ renderer });

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

  // Sync scroll from editor
  $effect(() => {
    const pct = scrollPercent;
    if (panelEl && !isSyncingScroll) {
      isSyncingScroll = true;
      const maxScroll = panelEl.scrollHeight - panelEl.clientHeight;
      panelEl.scrollTop = pct * maxScroll;
      requestAnimationFrame(() => { isSyncingScroll = false; });
    }
  });
</script>

<div class="preview-panel overflow-y-auto h-full"
      bind:this={panelEl}
      style="background-color: var(--preview-bg); color: var(--text);">
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

  /* Fix pre/code blocks for dark/sepia themes */
  :global(.preview-panel pre),
  :global(.preview-panel code) {
    background-color: var(--secondary) !important;
    color: var(--text) !important;
  }

  :global(.preview-panel pre) {
    padding: 1em;
    border-radius: 5px;
    overflow-x: auto;
    border: 1px solid var(--border);
  }

  :global(.preview-panel pre code) {
    background: none !important;
    padding: 0 !important;
    border: none !important;
  }

  /* Override highlight.js inline code colors to respect theme */
  :global(.preview-panel code.hljs) {
    background-color: var(--secondary) !important;
  }

  /* Inline code (not in pre) */
  :global(.preview-panel p code),
  :global(.preview-panel li code),
  :global(.preview-panel td code) {
    background-color: var(--secondary) !important;
    color: var(--text) !important;
    padding: 0.15em 0.4em;
    border-radius: 3px;
    font-size: 0.9em;
  }
</style>
