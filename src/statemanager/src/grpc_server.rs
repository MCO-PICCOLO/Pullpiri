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
                Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
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
    let key_action = format!("{key}/action");
    let key_target = format!("{key}/target");
    let value_action = etcd::get(&key_action).await?;
    let value_target = etcd::get(&key_target).await?;
    let action: common::spec::scenario::Action = serde_yaml::from_str(&value_action)?;
    let target: common::spec::scenario::Target = serde_yaml::from_str(&value_target)?;

    let key_package = format!("package/{}", target.get_name());
    let value_package = etcd::get(&key_package).await?;
    let package: common::spec::package::Package = serde_yaml::from_str(&value_package)?;

    let mut list_model = package.get_model_name();
    // TODO : fix it for multiple models (pods)
    let key_model = format!("model/{}", list_model.pop().unwrap());
    let value_model = etcd::get(&key_model).await?;
    let model: common::spec::package::model::Model = serde_yaml::from_str(&value_model)?;

    let name = model.get_name();
    let operation = &*action.get_operation();

    println!("name : {}\noperation : {}\n", name, operation);

    match operation {
        "launch" => {
            make_and_start_new_symlink(&name).await?;
        }
        "terminate" => {
            delete_symlink_and_reload(&name).await?;
        }
        "update" | "rollback" => {
            delete_symlink_and_reload(&name).await?;
            make_and_start_new_symlink(&name).await?;
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

async fn delete_symlink_and_reload(name: &str) -> Result<(), Box<dyn Error>> {
    let _ = send_dbus(vec![
        "STOP",
        &common::get_conf("HOST_NODE"),
        &format!("{}.service", name),
    ])
    .await;
    thread::sleep(Duration::from_millis(100));
    let kube_symlink_path = format!("{}{}.kube", SYSTEMD_PATH, name);
    let _ = std::fs::remove_file(kube_symlink_path);
    send_dbus(vec!["DAEMON_RELOAD"]).await?;
    thread::sleep(Duration::from_millis(100));
    Ok(())
}

async fn make_and_start_new_symlink(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let original = format!("{0}{1}/{1}.kube", common::get_conf("YAML_STORAGE"), name,);
    let link = format!("{}{}.kube", SYSTEMD_PATH, name);
    std::os::unix::fs::symlink(original, link)?;

    send_dbus(vec!["DAEMON_RELOAD"]).await?;
    thread::sleep(Duration::from_millis(100));
    send_dbus(vec![
        "START",
        &common::get_conf("HOST_NODE"),
        &format!("{}.service", name),
    ])
    .await?;
    thread::sleep(Duration::from_millis(100));

    Ok(())
}
