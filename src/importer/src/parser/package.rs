use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use common::spec::package;
use crate::file_handler;

pub struct Package {
    pub name: String,
    pub models: String,
    pub network: String,
    pub volume: String,
}

pub fn package_parse(path: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let package_yaml = format!("{}/package.yaml", path);
    let mut f = File::open(package_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    //name
    let package: package::Package = serde_yaml::from_str(&contents)?;
    let name = package.get_name();
    let model_name = package.get_model_name();

    //models
    let package_model = package.get_models();
    let model: Result<package::model::Model, Box<dyn Error>> = model_parse(path, &model_name[0]);

    //networks
    let network_yaml = &package_model[0].get_resources().get_network();
    let networks = network_parse(path, &network_yaml);

    //volumes
    let volume_yaml = &package_model[0].get_resources().get_volume();
    let volumes = volume_parse(path, &volume_yaml)?;
    let volume_copy = volumes.clone();

    //merge data
    let model_copy = model.unwrap().clone();
    model_copy.get_podspec().set_volumes(volume_copy.get_volume().clone());
    let models = serde_yaml::to_string(&model_copy)?;

    //perform_file
    _= file_handler::perform(path, &models);

    Ok(Package {
        name,
        models,
        network: serde_yaml::to_string(&networks.unwrap())?,
        volume: serde_yaml::to_string(&volumes)?,
    })

}

fn model_parse(path: &str, name: &str) -> Result<package::model::Model, Box<dyn std::error::Error>> {
    let model_yaml: String = format!("{}/models/{}.yaml", path, name);
    let mut f = File::open(model_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let model: package::model::Model = serde_yaml::from_str(&contents)?;
    Ok(model)
}

fn network_parse(path: &str, name: &str) -> Result<package::network::Network, Box<dyn std::error::Error>> {
    let network_yaml = format!("{}/networks/{}.yaml", path, name);
    let mut f = File::open(network_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let network: package::network::Network = serde_yaml::from_str(&contents)?;
    Ok(network)
}

fn volume_parse(path: &str, name: &str) -> Result<package::volume::VolumeSpec, Box<dyn std::error::Error>> {
    let volume_yaml = format!("{}/volumes/{}.yaml", path, name);
    let mut f = File::open(volume_yaml)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let volume: package::volume::Volume = serde_yaml::from_str(&contents)?;
    let volume_spec = volume.get_spec().clone().unwrap();
    Ok(volume_spec)
}
