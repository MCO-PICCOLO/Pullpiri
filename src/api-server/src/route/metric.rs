use axum::{routing::get, Json, Router};

use crate::grpc::receiver::metric_notifier::{NewContainerList, NewImageList, NewPodList};

pub fn get_route() -> Router {
    Router::new()
        .route("/metric/image", get(list_image))
        .route("/metric/container", get(list_container))
        .route("/metric/pod", get(list_pod))
}

pub async fn list_image() -> Json<NewImageList> {
    let s = common::etcd::get("metric/image").await.unwrap_or_default();
    let image: NewImageList = serde_json::from_str(&s).unwrap_or_default();
    Json(image)
}

pub async fn list_container() -> Json<NewContainerList> {
    let s = common::etcd::get("metric/container")
        .await
        .unwrap_or_default();
    let container: NewContainerList = serde_json::from_str(&s).unwrap_or_default();
    Json(container)
}
pub async fn list_pod() -> Json<NewPodList> {
    let s = common::etcd::get("metric/pod").await.unwrap_or_default();
    let pod: NewPodList = serde_json::from_str(&s).unwrap_or_default();
    Json(pod)
}