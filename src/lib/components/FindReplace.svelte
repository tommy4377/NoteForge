<script lang="ts">
  import { EditorView } from '@codemirror/view';

  let {
    visible,
    editorView,
    onClose,
  }: {
    visible: boolean;
    editorView: EditorView | null;
    onClose: () => void;
  } = $props();

  let searchText = $state('');
  let replaceText = $state('');
  let useRegex = $state(false);
  let caseSensitive = $state(false);
  let matchCount = $state(0);

  function getSearchQuery(): { test: (text: string) => boolean; exec: (text: string) => RegExpExecArray | null } {
    if (!searchText) return { test: () => false, exec: () => null };
    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const pattern = useRegex ? searchText : searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const re = new RegExp(pattern, flags);
      return { test: (t) => re.test(t), exec: (t) => { re.lastIndex = 0; return re.exec(t); } };
    } catch {
      return { test: () => false, exec: () => null };
    }
  }

  function countMatches() {
    if (!editorView || !searchText) {
      matchCount = 0;
      return;
    }
    const doc = editorView.state.doc.toString();
    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const pattern = useRegex ? searchText : searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const re = new RegExp(pattern, flags);
      const matches = doc.match(re);
      matchCount = matches ? matches.length : 0;
    } catch {
      matchCount = 0;
    }
  }

  function findNext() {
    if (!editorView || !searchText) return;
    const doc = editorView.state.doc.toString();
    const from = editorView.state.selection.main.head;
    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const pattern = useRegex ? searchText : searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const re = new RegExp(pattern, flags);
      re.lastIndex = from;
      let match = re.exec(doc);
      if (!match) {
        re.lastIndex = 0;
        match = re.exec(doc);
      }
      if (match) {
        const start = match.index;
        const end = start + match[0].length;
        editorView.dispatch({
          selection: { anchor: start, head: end },
          effects: EditorView.scrollIntoView(start, { y: 'center' }),
        });
      }
    } catch { /* regex error */ }
  }

  function findPrev() {
    if (!editorView || !searchText) return;
    const doc = editorView.state.doc.toString();
    const cursorPos = editorView.state.selection.main.from;
    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const pattern = useRegex ? searchText : searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const re = new RegExp(pattern, flags);
      let lastMatch: { index: number; length: number } | null = null;
      let m: RegExpExecArray | null;
      while ((m = re.exec(doc)) !== null) {
        if (m.index < cursorPos) {
          lastMatch = { index: m.index, length: m[0].length };
        } else {
          break;
        }
      }
      if (lastMatch) {
        editorView.dispatch({
          selection: { anchor: lastMatch.index, head: lastMatch.index + lastMatch.length },
          effects: EditorView.scrollIntoView(lastMatch.index, { y: 'center' }),
        });
      } else {
        // Wrap around to end
        const allMatches = [...doc.matchAll(new RegExp(pattern, flags))];
        if (allMatches.length > 0) {
          const last = allMatches[allMatches.length - 1];
          editorView.dispatch({
            selection: { anchor: last.index, head: last.index + last[0].length },
            effects: EditorView.scrollIntoView(last.index, { y: 'center' }),
          });
        }
      }
    } catch { /* regex error */ }
  }

  function replaceOne() {
    if (!editorView || !searchText) return;
    const sel = editorView.state.selection.main;
    const selectedText = editorView.state.sliceDoc(sel.from, sel.to);
    const q = getSearchQuery();
    if (q.test(selectedText)) {
      editorView.dispatch({
        changes: { from: sel.from, to: sel.to, insert: replaceText },
      });
      countMatches();
      findNext();
    } else {
      findNext();
    }
  }

  function replaceAll() {
    if (!editorView || !searchText) return;
    const doc = editorView.state.doc.toString();
    try {
      const flags = caseSensitive ? 'g' : 'gi';
      const pattern = useRegex ? searchText : searchText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      const re = new RegExp(pattern, flags);
      const newDoc = doc.replace(re, replaceText);
      editorView.dispatch({
        changes: { from: 0, to: doc.length, insert: newDoc },
      });
      countMatches();
    } catch { /* regex error */ }
  }

  $effect(() => {
    // Re-count when search params change
    searchText;
    useRegex;
    caseSensitive;
    countMatches();
  });
</script>

{#if visible}
  <div class="find-replace-panel flex flex-col gap-2 p-3 border-t"
       style="background-color: var(--secondary); border-color: var(--border); color: var(--text);">
    <div class="flex items-center gap-2">
      <input
        type="text"
        placeholder="Search…"
        bind:value={searchText}
        onkeydown={(e) => { if (e.key === 'Enter') { if (e.shiftKey) findPrev(); else findNext(); } }}
        class="flex-1 px-2 py-1 text-sm rounded border"
        style="background-color: var(--bg); border-color: var(--border); color: var(--text); outline: none;"
      />
      <span class="text-xs whitespace-nowrap" style="color: var(--muted);">
        {matchCount} match{matchCount !== 1 ? 'es' : ''}
      </span>
      <button class="px-2 py-1 text-xs rounded hover:bg-[var(--hover-bg)]" onclick={findPrev}>Prev</button>
      <button class="px-2 py-1 text-xs rounded hover:bg-[var(--hover-bg)]" onclick={findNext}>Next</button>
      <button class="px-2 py-1 text-xs rounded hover:bg-[var(--hover-bg)]" onclick={onClose}>✕</button>
    </div>
    <div class="flex items-center gap-2">
      <input
        type="text"
        placeholder="Replace…"
        bind:value={replaceText}
        class="flex-1 px-2 py-1 text-sm rounded border"
        style="background-color: var(--bg); border-color: var(--border); color: var(--text); outline: none;"
      />
      <button class="px-2 py-1 text-xs rounded hover:bg-[var(--hover-bg)]" onclick={replaceOne}>Replace</button>
      <button class="px-2 py-1 text-xs rounded hover:bg-[var(--hover-bg)]" onclick={replaceAll}>Replace All</button>
      <label class="flex items-center gap-1 text-xs cursor-pointer">
        <input type="checkbox" bind:checked={useRegex} class="accent-[var(--accent)]" />
        Regex
      </label>
      <label class="flex items-center gap-1 text-xs cursor-pointer">
        <input type="checkbox" bind:checked={caseSensitive} class="accent-[var(--accent)]" />
        Match Case
      </label>
    </div>
  </div>
{/if}
