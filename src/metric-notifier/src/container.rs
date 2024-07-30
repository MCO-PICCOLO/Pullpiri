use crate::metric::{Container, ContainerInspect};
use hyper::{Client, Uri};
use hyperlocal::{UnixConnector, Uri as UnixUri};
use std::error::Error;

pub async fn get_container_list() -> Result<Vec<Container>, Box<dyn Error>> {
    let connector = UnixConnector;
    let client = Client::builder().build::<_, hyper::Body>(connector);

    let uri: Uri = UnixUri::new(
        "/var/run/podman/podman.sock",
        "/v1.0.0/libpod/containers/json",
    )
    .into();

    let res = client.get(uri).await?;

    let body = hyper::body::to_bytes(res).await?;
    let containers: Vec<Container> = serde_json::from_slice(&body)?;
    println!("{:#?}", containers);

    Ok(containers)
}

pub async fn get_container_inspect(container_id: &str) -> Result<ContainerInspect, Box<dyn Error>> {
    let connector = UnixConnector;
    let client = Client::builder().build::<_, hyper::Body>(connector);

    let uri: Uri = UnixUri::new(
        "/var/run/podman/podman.sock",
        &format!("/v1.0.0/libpod/containers/{}/json", container_id),
    )
    .into();

    let res = client.get(uri).await?;

    let body = hyper::body::to_bytes(res).await?;
    let container_inspect: ContainerInspect = serde_json::from_slice(&body)?;
    println!("{:#?}", container_inspect);

    Ok(container_inspect)
}
