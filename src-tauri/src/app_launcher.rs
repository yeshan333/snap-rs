use std::process::Command;

/// Toggle an application: activate it if not frontmost, hide it if
/// it is already the frontmost app.
pub fn activate_app(bundle_id: &str) -> Result<(), String> {
    if is_frontmost(bundle_id) {
        hide_app(bundle_id)
    } else {
        launch_or_focus(bundle_id)
    }
}

fn launch_or_focus(bundle_id: &str) -> Result<(), String> {
    Command::new("open")
        .args(["-b", bundle_id])
        .spawn()
        .map_err(|e| {
            format!("Failed to activate app {bundle_id}: {e}")
        })?;
    Ok(())
}

fn is_frontmost(bundle_id: &str) -> bool {
    let script = concat!(
        "tell application \"System Events\" to get ",
        "bundle identifier of first application ",
        "process whose frontmost is true",
    );
    Command::new("osascript")
        .args(["-e", script])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .is_some_and(|id| id.trim() == bundle_id)
}

fn hide_app(bundle_id: &str) -> Result<(), String> {
    let script = format!(
        "tell application \"System Events\" to set visible of \
         (first application process whose bundle identifier \
         is \"{bundle_id}\") to false"
    );
    Command::new("osascript")
        .args(["-e", &script])
        .spawn()
        .map_err(|e| {
            format!("Failed to hide app {bundle_id}: {e}")
        })?;
    Ok(())
}
