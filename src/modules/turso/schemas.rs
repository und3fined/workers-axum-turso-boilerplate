// Copyright (c) 2024 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2024 Jun 11.

#![allow(dead_code, unused_variables)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,

    pub created_at: i64,
}
