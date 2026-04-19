use tauri::AppHandle;
use tauri_plugin_global_shortcut::{
    GlobalShortcutExt, Shortcut, ShortcutState,
};

use crate::app_launcher;
use crate::dock::DockApp;

/// Maximum number of shortcuts to register (Cmd+1 through Cmd+0).
const MAX_SHORTCUTS: usize = 10;

/// Register global shortcuts for the given dock apps.
///
/// First unregisters all existing shortcuts, then registers
/// Cmd+1 through Cmd+0 for the first 10 apps.
pub fn register_shortcuts(
    app: &AppHandle,
    dock_apps: &[DockApp],
) -> Result<(), String> {
    unregister_all(app)?;

    let count = dock_apps.len().min(MAX_SHORTCUTS);

    for i in 0..count {
        let Some(key) = shortcut_key_for_index(i) else {
            continue;
        };
        let Ok(shortcut) = key.parse::<Shortcut>() else {
            continue;
        };
        let bundle_id = dock_apps[i].bundle_id.clone();

        if let Err(e) = app.global_shortcut().on_shortcut(
            shortcut,
            move |_app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    if let Err(e) =
                        app_launcher::activate_app(&bundle_id)
                    {
                        log::error!(
                            "Shortcut activate error: {e}"
                        );
                    }
                }
            },
        ) {
            log::warn!("Failed to register shortcut {key}: {e}");
        }
    }

    Ok(())
}

/// Unregister all currently registered global shortcuts.
pub fn unregister_all(app: &AppHandle) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| format!("Failed to unregister shortcuts: {e}"))
}

/// Map dock index to shortcut key string.
/// Index 0 -> "CmdOrCtrl+1", ... Index 8 -> "CmdOrCtrl+9",
/// Index 9 -> "CmdOrCtrl+0"
fn shortcut_key_for_index(index: usize) -> Option<String> {
    match index {
        0..=8 => Some(format!("CmdOrCtrl+{}", index + 1)),
        9 => Some("CmdOrCtrl+0".to_string()),
        _ => None,
    }
}
