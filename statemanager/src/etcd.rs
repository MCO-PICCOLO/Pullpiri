use etcd_client::{Client, Error};

async fn get_client() -> Result<Client, Error> {
    Client::connect([common::DEFAULT_ETCD_ENDPOINT], None).await
}

pub async fn put(key: &str, value: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    client.put(key, value, None).await?;
    Ok(())
}

pub async fn get(key: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    client.get(key, None).await?;
    Ok(())
}

pub async fn delete(key: &str) -> Result<(), Error> {
    let mut client = get_client().await?;
    client.delete(key, None).await?;
    Ok(())
}
