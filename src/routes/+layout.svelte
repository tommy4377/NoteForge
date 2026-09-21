<script lang="ts">
  import '../lib/styles/app.css';
  import { appStore, createDefaultSettings, zoomLevel } from '$lib/stores/app';
  import { onMount, onDestroy, type Snippet } from 'svelte';
  import { browser } from '$app/environment';
  import { Store } from '@tauri-apps/plugin-store';
  import type { AppSettings } from '$lib/stores/app';
  import CustomTitlebar from '$lib/components/CustomTitlebar.svelte';

  let { children }: { children: Snippet } = $props();

  let theme = $state(createDefaultSettings().theme);
  let currentZoom = $state(100);

  $effect(() => {
    if (browser) {
      document.documentElement.setAttribute('data-theme', theme);
    }
  });

  // React to zoom level changes — apply CSS zoom variable
  $effect(() => {
    if (browser) {
      document.documentElement.style.setProperty('--zoom-percent', `${currentZoom}%`);
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

    // Subscribe to zoom level
    const unsubZoom = zoomLevel.subscribe((z) => { currentZoom = z; });

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

    // Ctrl+Scroll zoom handler
    function handleWheel(e: WheelEvent) {
      if (!e.ctrlKey) return;
      e.preventDefault();

      const step = e.deltaY < 0 ? 10 : -10;
      zoomLevel.update((z) => {
        const next = z + step;
        return Math.max(50, Math.min(200, next));
      });
    }

    window.addEventListener('wheel', handleWheel, { passive: false });

    return () => {
      unsub();
      unsubZoom();
      window.removeEventListener('wheel', handleWheel);
    };
  });
</script>

<CustomTitlebar />
<div class="workspace-zoom">
  {@render children()}
</div>