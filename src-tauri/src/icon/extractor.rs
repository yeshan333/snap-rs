use std::io::Cursor;
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use icns::{IconFamily, IconType};
use image::ImageEncoder;
use image::codecs::png::PngEncoder;

/// Preferred icon types in descending priority.
const PREFERRED_TYPES: &[IconType] = &[
    IconType::RGBA32_128x128_2x,
    IconType::RGBA32_128x128,
    IconType::RGBA32_64x64,
    IconType::RGBA32_32x32_2x,
    IconType::RGBA32_32x32,
    IconType::RGBA32_16x16_2x,
];

/// Extract an app icon from its bundle and return as base64 data URI.
pub fn extract_icon_base64(app_path: &str) -> Option<String> {
    if app_path.is_empty() {
        return None;
    }

    let app_dir = Path::new(app_path);
    let icns_path = find_icns_path(app_dir)?;

    let png_bytes = icns_to_png(&icns_path)?;
    let b64 = BASE64.encode(&png_bytes);

    Some(format!("data:image/png;base64,{b64}"))
}

fn find_icns_path(app_dir: &Path) -> Option<std::path::PathBuf> {
    let info_plist = app_dir.join("Contents/Info.plist");
    let icon_name = read_icon_name_from_plist(&info_plist);

    let resources = app_dir.join("Contents/Resources");

    if let Some(name) = icon_name {
        // Try with and without .icns extension
        let with_ext = if name.ends_with(".icns") {
            resources.join(&name)
        } else {
            resources.join(format!("{name}.icns"))
        };
        if with_ext.exists() {
            return Some(with_ext);
        }
    }

    // Fallback: look for common icon names
    for fallback in &["AppIcon.icns", "app.icns", "icon.icns"] {
        let path = resources.join(fallback);
        if path.exists() {
            return Some(path);
        }
    }

    // Last resort: find any .icns file
    find_first_icns_file(&resources)
}

fn read_icon_name_from_plist(path: &Path) -> Option<String> {
    let value = plist::Value::from_file(path).ok()?;
    let dict = value.as_dictionary()?;

    // Try CFBundleIconFile first, then CFBundleIconName
    dict.get("CFBundleIconFile")
        .or_else(|| dict.get("CFBundleIconName"))
        .and_then(|v| v.as_string())
        .map(|s| s.to_string())
}

fn find_first_icns_file(dir: &Path) -> Option<std::path::PathBuf> {
    let entries = std::fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "icns") {
            return Some(path);
        }
    }
    None
}

fn icns_to_png(icns_path: &Path) -> Option<Vec<u8>> {
    let file = std::fs::File::open(icns_path).ok()?;
    let icon_family = IconFamily::read(file).ok()?;

    // Find best available icon type
    let available = icon_family.available_icons();
    let icon_type = PREFERRED_TYPES
        .iter()
        .find(|t| available.contains(t))
        .or_else(|| available.first())?;

    let icon_image = icon_family.get_icon_with_type(*icon_type).ok()?;
    let (w, h) = (icon_image.width(), icon_image.height());
    let rgba_data = icon_image.data();

    let mut png_buf = Vec::new();
    let encoder = PngEncoder::new(Cursor::new(&mut png_buf));
    encoder
        .write_image(rgba_data, w, h, image::ExtendedColorType::Rgba8)
        .ok()?;

    Some(png_buf)
}
