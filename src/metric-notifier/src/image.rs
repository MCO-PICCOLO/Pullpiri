use hyper::{Client, Uri};
use hyperlocal::{UnixConnector, Uri as UnixUri};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Image {
    id: String,
    repo_tags: Option<Vec<String>>,
}

pub async fn get_image_list() -> Result<Vec<String>, Box<dyn Error>> {
    let connector = UnixConnector;
    let client = Client::builder().build::<_, hyper::Body>(connector);

    let uri: Uri = UnixUri::new("/var/run/podman/podman.sock", "/v1.0.0/libpod/images/json").into();

    let res = client.get(uri).await?;

    let body = hyper::body::to_bytes(res).await?;
    let images: Vec<Image> = serde_json::from_slice(&body)?;

    let image_list: Vec<String> = images
        .into_iter()
        .filter_map(|image| image.repo_tags)
        .flatten()
        .collect();

    Ok(image_list)
}
