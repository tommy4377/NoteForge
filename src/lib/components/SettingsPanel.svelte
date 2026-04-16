<script lang="ts">
  import { appStore, createDefaultSettings } from '$lib/stores/app';
  import type { AppSettings, Theme } from '$lib/stores/app';
  import { Store } from '@tauri-apps/plugin-store';

  let {
    visible,
    onClose,
  }: {
    visible: boolean;
    onClose: () => void;
  } = $props();

  let settings = $state<AppSettings>(createDefaultSettings());

  $effect(() => {
    const unsub = appStore.settings.subscribe((s) => {
      settings = { ...s };
    });
    return unsub;
  });

  async function saveSettings() {
    appStore.settings.set({ ...settings });
    try {
      const store = await Store.load('settings.json');
      await store.set('settings', { ...settings });
      await store.save();
    } catch {
      // persistence may fail in dev
    }
  }

  function updateField<K extends keyof AppSettings>(key: K, value: AppSettings[K]) {
    settings[key] = value;
    saveSettings();
  }

  const fontFamilies = [
    { label: 'Cascadia Code', value: '"Cascadia Code", "Fira Code", "Consolas", monospace' },
    { label: 'Fira Code', value: '"Fira Code", "Consolas", monospace' },
    { label: 'Consolas', value: '"Consolas", "Courier New", monospace' },
    { label: 'JetBrains Mono', value: '"JetBrains Mono", "Consolas", monospace' },
    { label: 'Source Code Pro', value: '"Source Code Pro", "Consolas", monospace' },
    { label: 'Monaco', value: '"Monaco", "Consolas", monospace' },
  ];

  const fontSizes = [10, 11, 12, 13, 14, 15, 16, 18, 20, 22, 24];

  const themes: { label: string; value: Theme }[] = [
    { label: 'Light', value: 'light' },
    { label: 'Dark', value: 'dark' },
    { label: 'Sepia', value: 'sepia' },
  ];

  const encodings = ['utf-8', 'utf-16le', 'utf-16be', 'ascii', 'iso-8859-1'];
</script>

{#if visible}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="settings-overlay fixed inset-0 z-50 flex items-center justify-center"
       style="background-color: rgba(0,0,0,0.4);"
       onclick={(e) => { if ((e.target as HTMLElement).classList.contains('settings-overlay')) onClose(); }}
       onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
       role="dialog"
       aria-modal="true"
       tabindex="-1">
    <div class="settings-panel w-96 max-h-[80vh] overflow-y-auto rounded-lg p-6"
         style="background-color: var(--bg); color: var(--text); border: 1px solid var(--border); box-shadow: var(--shadow-lg);">
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-lg font-semibold">Settings</h2>
        <button class="w-7 h-7 flex items-center justify-center rounded hover:bg-[var(--hover-bg)]"
                onclick={onClose}>✕</button>
      </div>

      <div class="space-y-4">
        <!-- Font Family -->
        <div>
          <label class="block text-sm font-medium mb-1" for="font-family">Font Family</label>
          <select id="font-family"
                  class="w-full px-2 py-1.5 text-sm rounded border"
                  style="background-color: var(--bg); border-color: var(--border); color: var(--text);"
                  value={settings.fontFamily}
                  onchange={(e) => updateField('fontFamily', (e.target as HTMLSelectElement).value)}>
            {#each fontFamilies as ff}
              <option value={ff.value}>{ff.label}</option>
            {/each}
          </select>
        </div>

        <!-- Font Size -->
        <div>
          <label class="block text-sm font-medium mb-1" for="font-size">Font Size</label>
          <select id="font-size"
                  class="w-full px-2 py-1.5 text-sm rounded border"
                  style="background-color: var(--bg); border-color: var(--border); color: var(--text);"
                  value={String(settings.fontSize)}
                  onchange={(e) => updateField('fontSize', parseInt((e.target as HTMLSelectElement).value))}>
            {#each fontSizes as size}
              <option value={String(size)}>{size}px</option>
            {/each}
          </select>
        </div>

        <!-- Theme -->
        <div>
          <label class="block text-sm font-medium mb-1" for="theme">Theme</label>
          <select id="theme"
                  class="w-full px-2 py-1.5 text-sm rounded border"
                  style="background-color: var(--bg); border-color: var(--border); color: var(--text);"
                  value={settings.theme}
                  onchange={(e) => updateField('theme', (e.target as HTMLSelectElement).value as Theme)}>
            {#each themes as t}
              <option value={t.value}>{t.label}</option>
            {/each}
          </select>
        </div>

        <!-- Autosave Interval -->
        <div>
          <label class="block text-sm font-medium mb-1" for="autosave">Autosave Interval (seconds, 0 = off)</label>
          <input id="autosave"
                 type="number"
                 min="0"
                 max="3600"
                 class="w-full px-2 py-1.5 text-sm rounded border"
                 style="background-color: var(--bg); border-color: var(--border); color: var(--text);"
                 value={settings.autosaveInterval}
                 oninput={(e) => updateField('autosaveInterval', parseInt((e.target as HTMLInputElement).value) || 0)} />
        </div>

        <!-- Default Encoding -->
        <div>
          <label class="block text-sm font-medium mb-1" for="encoding">Default Encoding</label>
          <select id="encoding"
                  class="w-full px-2 py-1.5 text-sm rounded border"
                  style="background-color: var(--bg); border-color: var(--border); color: var(--text);"
                  value={settings.defaultEncoding}
                  onchange={(e) => updateField('defaultEncoding', (e.target as HTMLSelectElement).value)}>
            {#each encodings as enc}
              <option value={enc}>{enc}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>
  </div>
{/if}
