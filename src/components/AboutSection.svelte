<script lang="ts">
  const version = "0.1.0";
</script>

<section class="card about">
  <div class="about-header">
    <h1 class="app-name">Snap-rs</h1>
    <span class="version">v{version}</span>
  </div>
  <p class="description">
    Keyboard shortcuts for your Dock apps.
    Press &#8984;1&ndash;&#8984;0 to instantly launch or switch to
    the corresponding app in your Dock.
  </p>
  <p class="tech-note">Built with Tauri + Svelte + Rust</p>
</section>

<style>
  .card { background: var(--card-bg); border-radius: 10px; padding: 16px; }
  .about { text-align: center; }
  .about-header {
    display: flex; align-items: baseline; justify-content: center; gap: 8px;
  }
  .app-name { margin: 0; font-size: 18px; font-weight: 700; }
  .version { font-size: 12px; color: var(--text-secondary); }
  .description {
    font-size: 12px; color: var(--text-secondary);
    line-height: 1.5; margin: 8px 0 4px;
  }
  .tech-note { font-size: 11px; color: var(--text-tertiary); margin: 4px 0 0; }
</style>
