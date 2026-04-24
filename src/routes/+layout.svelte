<script lang="ts">
  import '../lib/styles/app.css';
  import { appStore, createDefaultSettings } from '$lib/stores/app';
  import { onMount, type Snippet } from 'svelte';
  import { browser } from '$app/environment';
  import { Store } from '@tauri-apps/plugin-store';
  import type { AppSettings } from '$lib/stores/app';
  import CustomTitlebar from '$lib/components/CustomTitlebar.svelte';

  let { children }: { children: Snippet } = $props();

  let theme = $state(createDefaultSettings().theme);

  $effect(() => {
    if (browser) {
      document.documentElement.setAttribute('data-theme', theme);
    }
  });

  onMount(() => {
    const unsub = appStore.settings.subscribe((s) => {
      theme = s.theme;
      if (browser) {
        document.documentElement.style.setProperty('--font-family', s.fontFamily);
        document.documentElement.style.setProperty('--font-size', `${s.fontSize}px`);
      }
    });

    async function loadSettings() {
      try {
        const store = await Store.load('settings.json');
        const saved = await store.get<AppSettings>('settings');
        if (saved) {
          appStore.settings.set(saved);
        }
      } catch {
        // first launch, no saved settings yet
      }
    }
    loadSettings();

    return unsub;
  });
</script>

<CustomTitlebar />
{@render children()}
