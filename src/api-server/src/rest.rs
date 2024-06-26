use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Scenario {
    name: String,
}

pub async fn list_scenario() -> Json<Vec<Scenario>> {
    let scenarios = vec![
        Scenario {
            name: "version".to_string(),
        },
        Scenario {
            name: "display".to_string(),
        },
    ];
    Json(scenarios)
}

pub async fn inspect_scenario() -> String {
    "Hello, Rust!\n".to_string()
}

pub async fn make_scenario() {
    
}