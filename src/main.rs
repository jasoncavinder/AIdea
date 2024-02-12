/*
 * AIdea
 * Copyright (C) 2024 Jason Cavinder
 *
 * This file is part of AIdea, which is dual-licensed. You can use, modify,
 * and distribute this file under either the GNU Affero General Public License v3.0
 * (AGPLv3) or the AIdea Commercial License, as per your licensing agreement.
 */

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
