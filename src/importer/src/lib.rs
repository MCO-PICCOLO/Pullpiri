/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::{error::Error, path::PathBuf};

mod downloader;
mod parser;

pub async fn handle_package(name: &str) {
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url: String = format!("{}/packages/{}.tar", base_url, name);
    // TODO
    // 1. download tar file (/root/piccolo_yaml/ ~~.tar)
    // 2. decompress tar file
    // 3. parsing - model,
    // ***** make pod.yaml .kube
    // 4. send result (name, model, network, volume)
}

pub async fn handle_scenario(name: &str) {
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url = format!("{}/scenarios/{}.yaml", base_url, name);

    //TODO name 명을 meta data의 name으로 해야 할 것 같은데 ? parsing후에 save하자.

    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/scenarios/{}.yaml", save_path, name);

    // TODO
    // 1. download yaml file : reqwest crate
    // 2. /root/piccolo_yaml/    scenarios/     yaml
    _ = downloader::download(&full_url, &full_save_path);
    let temp_path = PathBuf::from(full_save_path);

    // 3. parsing - action, condition, target
    let scenario: Result<common::apiserver::scenario::Scenario, Box<dyn Error>> =
        parser::scenario::scenario_parse(&temp_path);

    // 4. send result (name, action, condition, target, full)
    // grpc send
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn parsing_update_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo/examples/version-display/scenario/update-scenario.yaml",
        );

        let result = crate::parser::scenario_parse(&path);
        println!("{:#?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn parsing_rollback_scenario() {
        let path = std::path::PathBuf::from(
            "/root/work/projects-rust/piccolo/examples/version-display/scenario/rollback-scenario.yaml",
        );

        let result = crate::parser::scenario_parse(&path);
        println!("{:#?}", result);
        assert!(result.is_ok());
    }
}
*/

/*
use crate::file_handler;
use crate::grpc::sender::apiserver;
use crate::parser;
use common::apiserver::scenario::Scenario;
use common::yamlparser::connection_server::Connection;
use common::yamlparser::{SendRequest, SendResponse};
use tonic::{Code, Request, Response, Status};

#[derive(Default)]
pub struct YamlparserGrpcServer {}

#[tonic::async_trait]
impl Connection for YamlparserGrpcServer {
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
        let req = request.into_inner();
        let scenario = match handle_msg(&req.request) {
            Ok(scenario) => scenario,
            Err(e) => return Err(Status::new(Code::InvalidArgument, e.to_string())),
        };

        match apiserver::send_msg_to_apiserver(scenario).await {
            Ok(response) => Ok(tonic::Response::new(SendResponse {
                response: response.into_inner().resp,
            })),
            Err(e) => Err(Status::new(Code::Unavailable, e.to_string())),
        }
    }
}

pub fn handle_msg(path: &str) -> Result<Scenario, Box<dyn std::error::Error>> {
    let absolute_path = file_handler::get_absolute_file_path(path)?;
    let scenario = parser::scenario_parse(&absolute_path)?;
    Ok(scenario)
}
*/
