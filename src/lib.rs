// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

mod modules;
mod server;

use axum::{body, http};
use tower_service::Service;
use worker::*;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<http::Response<body::Body>> {
    let mut axum_client = server::new(env).await.unwrap();
    Ok(axum_client.call(req).await?)
}
