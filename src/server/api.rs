// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

use axum::{
    routing::{any, get},
    Router,
};

use super::{handler, state};

pub fn routes() -> Router<state::AxumStateWrapper> {
    let api_routes = Router::new()
        .route("/", any(handler::api_root))
        .route("/ping", get(handler::api_ping))
        .fallback(handler::api_fallback);

    api_routes
}
