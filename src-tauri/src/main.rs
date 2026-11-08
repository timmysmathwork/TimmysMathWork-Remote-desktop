use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::ImageFormat;
use serde::Serialize;
use std::{collections::HashMap, io::Cursor, sync::Mutex, time::Duration};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::oneshot;
use uuid::Uuid;
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

/// Details that the user sees before deciding whether to authorize a connection.
/// The identity fields must be authenticated by a future transport before this is called.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionRequest {
    request_id: String,
    device_name: String,
    device_id: String,
    operating_system: String,
    requested_access: Vec<String>,
}

/// Fail-closed authorization broker. It has no network listener: future transport code
/// must authenticate the peer, call request_connection_approval, and wait for the user's
/// explicit decision before starting any session.
#[derive(Default)]
pub struct ApprovalGate {
    pending: Mutex<HashMap<String, oneshot::Sender<bool>>>,
}

/// Wait for explicit local user approval. Timeout, UI failure, or a closed channel denies.
/// This is deliberately not a Tauri command and cannot be invoked by the web frontend.
pub async fn request_connection_approval(
    app: &AppHandle,
    gate: &ApprovalGate,
    device_name: String,
    device_id: String,
    operating_system: String,
    requested_access: Vec<String>,
) -> Result<bool, String> {
    if device_name.trim().is_empty()
        || device_name.len() > 120
        || device_id.trim().is_empty()
        || device_id.len() > 160
        || operating_system.trim().is_empty()
        || operating_system.len() > 80
        || requested_access.is_empty()
        || requested_access.len() > 8
        || requested_access.iter().any(|item| item.trim().is_empty() || item.len() > 80)
    {
        return Err("Connection request metadata is invalid.".into());
    }

    let request_id = Uuid::new_v4().to_string();
    let (sender, receiver) = oneshot::channel();
    {
        let mut pending = gate.pending.lock().map_err(|_| "Approval gate is unavailable.".to_string())?;
        pending.insert(request_id.clone(), sender);
    }

    let request = ConnectionRequest {
        request_id: request_id.clone(),
        device_name,
        device_id,
        operating_system,
        requested_access,
    };
    if let Err(error) = app.emit("connection-request", request) {
        if let Ok(mut pending) = gate.pending.lock() {
            pending.remove(&request_id);
        }
        return Err(format!("Could not display the local approval prompt: {error}"));
    }

    match tokio::time::timeout(Duration::from_secs(60), receiver).await {
        Ok(Ok(approved)) => Ok(approved),
        Ok(Err(_)) => Ok(false),
        Err(_) => {
            if let Ok(mut pending) = gate.pending.lock() {
                pending.remove(&request_id);
            }
            Ok(false)
        }
    }
}

#[tauri::command]
fn decide_connection_request(
    state: State<'_, ApprovalGate>,
    request_id: String,
    approve: bool,
) -> Result<(), String> {
    let sender = state
        .pending
        .lock()
        .map_err(|_| "Approval gate is unavailable.".to_string())?
        .remove(&request_id)
        .ok_or_else(|| "This request is no longer pending or has expired.".to_string())?;
    sender
        .send(approve)
        .map_err(|_| "The requesting session ended; no approval was granted.".to_string())
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
        .manage(ApprovalGate::default())
        .invoke_handler(tauri::generate_handler![list_monitors, capture_monitor, decide_connection_request])
        .run(tauri::generate_context!())
        .expect("error while running Relay desktop shell");
}
