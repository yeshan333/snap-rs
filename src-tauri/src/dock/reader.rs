use std::path::PathBuf;

use plist::Value;

use super::types::{shortcut_label_for_index, DockApp};
use crate::icon;

/// Maximum number of dock apps to assign shortcuts to.
const MAX_SHORTCUT_APPS: usize = 10;

/// Read the macOS Dock persistent-apps from the Dock plist.
pub fn read_dock_apps() -> Result<Vec<DockApp>, String> {
    let plist_path = dock_plist_path().ok_or("Cannot resolve home directory")?;

    let value = Value::from_file(&plist_path)
        .map_err(|e| format!("Failed to read Dock plist: {e}"))?;

    let dict = value
        .as_dictionary()
        .ok_or("Dock plist root is not a dictionary")?;

    let persistent_apps = dict
        .get("persistent-apps")
        .and_then(|v| v.as_array())
        .ok_or("Missing persistent-apps array")?;

    let mut apps = Vec::new();

    for entry in persistent_apps {
        if let Some(app) = parse_dock_entry(entry) {
            apps.push(app);
        }
    }

    // Assign shortcut keys and extract icons for the first N apps
    for (i, app) in apps.iter_mut().enumerate() {
        if i < MAX_SHORTCUT_APPS {
            app.shortcut_key = shortcut_label_for_index(i);
        }
        app.icon_base64 = icon::extract_icon_base64(&app.app_path);
    }

    Ok(apps)
}

/// Extract bundle IDs in order for change detection.
pub fn read_dock_bundle_ids() -> Vec<String> {
    read_dock_apps()
        .unwrap_or_default()
        .into_iter()
        .map(|a| a.bundle_id)
        .collect()
}

fn dock_plist_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| {
        h.join("Library/Preferences/com.apple.dock.plist")
    })
}

fn parse_dock_entry(entry: &Value) -> Option<DockApp> {
    let dict = entry.as_dictionary()?;

    // Skip spacer tiles
    let tile_type = dict
        .get("tile-type")
        .and_then(|v| v.as_string())?;
    if tile_type != "file-tile" {
        return None;
    }

    let tile_data = dict.get("tile-data")?.as_dictionary()?;

    let label = tile_data
        .get("file-label")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();

    let bundle_id = tile_data
        .get("bundle-identifier")
        .and_then(|v| v.as_string())?
        .to_string();

    let app_path = extract_app_path(tile_data).unwrap_or_default();

    if bundle_id.is_empty() {
        return None;
    }

    Some(DockApp::new(label, bundle_id, app_path))
}

fn extract_app_path(
    tile_data: &plist::Dictionary,
) -> Option<String> {
    let file_data = tile_data.get("file-data")?.as_dictionary()?;
    let url_str = file_data
        .get("_CFURLString")
        .and_then(|v| v.as_string())?;

    // Decode file:// URL to a filesystem path
    if let Ok(parsed) = url::Url::parse(url_str) {
        Some(
            parsed
                .to_file_path()
                .ok()?
                .to_string_lossy()
                .into_owned(),
        )
    } else {
        // Fallback: strip file:// prefix manually
        url_str
            .strip_prefix("file://")
            .map(|s| {
                url::form_urlencoded::parse(s.as_bytes())
                    .map(|(k, _)| k.into_owned())
                    .collect::<String>()
            })
            .or_else(|| Some(url_str.to_string()))
    }
}
