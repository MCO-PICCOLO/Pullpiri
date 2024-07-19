pub mod container;
pub mod image;
pub mod metric;
pub mod pod;

use container::{get_container_inspect, get_container_list};
use pod::{get_pod_inspect, get_pod_list};
//use common::apiserver::metric_notifier_client::MetricNotifierClient;
use common::apiserver::metric_notifier::{ContainerInfo, PodInfo, PodInfoContainer};
use futures::future::try_join_all;
use std::collections::HashMap;
use std::error::Error;

// use crate::metric::{PodContainer};

async fn pods_inspect() -> Result<Vec<PodInfo>, Box<dyn Error>> {
    let pod_inspect_list = get_pod_list().await?;

    let pod_inspect_infos: Vec<PodInfo> = try_join_all(pod_inspect_list.iter().map(|pod| {
        let pod_id = pod.id.clone();
        async move {
            let inspect_info = get_pod_inspect(&pod_id).await?;

            let containers_map: Vec<PodInfoContainer> = inspect_info
                .containers
                .into_iter()
                .map(|container| {
                    // println!("Pod Response: {:?}", container);
                    PodInfoContainer {
                        id: container.id,
                        name: container.name,
                        state: container.state,
                    }
                })
                .collect();

            Ok::<PodInfo, Box<dyn Error>>(PodInfo {
                id: inspect_info.id,
                name: inspect_info.name,
                state: inspect_info.state,
                host_name: inspect_info.hostname,
                created: inspect_info.created,
                containers: containers_map,
            })
        }
    }))
    .await?
    .into_iter()
    .collect();

    Ok(pod_inspect_infos)
}

async fn containers_inspect() -> Result<Vec<ContainerInfo>, Box<dyn Error>> {
    let container_list = get_container_list().await?;
    let container_infos: Vec<ContainerInfo> =
        try_join_all(container_list.iter().map(|container| {
            let container_id = container.id.clone();
            async move {
                let inspect_info = get_container_inspect(&container_id).await?;
                let mut state_map = HashMap::new();
                state_map.insert("Status".to_string(), inspect_info.state.status);
                state_map.insert(
                    "Running".to_string(),
                    inspect_info.state.running.to_string(),
                );
                state_map.insert("Paused".to_string(), inspect_info.state.paused.to_string());
                state_map.insert(
                    "Restarting".to_string(),
                    inspect_info.state.restarting.to_string(),
                );
                state_map.insert(
                    "OOMKilled".to_string(),
                    inspect_info.state.oom_killed.to_string(),
                );
                state_map.insert("Dead".to_string(), inspect_info.state.dead.to_string());
                state_map.insert("Pid".to_string(), inspect_info.state.pid.to_string());
                state_map.insert(
                    "ExitCode".to_string(),
                    inspect_info.state.exit_code.to_string(),
                );
                state_map.insert("Error".to_string(), inspect_info.state.error);
                state_map.insert("StartedAt".to_string(), inspect_info.state.started_at);
                state_map.insert("FinishedAt".to_string(), inspect_info.state.finished_at);

                let mut config_map = HashMap::new();
                config_map.insert("Hostname".to_string(), inspect_info.config.hostname);
                config_map.insert("Domainname".to_string(), inspect_info.config.domainname);
                config_map.insert("User".to_string(), inspect_info.config.user);
                config_map.insert(
                    "AttachStdin".to_string(),
                    inspect_info.config.attach_stdin.to_string(),
                );
                config_map.insert(
                    "AttachStdout".to_string(),
                    inspect_info.config.attach_stdout.to_string(),
                );
                config_map.insert(
                    "AttachStderr".to_string(),
                    inspect_info.config.attach_stderr.to_string(),
                );
                config_map.insert("tty".to_string(), inspect_info.config.tty.to_string());
                config_map.insert(
                    "OpenStdin".to_string(),
                    inspect_info.config.open_stdin.to_string(),
                );
                config_map.insert(
                    "StdinOnce".to_string(),
                    inspect_info.config.stdin_once.to_string(),
                );
                config_map.insert("Image".to_string(), inspect_info.config.image.clone());
                config_map.insert("WorkingDir".to_string(), inspect_info.config.working_dir);

                Ok::<ContainerInfo, Box<dyn Error>>(ContainerInfo {
                    id: inspect_info.id,
                    names: vec![inspect_info.name],
                    image: inspect_info.config.image.clone(),
                    state: state_map,
                    config: config_map,
                })
            }
        }))
        .await?
        .into_iter()
        .collect();

    Ok(container_infos)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pods: Vec<PodInfo> = pods_inspect().await?;
    println!("pod inspect info: {:#?}", pods);

    let containers: Vec<ContainerInfo> = containers_inspect().await?;
    println!("container inspect info: {:#?}", containers);

    Ok(())
}
