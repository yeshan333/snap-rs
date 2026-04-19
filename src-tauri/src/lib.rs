mod app_launcher;
mod commands;
mod dock;
mod icon;
mod shortcut;
mod tray;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::{Emitter, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new().build(),
        )
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_dock_apps,
            commands::refresh_dock_apps,
            commands::get_shortcuts_enabled,
            commands::set_shortcuts_enabled,
            commands::get_autostart_enabled,
            commands::set_autostart_enabled,
        ])
        .setup(|app| {
            // Hide from Dock — menu bar only app
            #[cfg(target_os = "macos")]
            app.set_activation_policy(
                tauri::ActivationPolicy::Accessory,
            );

            // Setup system tray
            if let Err(e) = tray::setup_tray(app.handle()) {
                log::error!("Tray setup failed: {e}");
            }

            // Initial dock read and shortcut registration
            let handle = app.handle().clone();
            let prev_ids: Arc<Mutex<Vec<String>>> =
                Arc::new(Mutex::new(Vec::new()));

            match dock::read_dock_apps() {
                Ok(apps) => {
                    if let Ok(mut ids) = prev_ids.lock() {
                        *ids = apps
                            .iter()
                            .map(|a| a.bundle_id.clone())
                            .collect();
                    }
                    if let Err(e) =
                        shortcut::register_shortcuts(
                            app.handle(),
                            &apps,
                        )
                    {
                        log::error!(
                            "Initial shortcut error: {e}"
                        );
                    }
                }
                Err(e) => {
                    log::error!("Initial dock read error: {e}")
                }
            }

            // Spawn background polling thread
            let poll_handle = handle.clone();
            let poll_ids = prev_ids.clone();
            std::thread::spawn(move || {
                poll_dock_changes(poll_handle, poll_ids);
            });

            // Hide window on close instead of destroying
            if let Some(window) =
                app.get_webview_window("main")
            {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested {
                        api,
                        ..
                    } = event
                    {
                        api.prevent_close();
                        let _ = win.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn poll_dock_changes(
    app: tauri::AppHandle,
    prev_ids: Arc<Mutex<Vec<String>>>,
) {
    loop {
        std::thread::sleep(Duration::from_secs(5));

        let new_ids = dock::reader::read_dock_bundle_ids();

        let changed = {
            let guard =
                prev_ids.lock().unwrap_or_else(|e| e.into_inner());
            *guard != new_ids
        };

        if changed {
            if let Ok(mut guard) = prev_ids.lock() {
                *guard = new_ids;
            }

            match dock::read_dock_apps() {
                Ok(apps) => {
                    let enabled = app
                        .store("settings.json")
                        .ok()
                        .and_then(|s| {
                            s.get("shortcuts_enabled")
                                .and_then(|v| v.as_bool())
                        })
                        .unwrap_or(true);

                    if enabled {
                        if let Err(e) =
                            shortcut::register_shortcuts(
                                &app, &apps,
                            )
                        {
                            log::error!(
                                "Poll shortcut error: {e}"
                            );
                        }
                    }
                    let _ = app.emit("dock-changed", &apps);
                }
                Err(e) => {
                    log::error!("Poll dock read error: {e}");
                }
            }
        }
    }
}
