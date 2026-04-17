use crate::config;
use std::path::PathBuf;

use reqwest::multipart::Part;
use tokio::{fs::File, io::AsyncReadExt};

pub async fn get_file_part(file: &mut File, path: PathBuf) -> Result<Part, String> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;
    let part =
        Part::bytes(buffer).file_name(path.file_name().unwrap().to_str().unwrap().to_string());
    Ok(part)
}

#[tauri::command]
pub async fn add_file(client_id: u32, category: String, filepath: String) -> Result<(), String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}/files", api_url, client_id);

    let client = reqwest::Client::new();
    let mut form = reqwest::multipart::Form::new();

    form = form.text("category", category);

    let path = PathBuf::from(&filepath);
    match File::open(&path).await {
        Ok(mut file) => {
            let part = get_file_part(&mut file, path).await?;
            form = form.part("file", part);
        }
        Err(e) => return Err(format!("Failed to open file: {}", e)),
    }

    let response = client
        .post(endpoint)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("Failed to upload file: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error uploading file: {} with status code {}",
            error_message, status_code
        ))
    }
}

#[tauri::command]
pub async fn delete_file(file_id: u32, client_id: u32) -> Result<(), String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}/files/{}", api_url, client_id, file_id);

    let client = reqwest::Client::new();
    let response = client
        .delete(endpoint)
        .send()
        .await
        .map_err(|e| format!("Failed to delete file: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error deleting file: {} with status code {}",
            error_message, status_code
        ))
    }
}
