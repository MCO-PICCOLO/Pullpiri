/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */
mod downloader;
mod parser;
mod file_handler;
mod decompress;
use std::error::Error;

pub async fn handle_package(name: &str) -> Result<parser::package::Package, Box<dyn std::error::Error>>{
//url path
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url: String = format!("{}/packages/{}.tar", base_url, name);

//save path
    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/scenarios/{}.tar", save_path, name);

//download, decompress    
    let _= downloader::download(&full_url, &full_save_path);
    let _= decompress::decompress(&full_save_path);

//parsing
    let parsing_path = format!("{}/scenarios/{}",save_path, name);
    let package: Result<parser::package::Package, Box<dyn Error>> = parser::package::package_parse(&parsing_path);

    Ok(package?)
}

pub async fn handle_scenario(name: &str) -> Result<parser::scenario::Scenario, Box<dyn std::error::Error>> {
    let base_url = common::get_conf("DOC_REGISTRY");
    let full_url = format!("{}/scenarios/{}.yaml", base_url, name);

    let save_path: String = common::get_conf("YAML_STORAGE");
    let full_save_path = format!("{}/scenarios/{}.yaml", save_path, name);

    let _= downloader::download(&full_url, &full_save_path);

    let scenario: Result<parser::scenario::Scenario, Box<dyn Error>> = parser::scenario::scenario_parse(&full_save_path);

    Ok(scenario?)
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
