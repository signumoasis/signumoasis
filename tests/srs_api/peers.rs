use anyhow::Context;
use signum_node_rs::srs_protocol::api::request_models::GetInfoRequestModel;

use crate::helpers::spawn_app;

#[tokio::test]
async fn srs_api_handler_returns_valid_data_for_get_info_request() -> Result<(), anyhow::Error> {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "requestType": "getInfo",
        "application": "SignumRust",
        "version": "0.1.0",
        "platform": "Test",
        "shareAddress": false,
        "networkName": "Signum-TEST",
    });

    // Act
    let response = client
        .post(&format!("{}/", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_ne!(Some(0), response.content_length());
    let json = response
        .json::<GetInfoRequestModel>()
        .await
        .context("couldn't deserialize json")?;

    let template = GetInfoRequestModel {
        announced_address: Some("http://localhost".to_string().to_string()),
        application: Some("SignumRust".to_string()),
        version: Some("0.1.0".to_string()),
        platform: Some("Test".to_string()),
        share_address: Some(true),
        network_name: "TEST".to_string(),
    };
    assert_eq!(json, template);
    Ok(())
}

#[tokio::test]
#[ignore]
async fn srs_api_handler_returns_valid_data_for_add_peers_request() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "requestType": "getInfo",
    });
    // Act
    let response = client
        .post(&format!("{}/", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    //TODO: Test adding new peers to database

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
#[ignore]
async fn srs_api_handler_returns_valid_data_for_get_peers_request() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "requestType": "getPeers",
    });

    // Act
    let response = client
        .post(&format!("{}/", &app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    //TODO: Add a peer to the database in spawn_app, test against it here

    // Assert
    assert!(response.status().is_success());
    assert_ne!(Some(0), response.content_length());
}
