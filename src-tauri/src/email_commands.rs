use crate::config;

#[tauri::command]
pub async fn send_file_email(file_id: u32, client_id: u32) -> Result<(), String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}/files/{}/send", api_url, client_id, file_id);

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .send()
        .await
        .map_err(|e| format!("Failed to send file: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error sending file: {} with status code {}",
            error_message, status_code
        ))
    }
}

#[tauri::command]
pub async fn send_category_files_email(category: String, client_id: u32) -> Result<(), String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}/files/send/{}", api_url, client_id, category);

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint)
        .send()
        .await
        .map_err(|e| format!("Failed to send files: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error sending files: {} with status code {}",
            error_message, status_code
        ))
    }
}
