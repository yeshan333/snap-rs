<script lang="ts">
  import type { DockApp } from "$lib/types";

  let { apps, onRefresh }: { apps: DockApp[]; onRefresh: () => void } = $props();
</script>

<section class="card">
  <div class="card-header">
    <h2>Dock Apps</h2>
    <button class="btn-refresh" onclick={onRefresh}>Refresh</button>
  </div>

  <div class="app-list">
    {#each apps as app, i}
      <div class="app-row" class:has-shortcut={app.shortcut_key !== null}>
        <div class="app-icon">
          {#if app.icon_base64}
            <img src={app.icon_base64} alt={app.name} width="32" height="32" />
          {:else}
            <div class="icon-placeholder">
              <span>{app.name.charAt(0)}</span>
            </div>
          {/if}
        </div>
        <div class="app-name">{app.name}</div>
        {#if app.shortcut_key}
          <kbd class="shortcut-badge">{app.shortcut_key}</kbd>
        {/if}
      </div>
    {/each}

    {#if apps.length === 0}
      <div class="empty-state">No Dock apps found</div>
    {/if}
  </div>
</section>

<style>
  .card { background: var(--card-bg); border-radius: 10px; overflow: hidden; }
  .card-header {
    display: flex; justify-content: space-between; align-items: center;
    padding: 12px 16px; border-bottom: 1px solid var(--border);
  }
  h2 { margin: 0; font-size: 14px; font-weight: 600; }
  .btn-refresh {
    font-size: 12px; padding: 4px 12px; border-radius: 6px;
    border: 1px solid var(--border); background: var(--btn-bg);
    color: var(--text); cursor: pointer;
  }
  .btn-refresh:hover { background: var(--btn-hover); }
  .app-list { max-height: 340px; overflow-y: auto; }
  .app-row {
    display: flex; align-items: center; gap: 12px;
    padding: 8px 16px; border-bottom: 1px solid var(--border);
  }
  .app-row:last-child { border-bottom: none; }
  .app-row:not(.has-shortcut) { opacity: 0.5; }
  .app-icon { flex-shrink: 0; width: 32px; height: 32px; }
  .app-icon img { border-radius: 6px; }
  .icon-placeholder {
    width: 32px; height: 32px; border-radius: 6px;
    background: var(--placeholder-bg); display: flex;
    align-items: center; justify-content: center;
    font-size: 14px; font-weight: 600; color: var(--text-secondary);
  }
  .app-name { flex: 1; font-size: 13px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .shortcut-badge {
    flex-shrink: 0; font-size: 11px; padding: 2px 8px;
    border-radius: 4px; background: var(--badge-bg);
    color: var(--badge-text); font-family: inherit;
    border: 1px solid var(--badge-border);
  }
  .empty-state {
    padding: 32px; text-align: center;
    color: var(--text-secondary); font-size: 13px;
  }
</style>
