use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt as AutostartManagerExt;
use tauri_plugin_store::StoreExt;

use crate::dock::{self, DockApp};
use crate::shortcut;

#[tauri::command]
pub fn get_dock_apps() -> Result<Vec<DockApp>, String> {
    dock::read_dock_apps()
}

#[tauri::command]
pub fn refresh_dock_apps(
    app: AppHandle,
) -> Result<Vec<DockApp>, String> {
    let apps = dock::read_dock_apps()?;
    shortcut::register_shortcuts(&app, &apps)?;
    Ok(apps)
}

#[tauri::command]
pub fn get_shortcuts_enabled(
    app: AppHandle,
) -> Result<bool, String> {
    let store = app
        .store("settings.json")
        .map_err(|e| format!("Store error: {e}"))?;

    Ok(store
        .get("shortcuts_enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true))
}

#[tauri::command]
pub fn set_shortcuts_enabled(
    app: AppHandle,
    enabled: bool,
) -> Result<(), String> {
    let store = app
        .store("settings.json")
        .map_err(|e| format!("Store error: {e}"))?;

    store.set("shortcuts_enabled", enabled);

    if enabled {
        let apps = dock::read_dock_apps()?;
        shortcut::register_shortcuts(&app, &apps)?;
    } else {
        shortcut::unregister_all(&app)?;
    }

    Ok(())
}

#[tauri::command]
pub fn get_autostart_enabled(
    app: AppHandle,
) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|e| format!("Autostart error: {e}"))
}

#[tauri::command]
pub fn set_autostart_enabled(
    app: AppHandle,
    enabled: bool,
) -> Result<(), String> {
    let manager = app.autolaunch();
    if enabled {
        manager
            .enable()
            .map_err(|e| format!("Autostart enable error: {e}"))
    } else {
        manager
            .disable()
            .map_err(|e| format!("Autostart disable error: {e}"))
    }
}
