use crate::models::settings::SettingUpsert;
use crate::services::settings_service;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::MySqlPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingPayload {
    code: String,
    description: String,
    value: Option<String>,
}

fn map_payload(payload: SettingPayload) -> Result<SettingUpsert, &'static str> {
    let code = payload.code.trim().to_string();
    let description = payload.description.trim().to_string();
    if code.is_empty() || description.is_empty() {
        return Err("Setting code and description are required");
    }
    Ok(SettingUpsert {
        code,
        description,
        value: payload.value.filter(|v| !v.is_empty()),
    })
}

pub async fn list_settings(State(pool): State<MySqlPool>) -> impl IntoResponse {
    match settings_service::list_settings(&pool).await {
        Ok(settings) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({
                "success": true,
                "data": settings,
                "count": settings.len()
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to fetch settings: {}", e)
            })),
        )
            .into_response(),
    }
}

pub async fn get_setting(
    State(pool): State<MySqlPool>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    match settings_service::get_setting(&pool, &code).await {
        Ok(setting) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({ "success": true, "data": setting })),
        )
            .into_response(),
        Err(e) if e.to_string().contains("not found") => (
            StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to fetch setting: {}", e)
            })),
        )
            .into_response(),
    }
}

pub async fn create_setting(
    State(pool): State<MySqlPool>,
    Json(payload): Json<SettingPayload>,
) -> impl IntoResponse {
    let payload = match map_payload(payload) {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({ "success": false, "error": e })),
            )
                .into_response();
        }
    };
    match settings_service::create_setting(&pool, payload).await {
        Ok(setting) => (
            StatusCode::CREATED,
            axum::Json(serde_json::json!({ "success": true, "data": setting })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to create setting: {}", e)
            })),
        )
            .into_response(),
    }
}

pub async fn update_setting(
    State(pool): State<MySqlPool>,
    Path(code): Path<String>,
    Json(payload): Json<SettingPayload>,
) -> impl IntoResponse {
    let payload = match map_payload(payload) {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({ "success": false, "error": e })),
            )
                .into_response();
        }
    };
    match settings_service::update_setting(&pool, &code, payload).await {
        Ok(setting) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({ "success": true, "data": setting })),
        )
            .into_response(),
        Err(e) if e.to_string().contains("not found") => (
            StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to update setting: {}", e)
            })),
        )
            .into_response(),
    }
}

pub async fn delete_setting(
    State(pool): State<MySqlPool>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    match settings_service::delete_setting(&pool, &code).await {
        Ok(()) => (
            StatusCode::OK,
            axum::Json(serde_json::json!({ "success": true })),
        )
            .into_response(),
        Err(e) if e.to_string().contains("not found") => (
            StatusCode::NOT_FOUND,
            axum::Json(serde_json::json!({ "success": false, "error": e.to_string() })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to delete setting: {}", e)
            })),
        )
            .into_response(),
    }
}
