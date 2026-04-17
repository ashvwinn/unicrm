use crate::{
    api_types::{ClientResponse, CreateFormData, FetchClientsResponse, UpdateClientRequest},
    config,
    file_commands::get_file_part,
};
use reqwest::multipart::Form;
use std::path::PathBuf;
use tokio::fs::File;

#[tauri::command]
pub async fn fetch_clients(query_params: Option<String>) -> Result<FetchClientsResponse, String> {
    let api_url = config::api_url();
    let query_params = query_params.unwrap_or_default();
    let endpoint = format!("{}/clients?{}", api_url, query_params);

    let response = reqwest::get(&endpoint)
        .await
        .map_err(|e| format!("Failed to fetch clients: {}", e))?;

    if response.status().is_success() {
        let clients = response
            .json::<FetchClientsResponse>()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        Ok(clients)
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error fetching clients: {} with status code {}",
            error_message, status_code
        ))
    }
}

#[tauri::command]
pub async fn fetch_client(client_id: u32) -> Result<ClientResponse, String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}", api_url, client_id);

    let response = reqwest::get(&endpoint)
        .await
        .map_err(|e| format!("Failed to fetch client: {}", e))?;

    if response.status().is_success() {
        let client = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;
        Ok(client)
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error fetching client: {} with status code {}",
            error_message, status_code
        ))
    }
}

#[tauri::command]
pub async fn create_client(form_data: CreateFormData) -> Result<ClientResponse, String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients", api_url);

    let client = reqwest::Client::new();
    let mut form = Form::new();

    form = form.text("company_name", form_data.company_name);
    form = form.text("client_name", form_data.client_name);
    form = form.text("email", form_data.email);
    form = form.text("phone", form_data.phone);
    form = form.text("state", form_data.state);
    form = form.text("city", form_data.city);
    form = form.text("segment", form_data.segment);

    if let Some(file_path) = form_data.purchase_order {
        let path = PathBuf::from(&file_path);
        match File::open(&path).await {
            Ok(mut file) => {
                let part = get_file_part(&mut file, path).await?;
                form = form.part("purchase_order", part);
            }
            Err(e) => return Err(format!("Failed to open file: {}", e)),
        }
    }

    if let Some(file_paths) = form_data.invoice {
        for path_str in file_paths {
            let path = PathBuf::from(&path_str);
            match File::open(&path).await {
                Ok(mut file) => {
                    let part = get_file_part(&mut file, path).await?;
                    form = form.part("invoice", part);
                }
                Err(e) => return Err(format!("Failed to open file: {}", e)),
            }
        }
    }

    if let Some(file_path) = form_data.handing_over_report {
        let path = PathBuf::from(&file_path);
        match File::open(&path).await {
            Ok(mut file) => {
                let part = get_file_part(&mut file, path).await?;
                form = form.part("handing_over_report", part);
            }
            Err(e) => return Err(format!("Failed to open file: {}", e)),
        }
    }

    if let Some(file_paths) = form_data.pms_report {
        for path_str in file_paths {
            let path = PathBuf::from(&path_str);
            match File::open(&path).await {
                Ok(mut file) => {
                    let part = get_file_part(&mut file, path).await?;
                    form = form.part("pms_report", part);
                }
                Err(e) => return Err(format!("Failed to open file: {}", e)),
            }
        }
    }

    match client.post(endpoint).multipart(form).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let create_response = response
                    .json::<ClientResponse>()
                    .await
                    .map_err(|e| format!("Failed to parse JSON: {}", e))?;
                Ok(create_response)
            } else {
                Err(format!("API returned error status: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Failed to send request: {}", e)),
    }
}

#[tauri::command]
pub async fn update_client(client_data: UpdateClientRequest) -> Result<(), String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}", api_url, client_data.id);

    let client = reqwest::Client::new();
    let response = client
        .put(endpoint)
        .json(&client_data)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch client: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error fetching client: {} with status code {}",
            error_message, status_code
        ))
    }
}

#[tauri::command]
pub async fn delete_client(client_id: u32) -> Result<(), String> {
    let api_url = config::api_url();
    let endpoint = format!("{}/clients/{}", api_url, client_id);

    let client = reqwest::Client::new();
    let response = client
        .delete(endpoint)
        .send()
        .await
        .map_err(|e| format!("Failed to delete client: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status_code = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!(
            "Error deleting client: {} with status code {}",
            error_message, status_code
        ))
    }
}
