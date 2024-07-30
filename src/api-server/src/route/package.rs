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
    println!("todo - inspect {name}");
    return_ok()
}

pub async fn import_package(Path(name): Path<String>, body: String) -> impl IntoResponse {
    let package = importer::handle_package(&body).await;

    println!("POST : package {name} is called.");
    println!("       Path is {body}.\n");

    /*if package.is_err() {
        return return_err();
    }*/

    if crate::manager::handle_package_msg(package.unwrap())
        .await
        .is_err()
    {
        println!("error: writing scenario in etcd");
        return return_err();
    }

    return_ok()
}

pub async fn delete_package(Path(name): Path<String>) -> impl IntoResponse {
    println!("todo - delete {name}");
    return_ok()
}

fn return_ok() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Ok".to_string()))
        .unwrap()
}

fn return_err() -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("Error".to_string()))
        .unwrap()
}
