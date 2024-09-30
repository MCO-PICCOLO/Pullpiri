use crate::file_handler;
use common::spec::package;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub models: String,
    pub network: String,
    pub volume: String,
}

pub fn parse(path: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let package_yaml = format!("{}/package.yaml", path);
    let mut f = File::open(package_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let package: package::Package = serde_yaml::from_str(&contents)?;
    let name: String = package.get_name();
    let model_name = package.get_model_name();

    let package_model = package.get_models();
    let mut model: package::model::Model = model_parse(path, &model_name[0]).unwrap();

    let network_yaml = &package_model[0].get_resources().get_network();
    let networks = network_parse(path, network_yaml);

    let volume_yaml = &package_model[0].get_resources().get_volume();
    let volumes = &volume_parse(path, volume_yaml);

    if let Some(volume_spec) = volumes {
        model.spec.volumes.clone_from(volume_spec.get_volume());
    }

    let models = serde_yaml::to_string(&model)?;

    //make kube,yaml file
    file_handler::perform(&model_name[0], &models, &name)?;

    Ok(Package {
        name,
        models,
        network: serde_yaml::to_string(&networks)?,
        volume: serde_yaml::to_string(&volumes)?,
    })
}

fn parse_yaml<T>(path: &str, name: &str, subdir: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let yaml_path = format!("{}/{}/{}.yaml", path, subdir, name);
    let mut f = File::open(yaml_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let parsed: T = serde_yaml::from_str(&contents)?;
    Ok(parsed)
}

fn model_parse(path: &str, name: &str) -> Result<package::model::Model, Box<dyn Error>> {
    parse_yaml(path, name, "models")
}

fn network_parse(path: &str, name: &str) -> Option<package::network::NetworkSpec> {
    let network: package::network::Network = parse_yaml(path, name, "networks").unwrap();
    network.get_spec().clone()
}

fn volume_parse(path: &str, name: &str) -> Option<package::volume::VolumeSpec> {
    if let Ok(volume) = parse_yaml::<package::volume::Volume>(path, name, "volumes") {
        volume.get_spec().clone()
    } else {
        None
    }
}
