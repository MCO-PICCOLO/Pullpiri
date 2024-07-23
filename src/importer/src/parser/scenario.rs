use std::fs::File;
use std::io::prelude::*;

pub struct Scenario {
    pub name: String,
    pub conditions: String,
    pub actions: String,
    pub targets: String,
    pub scene: String,
}

pub fn scenario_parse(path: &str) -> Result<Scenario, Box<dyn std::error::Error>> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let scene: common::spec::scenario::Scenario = serde_yaml::from_str(&contents)?;

    let name: String = scene.get_name();
    let conditions: &Option<common::spec::scenario::Condition> = &scene.get_conditions();
    let actions: &common::spec::scenario::Action = &scene.get_actions()[0];
    let targets: &common::spec::scenario::Target = &scene.get_targets()[0];

    Ok(Scenario {
        name,
        conditions: serde_yaml::to_string(conditions)?,
        actions: serde_yaml::to_string(actions)?,
        targets: serde_yaml::to_string(targets)?,
        scene: serde_yaml::to_string(&scene)?,
    })
}
