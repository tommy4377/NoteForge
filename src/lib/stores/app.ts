// NoteForge — Central State Store
// Uses Svelte 5 runes ($state) exported from module-level reactive context

import { writable } from 'svelte/store';

export type Theme = 'light' | 'dark' | 'sepia';

export interface TabState {
  id: string;
  filePath: string;
  fileName: string;
  content: string;
  originalContent: string;
  encoding: string;
  lineEnding: string; // 'LF' | 'CRLF'
  isDirty: boolean;
  language: string;
}

export interface AppSettings {
  fontFamily: string;
  fontSize: number;
  theme: Theme;
  autosaveInterval: number; // seconds, 0 = off
  defaultEncoding: string;
  wordWrap: boolean;
  showPreview: boolean;
  showFindReplace: boolean;
}

export function createDefaultSettings(): AppSettings {
  return {
    fontFamily: '"Cascadia Code", "Fira Code", "Consolas", monospace',
    fontSize: 14,
    theme: 'light',
    autosaveInterval: 0,
    defaultEncoding: 'utf-8',
    wordWrap: true,
    showPreview: false,
    showFindReplace: false,
  };
}

// We use Svelte writable stores for cross-component reactivity in Svelte 5
// These can be imported and used with $store syntax in .svelte files

function createAppStore() {
  const tabs = writable<TabState[]>([]);
  const activeTabId = writable<string | null>(null);
  const settings = writable<AppSettings>(createDefaultSettings());
  const recentFiles = writable<string[]>([]);
  const fileChangedNotification = writable<{ path: string; visible: boolean } | null>(null);

  return {
    tabs,
    activeTabId,
    settings,
    recentFiles,
    fileChangedNotification,
  };
}

export const appStore = createAppStore();