use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};

#[derive(serde::Serialize)]
pub struct Scenario {
    name: String,
}

pub fn get_route() -> Router {
    Router::new()
        .route("/scenario/", get(list_scenario))
        .route("/scenario/:name", get(inspect_scenario))
        .route("/scenario/:name", post(import_scenario))
        .route("/scenario/:name", delete(delete_scenario))
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

pub async fn inspect_scenario(Path(name): Path<String>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("name '{name}' is existed\n")))
        .unwrap()
}

pub async fn import_scenario(Path(name): Path<String>, body: String) -> impl IntoResponse {
    importer::handle_scenario(&name).await;
    println!("POST : scenario {name} is called.\n");
    println!("       Path is {body}.\n");
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("name '{name}' is existed\n")))
        .unwrap()
}

pub async fn delete_scenario(Path(name): Path<String>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("name '{name}' is existed\n")))
        .unwrap()
}
