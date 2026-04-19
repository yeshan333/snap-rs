use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockApp {
    pub name: String,
    pub bundle_id: String,
    pub app_path: String,
    pub icon_base64: Option<String>,
    pub shortcut_key: Option<String>,
}

impl DockApp {
    pub fn new(name: String, bundle_id: String, app_path: String) -> Self {
        Self {
            name,
            bundle_id,
            app_path,
            icon_base64: None,
            shortcut_key: None,
        }
    }
}

/// Assign shortcut labels to dock apps (first 10 only).
/// Index 0 -> "1", index 1 -> "2", ... index 8 -> "9", index 9 -> "0"
pub fn shortcut_label_for_index(index: usize) -> Option<String> {
    match index {
        0..=8 => Some(format!("{}", index + 1)),
        9 => Some("0".to_string()),
        _ => None,
    }
}
