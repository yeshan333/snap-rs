<script lang="ts">
  import * as api from "$lib/api";
  import type { AppSettings } from "$lib/types";

  let { settings }: { settings: AppSettings } = $props();

  let shortcutsEnabled = $derived(settings.shortcuts_enabled);
  let autostartEnabled = $derived(settings.autostart_enabled);
  let shortcutsLocal = $state<boolean | null>(null);
  let autostartLocal = $state<boolean | null>(null);

  let shortcutsOn = $derived(shortcutsLocal ?? shortcutsEnabled);
  let autostartOn = $derived(autostartLocal ?? autostartEnabled);

  async function toggleShortcuts() {
    const newVal = !shortcutsOn;
    shortcutsLocal = newVal;
    try {
      await api.setShortcutsEnabled(newVal);
    } catch (e) {
      shortcutsLocal = null;
      console.error("Failed to toggle shortcuts:", e);
    }
  }

  async function toggleAutostart() {
    const newVal = !autostartOn;
    autostartLocal = newVal;
    try {
      await api.setAutoStartEnabled(newVal);
    } catch (e) {
      autostartLocal = null;
      console.error("Failed to toggle autostart:", e);
    }
  }
</script>

<section class="card">
  <div class="setting-row">
    <div class="setting-info">
      <span class="setting-label">Enable Shortcuts</span>
      <span class="setting-desc">
        Register global keyboard shortcuts (&#8984;1-&#8984;0)
      </span>
    </div>
    <button
      class="toggle"
      class:active={shortcutsOn}
      onclick={toggleShortcuts}
      role="switch"
      aria-checked={shortcutsOn}
      aria-label="Enable Shortcuts"
    >
      <span class="toggle-knob"></span>
    </button>
  </div>

  <div class="divider"></div>

  <div class="setting-row">
    <div class="setting-info">
      <span class="setting-label">Launch at Login</span>
      <span class="setting-desc">Automatically start when you log in</span>
    </div>
    <button
      class="toggle"
      class:active={autostartOn}
      onclick={toggleAutostart}
      role="switch"
      aria-checked={autostartOn}
      aria-label="Launch at Login"
    >
      <span class="toggle-knob"></span>
    </button>
  </div>
</section>

<style>
  .card { background: var(--card-bg); border-radius: 10px; padding: 0; }
  .setting-row {
    display: flex; align-items: center; justify-content: space-between;
    padding: 12px 16px;
  }
  .setting-info { display: flex; flex-direction: column; gap: 2px; }
  .setting-label { font-size: 13px; font-weight: 500; }
  .setting-desc { font-size: 11px; color: var(--text-secondary); }
  .divider { height: 1px; background: var(--border); margin: 0 16px; }

  .toggle {
    position: relative; width: 42px; height: 24px; border-radius: 12px;
    border: none; background: var(--toggle-off); cursor: pointer;
    transition: background 0.2s ease; flex-shrink: 0;
  }
  .toggle.active { background: var(--toggle-on); }
  .toggle-knob {
    position: absolute; top: 2px; left: 2px;
    width: 20px; height: 20px; border-radius: 50%;
    background: white; transition: transform 0.2s ease;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
  }
  .toggle.active .toggle-knob { transform: translateX(18px); }
</style>
