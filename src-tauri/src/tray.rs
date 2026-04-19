use tauri::{
    AppHandle, Emitter, Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
};

use crate::dock;
use crate::shortcut;

pub fn setup_tray(app: &AppHandle) -> Result<(), String> {
    let settings_item = MenuItem::with_id(
        app, "settings", "Settings...", true, None::<&str>,
    )
    .map_err(|e| format!("Menu item error: {e}"))?;

    let refresh_item = MenuItem::with_id(
        app, "refresh", "Refresh Dock", true, None::<&str>,
    )
    .map_err(|e| format!("Menu item error: {e}"))?;

    let about_item = MenuItem::with_id(
        app, "about", "About Snap-rs", true, None::<&str>,
    )
    .map_err(|e| format!("Menu item error: {e}"))?;

    let quit_item = MenuItem::with_id(
        app, "quit", "Quit", true, None::<&str>,
    )
    .map_err(|e| format!("Menu item error: {e}"))?;

    let separator = PredefinedMenuItem::separator(app)
        .map_err(|e| format!("Separator error: {e}"))?;
    let separator2 = PredefinedMenuItem::separator(app)
        .map_err(|e| format!("Separator error: {e}"))?;

    let menu = Menu::with_items(
        app,
        &[
            &settings_item,
            &refresh_item,
            &separator,
            &about_item,
            &separator2,
            &quit_item,
        ],
    )
    .map_err(|e| format!("Menu error: {e}"))?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().cloned().unwrap())
        .menu(&menu)
        .tooltip("Snap-rs")
        .on_menu_event(handle_menu_event)
        .build(app)
        .map_err(|e| format!("Tray error: {e}"))?;

    Ok(())
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "settings" => show_settings_window(app),
        "refresh" => handle_refresh(app),
        "about" => show_settings_window(app),
        "quit" => app.exit(0),
        _ => {}
    }
}

fn show_settings_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn handle_refresh(app: &AppHandle) {
    match dock::read_dock_apps() {
        Ok(apps) => {
            if let Err(e) =
                shortcut::register_shortcuts(app, &apps)
            {
                log::error!("Refresh shortcut error: {e}");
            }
            let _ = app.emit("dock-changed", &apps);
        }
        Err(e) => log::error!("Refresh dock error: {e}"),
    }
}
