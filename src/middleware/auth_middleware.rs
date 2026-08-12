use anyhow::Context;
use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::app_state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub preferred_username: Option<String>,
}

#[derive(Deserialize)]
struct RsaJwk {
    kid: Option<String>,
    kty: String,
    n: String,
    e: String,
}

#[derive(Deserialize)]
struct JwksResponse {
    keys: Vec<RsaJwk>,
}

async fn refresh_jwks(state: &AppState) -> anyhow::Result<()> {
    let certs_url = format!("{}/protocol/openid-connect/certs", state.keycloak_realm_url);
    let jwks: JwksResponse = state
        .http_client
        .get(&certs_url)
        .send()
        .await
        .context("JWKS request failed")?
        .error_for_status()
        .context("JWKS endpoint returned an error status")?
        .json()
        .await
        .context("Failed to parse JWKS JSON")?;

    let mut cache = state.jwks_cache.write().await;
    for key in jwks.keys {
        if key.kty != "RSA" {
            continue;
        }
        if let Some(kid) = key.kid {
            match DecodingKey::from_rsa_components(&key.n, &key.e) {
                Ok(decoding_key) => {
                    cache.insert(kid, decoding_key);
                }
                Err(e) => {
                    eprintln!("Warning: skipping JWKS key '{kid}': {e}");
                }
            }
        }
    }
    Ok(())
}

fn try_validate(
    token: &str,
    kid: &str,
    cache: &HashMap<String, DecodingKey>,
) -> Option<Result<JwtClaims, jsonwebtoken::errors::Error>> {
    let key = cache.get(kid)?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.validate_aud = false;
    Some(decode::<JwtClaims>(token, key, &validation).map(|d| d.claims))
}

pub async fn validate_bearer_token(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let kid = decode_header(token)
        .ok()
        .and_then(|h| h.kid)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Try cached key first
    {
        let cache = state.jwks_cache.read().await;
        if let Some(result) = try_validate(token, &kid, &cache) {
            return match result {
                Ok(_) => Ok(next.run(request).await),
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            };
        }
    }

    // Cache miss — refresh and retry once
    if let Err(e) = refresh_jwks(&state).await {
        eprintln!("Failed to refresh JWKS: {e}");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let cache = state.jwks_cache.read().await;
    match try_validate(token, &kid, &cache) {
        Some(Ok(_)) => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
