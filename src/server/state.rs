// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

use std::sync::Arc;

use worker::send;

use crate::modules::turso;

#[derive(Clone)]
pub struct AxumState {
    pub env: Arc<worker::Env>,
    pub turso: Arc<turso::Client>,
}

unsafe impl Send for AxumState {}
unsafe impl Sync for AxumState {}

impl AxumState {
    pub fn new(env: worker::Env, turso_client: turso::Client) -> Self {
        Self {
            env: Arc::new(env),
            turso: Arc::new(turso_client),
        }
    }
}

pub fn new_state(env: worker::Env, turso_client: turso::Client) -> AxumState {
    AxumState::new(env, turso_client)
}

pub type AxumStateWrapper = send::SendWrapper<AxumState>;
