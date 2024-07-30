/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::method_bluechi::send_dbus;
use common::etcd;
use common::statemanager::connection_server::Connection;
use common::statemanager::{SendRequest, SendResponse};
use std::error::Error;
use std::{thread, time::Duration};

const SYSTEMD_PATH: &str = "/etc/containers/systemd/";

#[derive(Default)]
pub struct StateManagerGrpcServer {}

#[tonic::async_trait]
impl Connection for StateManagerGrpcServer {
    async fn send(
        &self,
        request: tonic::Request<SendRequest>,
    ) -> Result<tonic::Response<SendResponse>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let req = request.into_inner();
        let from = req.from;
        let command = req.request;
        println!("{}/{}", from, command);

        if from == i32::from(common::constants::PiccoloModuleName::Gateway) {
            match make_action_for_scenario(&command).await {
                Ok(v) => Ok(tonic::Response::new(SendResponse { response: v })),
                Err(e) => {
                    println!("{:?}", e);
                    Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string()))
                }
            }
        } else {
            Err(tonic::Status::new(
                tonic::Code::Unavailable,
                "unsupported 'from' module",
            ))
        }
    }
}

pub async fn make_action_for_scenario(key: &str) -> Result<String, Box<dyn Error>> {
    let key_action = format!("{key}/actions");
    let key_target = format!("{key}/targets");
    let value_action = etcd::get(&key_action).await?;
    let value_target = etcd::get(&key_target).await?;
    let action: common::spec::scenario::Action = serde_yaml::from_str(&value_action)?;
    let target: common::spec::scenario::Target = serde_yaml::from_str(&value_target)?;

    let target_name = target.get_name();
    let key_model = format!("package/{}/models", &target_name);
    let value_model = etcd::get(&key_model).await?;
    let model: common::spec::package::model::Model = serde_yaml::from_str(&value_model)?;

    let model_name = model.get_name();
    let operation = &*action.get_operation();

    println!("model name : {}\noperation : {}\n", model_name, operation);

    match operation {
        "launch" => {
            // make symlink & reload
            make_symlink_and_reload(&model_name, &target_name).await?;
            // start service
            try_service(&model_name, "START").await?;
        }
        "terminate" => {
            // stop service
            try_service(&model_name, "STOP").await?;
            thread::sleep(Duration::from_secs(5));
            // delete symlink & reload
            delete_symlink_and_reload(&model_name).await?;
        }
        "update" | "rollback" => {
            // delete symlink & reload
            delete_symlink_and_reload(&model_name).await?;
            // make symlink & reload
            make_symlink_and_reload(&model_name, &target_name).await?;
            // restart service
            try_service(&model_name, "RESTART").await?;
        }
        "download" => {
            println!("do something");
        }
        _ => {
            return Err("not supported operation".into());
        }
    }

    Ok(format!("Done : {}\n", operation))
}

async fn delete_symlink_and_reload(model_name: &str) -> Result<(), Box<dyn Error>> {
    let kube_symlink_path = format!("{}{}.kube", SYSTEMD_PATH, model_name);
    let _ = std::fs::remove_file(kube_symlink_path);

    send_dbus(vec!["DAEMON_RELOAD"]).await?;
    thread::sleep(Duration::from_millis(100));

    Ok(())
}

async fn make_symlink_and_reload(
    model_name: &str,
    target_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let original = format!(
        "{0}/packages/{1}/{2}/{2}.kube",
        common::get_conf("YAML_STORAGE"),
        target_name,
        model_name
    );
    let link = format!("{}{}.kube", SYSTEMD_PATH, model_name);
    std::os::unix::fs::symlink(original, link)?;

    send_dbus(vec!["DAEMON_RELOAD"]).await?;
    thread::sleep(Duration::from_millis(100));

    Ok(())
}

async fn try_service(model_name: &str, act: &str) -> Result<(), Box<dyn std::error::Error>> {
    send_dbus(vec![
        act,
        &common::get_conf("HOST_NODE"),
        &format!("{}.service", model_name),
    ])
    .await?;
    thread::sleep(Duration::from_millis(100));

    Ok(())
}
