#[tauri::command]
async fn track_event(event_type: String) -> Result<(), String> {
    let client = reqwest::Client::new();

    let payload = serde_json::json!({
        "event": event_type,
        "timestamp": chrono::Utc::now().timestamp(),
    });

    client
        .post("https://gpt-metrics-cloudflare.andrewkieckhefer.workers.dev")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
