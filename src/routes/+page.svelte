<script lang="ts">
  import { onMount } from "svelte";
  import * as api from "$lib/api";
  import type { DockApp, AppSettings } from "$lib/types";
  import DockAppList from "../components/DockAppList.svelte";
  import SettingsPanel from "../components/SettingsPanel.svelte";
  import AboutSection from "../components/AboutSection.svelte";

  let apps: DockApp[] = $state([]);
  let settings: AppSettings = $state({
    shortcuts_enabled: true,
    autostart_enabled: false,
  });
  let loading = $state(true);
  let unlistenFn: (() => void) | null = $state(null);

  onMount(() => {
    loadInitialData();
    return () => { unlistenFn?.(); };
  });

  async function loadInitialData() {
    try {
      const [dockApps, shortcutsOn, autostartOn] = await Promise.all([
        api.getDockApps(),
        api.getShortcutsEnabled(),
        api.getAutoStartEnabled(),
      ]);
      apps = dockApps;
      settings = {
        shortcuts_enabled: shortcutsOn,
        autostart_enabled: autostartOn,
      };
    } catch (e) {
      console.error("Init error:", e);
    } finally {
      loading = false;
    }

    const unlisten = await api.listenDockChanged((newApps) => {
      apps = newApps;
    });
    unlistenFn = unlisten;
  }

  async function handleRefresh() {
    try {
      apps = await api.refreshDockApps();
    } catch (e) {
      console.error("Refresh error:", e);
    }
  }
</script>

<main class="container">
  {#if loading}
    <div class="loading">Loading...</div>
  {:else}
    <DockAppList {apps} onRefresh={handleRefresh} />
    <SettingsPanel {settings} />
    <AboutSection />
  {/if}
</main>

<style>
  .container {
    display: flex; flex-direction: column; gap: 12px;
    padding: 16px; max-width: 480px; margin: 0 auto;
    user-select: none; -webkit-user-select: none;
  }
  .loading {
    display: flex; align-items: center; justify-content: center;
    height: 200px; color: var(--text-secondary); font-size: 14px;
  }
</style>
