/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub mod package;
pub mod scenario;
/*
 use crate::old_file_handler;
 use common::apiserver::scenario::Scenario;
 use std::fs::File;
 use std::io::prelude::*;
 use std::path::PathBuf;

 pub fn scenario_parse(path: &PathBuf) -> Result<Scenario, Box<dyn std::error::Error>> {
     let mut f = File::open(path)?;
     let mut contents = String::new();
     f.read_to_string(&mut contents)?;

     let scene: common::spec::scenario::Scenario = serde_yaml::from_str(&contents)?;

     let name = scene.get_name();
     let conditions = &scene.get_conditions();
     let actions = &scene.get_actions()[0];

     old_file_handler::perform(&name, actions)?;

     Ok(Scenario {
         name: scene.get_name(),
         conditions: serde_yaml::to_string(conditions)?,
         actions: serde_yaml::to_string(actions)?,
     })
 }
*/
