// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

pub mod schemas;

use anyhow::{bail, Result};
use libsql::wasm::{CloudflareSender, Connection};

pub type Client = Connection<CloudflareSender>;

pub fn new(env: &worker::Env) -> Result<Client> {
    let url = match env.var("TURSO_DB_URL") {
        Ok(url) => url.to_string(),
        Err(_) => bail!("TURSO_DB_URL is not set"),
    };

    let auth_token = match env.var("TURSO_DB_AUTH_TOKEN") {
        Ok(token) => token.to_string(),
        Err(_) => bail!("TURSO_DB_AUTH_TOKEN missing"),
    };

    let client = Connection::open_cloudflare_worker(url, auth_token);
    Ok(client)
}
