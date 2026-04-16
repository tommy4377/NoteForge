<script lang="ts">
  import { EditorView, keymap, drawSelection } from '@codemirror/view';
  import { EditorState } from '@codemirror/state';
  import { markdown } from '@codemirror/lang-markdown';
  import { javascript } from '@codemirror/lang-javascript';
  import { json } from '@codemirror/lang-json';
  import { python } from '@codemirror/lang-python';
  import { html } from '@codemirror/lang-html';
  import { css } from '@codemirror/lang-css';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
  import { highlightSelectionMatches } from '@codemirror/search';
  import { bracketMatching, indentOnInput, syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
  import { closeBrackets, closeBracketsKeymap } from '@codemirror/autocomplete';
  import { onMount, onDestroy } from 'svelte';

  let {
    content,
    language,
    wordWrap,
    onContentChange,
    theme,
    onViewReady,
  }: {
    content: string;
    language: string;
    wordWrap: boolean;
    onContentChange: (content: string) => void;
    theme: string;
    onViewReady?: (view: EditorView) => void;
  } = $props();

  let editorContainer: HTMLDivElement;
  let view: EditorView | null = null;
  let isUpdatingFromProp = false;

  function getLanguageExtension(lang: string) {
    switch (lang) {
      case 'markdown': return markdown();
      case 'javascript': return javascript();
      case 'json': return json();
      case 'python': return python();
      case 'html': return html();
      case 'css': return css();
      default: return markdown();
    }
  }

  function createEditor() {
    if (!editorContainer) return;

    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged && !isUpdatingFromProp) {
        onContentChange(update.state.doc.toString());
      }
    });

    const themeExt = theme === 'dark' ? [oneDark] : [];

    const wrapExt = wordWrap
      ? [EditorView.lineWrapping]
      : [];

    const state = EditorState.create({
      doc: content,
      extensions: [
        // No line numbers gutter
        history(),
        highlightSelectionMatches(),
        bracketMatching(),
        closeBrackets(),
        indentOnInput(),
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
        drawSelection(),
        getLanguageExtension(language),
        keymap.of([
          ...closeBracketsKeymap,
          ...defaultKeymap,
          ...historyKeymap,
          indentWithTab,
        ]),
        ...wrapExt,
        ...themeExt,
        updateListener,
        EditorView.theme({
          '&': {
            backgroundColor: 'var(--bg)',
            color: 'var(--text)',
          },
          '.cm-content': {
            caretColor: 'var(--text)',
            padding: '8px 0',
          },
          '.cm-cursor, .cm-dropCursor': {
            borderLeftColor: 'var(--text)',
          },
          '.cm-activeLine': {
            backgroundColor: 'var(--hover-bg)',
          },
          '.cm-selectionBackground, .cm-focused .cm-selectionBackground': {
            backgroundColor: 'var(--selection-bg) !important',
          },
          '&.cm-focused .cm-selectionBackground': {
            backgroundColor: 'var(--selection-bg) !important',
          },
          '.cm-gutters': {
            display: 'none',
          },
        }),
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });

    if (onViewReady && view) {
      onViewReady(view);
    }
  }

  function destroyEditor() {
    if (view) {
      view.destroy();
      view = null;
    }
  }

  onMount(() => {
    createEditor();
  });

  onDestroy(() => {
    destroyEditor();
  });

  // Reconfigure when language, wordWrap, or theme changes
  $effect(() => {
    const _lang = language;
    const _wrap = wordWrap;
    const _theme = theme;

    if (view) {
      destroyEditor();
      createEditor();
    }
  });

  // Update editor content when prop changes (tab switch)
  $effect(() => {
    const newContent = content;
    if (view && view.state.doc.toString() !== newContent) {
      isUpdatingFromProp = true;
      view.dispatch({
        changes: {
          from: 0,
          to: view.state.doc.length,
          insert: newContent,
        },
      });
      isUpdatingFromProp = false;
    }
  });
</script>

<div class="editor-area flex-1 min-h-0 overflow-hidden" bind:this={editorContainer}></div>
