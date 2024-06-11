// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

mod api;
mod handler;
pub mod state;

use anyhow::Result;
use axum::{routing::get, Router};
use worker::send;

use crate::modules::turso;

pub async fn new(env: worker::Env) -> Result<Router> {
    let turso_client = turso::new(&env)?;
    let axum_state = send::SendWrapper::new(state::new_state(env, turso_client));

    let app = Router::new()
        .route("/", get(handler::root))
        .route("/health-check", get(handler::health_check))
        .fallback(handler::fallback)
        // Mount Nesting/Subroute API routes
        .nest("/api", api::routes())
        .with_state(axum_state);

    Ok(app)
}
