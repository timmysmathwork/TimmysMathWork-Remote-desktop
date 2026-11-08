use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::ImageFormat;
use serde::Serialize;
use std::io::Cursor;
use xcap::Monitor;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct MonitorInfo {
    id: String,
    name: String,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    is_primary: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ScreenFrame {
    width: u32,
    height: u32,
    data_url: String,
}

#[tauri::command]
fn list_monitors() -> Result<Vec<MonitorInfo>, String> {
    let monitors = Monitor::all().map_err(|error| format!("Could not enumerate displays: {error}"))?;
    monitors
        .into_iter()
        .map(|monitor| {
            Ok(MonitorInfo {
                id: monitor.id().to_string(),
                name: monitor.name().map_err(|error| format!("Could not read display name: {error}"))?,
                width: monitor.width().map_err(|error| format!("Could not read display width: {error}"))?,
                height: monitor.height().map_err(|error| format!("Could not read display height: {error}"))?,
                x: monitor.x().map_err(|error| format!("Could not read display position: {error}"))?,
                y: monitor.y().map_err(|error| format!("Could not read display position: {error}"))?,
                is_primary: monitor.is_primary().map_err(|error| format!("Could not determine primary display: {error}"))?,
            })
        })
        .collect()
}

#[tauri::command]
fn capture_monitor(monitor_id: String) -> Result<ScreenFrame, String> {
    let monitor = Monitor::all()
        .map_err(|error| format!("Could not enumerate displays: {error}"))?
        .into_iter()
        .find(|monitor| monitor.id().to_string() == monitor_id)
        .ok_or_else(|| "The selected display is no longer available. Refresh the display list.".to_string())?;

    let image = monitor.capture_image().map_err(|error| {
        format!("Screen capture failed. On macOS, allow Screen Recording for Relay in System Settings > Privacy & Security > Screen Recording, then restart the app. Details: {error}")
    })?;
    let (width, height) = (image.width(), image.height());
    let mut png = Cursor::new(Vec::new());
    image
        .write_to(&mut png, ImageFormat::Png)
        .map_err(|error| format!("Could not encode the captured display: {error}"))?;

    Ok(ScreenFrame {
        width,
        height,
        data_url: format!("data:image/png;base64,{}", STANDARD.encode(png.into_inner())),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_monitors, capture_monitor])
        .run(tauri::generate_context!())
        .expect("error while running Relay desktop shell");
}
