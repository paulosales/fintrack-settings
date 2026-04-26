use crate::app_state::AppState;
use crate::controllers::settings_controller;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

/// Public read-only routes — no authentication required.
/// Used for internal service-to-service calls on the Docker network.
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/settings", get(settings_controller::list_settings))
        .route("/settings/{code}", get(settings_controller::get_setting))
}

/// Protected routes — require a valid Keycloak Bearer token.
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/settings", post(settings_controller::create_setting))
        .route("/settings/{code}", put(settings_controller::update_setting))
        .route(
            "/settings/{code}",
            delete(settings_controller::delete_setting),
        )
}
