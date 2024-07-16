use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};

#[derive(serde::Serialize)]
pub struct Package {
    name: String,
}

pub fn get_route() -> Router {
    Router::new()
        .route("/package/", get(list_package))
        .route("/package/:name", get(inspect_package))
        .route("/package/:name", post(import_package))
        .route("/package/:name", delete(delete_package))
}

pub async fn list_package() -> Json<Vec<Package>> {
    let packages = vec![
        Package {
            name: "version".to_string(),
        },
        Package {
            name: "display".to_string(),
        },
    ];
    Json(packages)
}

pub async fn inspect_package(Path(name): Path<String>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("name '{name}' is existed\n")))
        .unwrap()
}

pub async fn import_package(Path(name): Path<String>, body: String) -> impl IntoResponse {
    importer::handle_package(&name).await;
    println!("POST : package {name} is called.");
    println!("       Path is {body}.\n");
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("name '{name}' is existed\n")))
        .unwrap()
}

pub async fn delete_package(Path(name): Path<String>) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!("name '{name}' is existed\n")))
        .unwrap()
}
