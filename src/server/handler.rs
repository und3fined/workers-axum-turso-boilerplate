// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

use axum::{
    extract::State,
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use serde_json::{json, Value};
use worker::console_error;

use crate::modules::turso;

use super::state;

pub async fn root() -> &'static str {
    "Hello, there. I'm Workers Axum Turso Boilerplate!"
}

#[worker::send]
pub async fn health_check(State(state): State<state::AxumStateWrapper>) -> impl IntoResponse {
    let db: &turso::Client = state.turso.as_ref();

    match db.query("SELECT 1;", ()).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "OK" }))),
        Err(e) => {
            console_error!("Health check failed. Err: {}", e.to_string());
            (StatusCode::SERVICE_UNAVAILABLE, Json(json!({ "status": "Failed"})))
        }
    }
}

pub async fn fallback(method: Method, uri: Uri) -> impl IntoResponse {
    let err = format!("{} {} {}", method, uri.path().to_string(), StatusCode::NOT_FOUND);

    (StatusCode::NOT_FOUND, err)
}

pub async fn api_root() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "Welcome to Workers Axum Turso Boilerplate API!" })),
    )
}

pub async fn api_fallback(uri: Uri) -> (StatusCode, Json<Value>) {
    let err_json = json!({
      "error": {
        "code": "API_NOT_FOUND",
        "message": StatusCode::NOT_FOUND.to_string(),
        "uri": uri.path().to_string()
      }
    });

    (StatusCode::NOT_FOUND, Json(err_json))
}

pub async fn api_ping() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "message": "pong" })))
}
