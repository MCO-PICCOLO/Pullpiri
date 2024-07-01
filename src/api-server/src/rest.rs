use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use common::etcd;
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

pub async fn inspect_scenario(Path(name): Path<String>) -> impl IntoResponse {
    let result = exist_etcd(&name).await;

    if result {
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("name '{name}' is existed\n")))
            .unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("name '{name}' is not existed\n")))
            .unwrap();
    }
}

pub async fn make_scenario(Path(name): Path<String>) -> impl IntoResponse {
    let result = write_etcd(&name).await;

    if result.is_ok() {
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("name '{name}' is created\n")))
            .unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("name '{name}' is not created\n")))
            .unwrap();
    }
}

pub async fn delete_scenario(Path(name): Path<String>) -> impl IntoResponse {
    if !exist_etcd(&name).await {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("name '{name}' is not existed\n")))
            .unwrap();
    }
    let result = delete_etcd(&name).await;

    if result.is_ok() {
        return Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("name '{name}' is deleted\n")))
            .unwrap();
    } else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("name '{name}' is not deleted\n")))
            .unwrap();
    }
}

async fn exist_etcd(name: &str) -> bool {
    etcd::get(name).await.is_ok()
}

async fn write_etcd(name: &str) -> Result<(), etcd::Error> {
    let key = name;
    let value = "SAVED";

    etcd::put(key, value).await?;

    Ok(())
}

async fn delete_etcd(name: &str) -> Result<(), etcd::Error> {
    etcd::delete(name).await?;

    Ok(())
}
