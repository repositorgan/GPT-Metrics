#[tauri::command]
async fn track_app_open() -> Result<(), String> {
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "event": "app_open",
        "timestamp": chrono::Utc::now().timestamp(),
        "machine_id": tauri::api::hash::hash("unique-machine-id")
    });

    client.post("https://your-admin-endpoint.com/track")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
