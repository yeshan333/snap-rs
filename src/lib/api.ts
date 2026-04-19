import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { DockApp } from "./types";

export async function getDockApps(): Promise<DockApp[]> {
  return invoke("get_dock_apps");
}

export async function refreshDockApps(): Promise<DockApp[]> {
  return invoke("refresh_dock_apps");
}

export async function getShortcutsEnabled(): Promise<boolean> {
  return invoke("get_shortcuts_enabled");
}

export async function setShortcutsEnabled(
  enabled: boolean,
): Promise<void> {
  return invoke("set_shortcuts_enabled", { enabled });
}

export async function getAutoStartEnabled(): Promise<boolean> {
  return invoke("get_autostart_enabled");
}

export async function setAutoStartEnabled(
  enabled: boolean,
): Promise<void> {
  return invoke("set_autostart_enabled", { enabled });
}

export async function listenDockChanged(
  callback: (apps: DockApp[]) => void,
): Promise<UnlistenFn> {
  return listen<DockApp[]>("dock-changed", (event) => {
    callback(event.payload);
  });
}
