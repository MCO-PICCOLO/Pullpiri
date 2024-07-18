use crate::metric::{Pod, PodInspect};
use hyper::{Client, Uri};
use hyperlocal::{UnixConnector, Uri as UnixUri};
use std::error::Error;

pub async fn get_pod_list() -> Result<Vec<Pod>, Box<dyn Error>> {
    let connector = UnixConnector;
    let client = Client::builder().build::<_, hyper::Body>(connector);

    let uri: Uri = UnixUri::new("/var/run/podman/podman.sock", "/v1.0.0/libpod/pods/json").into();

    let res = client.get(uri).await?;

    let body = hyper::body::to_bytes(res).await?;
    let pods: Vec<Pod> = serde_json::from_slice(&body)?;
    println!("{:#?}", pods);

    Ok(pods)
}

pub async fn get_pod_inspect(pod_id: &str) -> Result<PodInspect, Box<dyn Error>> {
    let connector = UnixConnector;
    let client = Client::builder().build::<_, hyper::Body>(connector);

    let uri: Uri = UnixUri::new(
        "/run/podman/podman.sock",
        &format!("/v1.0.0/libpod/pods/{}/json", pod_id),
    )
    .into();

    let res = client.get(uri).await?;

    let body = hyper::body::to_bytes(res).await?;
    let pod_inspect: PodInspect = serde_json::from_slice(&body)?;

    println!("Pod Response: {:#?}", pod_inspect);

    Ok(pod_inspect)
}
