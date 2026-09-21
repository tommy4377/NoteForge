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
    onScrollChange,
  }: {
    content: string;
    language: string;
    wordWrap: boolean;
    onContentChange: (content: string) => void;
    theme: string;
    onViewReady?: (view: EditorView) => void;
    onScrollChange?: (scrollPercent: number) => void;
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

  // Create editor with explicit initial doc so the recreate-effect
  // does NOT need to read the reactive `content` prop.
  function createEditor(initialDoc: string, lang: string, wrap: boolean, themeName: string) {
    if (!editorContainer) return;

    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged && !isUpdatingFromProp) {
        onContentChange(update.state.doc.toString());
      }
    });

    const themeExt = themeName === 'dark' ? [oneDark] : [];

    const wrapExt = wrap
      ? [EditorView.lineWrapping]
      : [];

    const state = EditorState.create({
      doc: initialDoc,
      extensions: [
        // No line numbers gutter
        history(),
        highlightSelectionMatches(),
        bracketMatching(),
        closeBrackets(),
        indentOnInput(),
        syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
        drawSelection(),
        getLanguageExtension(lang),
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

    // Attach native scroll listener for synchronized scroll
    if (onScrollChange) {
      const scroller = view.scrollDOM;
      scroller.addEventListener('scroll', () => {
        const maxScroll = scroller.scrollHeight - scroller.clientHeight;
        const percent = maxScroll > 0 ? scroller.scrollTop / maxScroll : 0;
        onScrollChange(percent);
      });
    }

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

  // Tracks whether the editor has been created at least once
  // (used to decide whether the recreate-effect should fire)
  // IMPORTANT: This must NOT be $state — otherwise the first $effect re-runs
  // when onMount sets it to true, destroying and recreating the editor
  // and causing focus loss on every keystroke.
  let editorCreated = false;

  // Non-reactive snapshot of the current content, used by the recreate-effect
  // so it doesn't need to read the reactive `content` prop (which would cause
  // the effect to re-fire on every keystroke).
  let contentSnapshot: string = '';

  onMount(() => {
    contentSnapshot = content;
    createEditor(contentSnapshot, language, wordWrap, theme);
    editorCreated = true;
  });

  onDestroy(() => {
    destroyEditor();
  });

  // Effect 1: recreate editor when language, wordWrap, or theme change.
  // IMPORTANT: This effect must NOT read `content`, otherwise it fires
  // on every keystroke, destroying the editor mid-type. Instead, it uses
  // `contentSnapshot` which is updated by Effect 2 but is not reactive.
  $effect(() => {
    const _lang = language;
    const _wrap = wordWrap;
    const _theme = theme;

    // Only recreate after initial mount
    if (editorCreated) {
      // Capture current doc from the view before destroying
      if (view) {
        contentSnapshot = view.state.doc.toString();
      }
      destroyEditor();
      createEditor(contentSnapshot, _lang, _wrap, _theme);
    }
  });

  // Effect 2: update editor content when the `content` prop changes (tab switch, external reload).
  // This does NOT recreate the editor — just dispatches a text change.
  // It also keeps `contentSnapshot` in sync so Effect 1 can use it.
  $effect(() => {
    const newContent = content;
    contentSnapshot = newContent;
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
