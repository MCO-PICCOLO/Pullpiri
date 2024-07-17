pub mod container;
pub mod image;
pub mod metric;
pub mod pod;

use container::{get_container_list, get_container_inspect};
use pod::{ get_pod_list, get_pod_inspect};
//use common::apiserver::metric_notifier_client::MetricNotifierClient;
use common::apiserver::metric_notifier::{ContainerInfo, PodInfo, PodInfoContainer};
use std::collections::HashMap;
use std::error::Error;
use futures::future::try_join_all;

// use crate::metric::{PodContainer};

async fn pods_inspect() -> Result<Vec<PodInfo>, Box<dyn Error>>{
    let pod_inspect_list = get_pod_list().await?; 

    let pod_inspect_infos: Vec<PodInfo> = try_join_all(
        pod_inspect_list.iter().map(|pod| {
            let pod_id = pod.Id.clone();
            async move {
                let inspect_info = get_pod_inspect(&pod_id).await?;
                
                let containers_map: Vec<PodInfoContainer> = inspect_info.Containers.into_iter().map(|container| {
                    // println!("Pod Response: {:?}", container);
                    PodInfoContainer {
                        id: container.Id,
                        name: container.Name,
                        state: container.State,
                    }
                }).collect();

                Ok::<PodInfo, Box<dyn Error>>(PodInfo {
                    id: inspect_info.Id,
                    name: inspect_info.Name,
                    state: inspect_info.State,           
                    host_name: inspect_info.Hostname,
                    created: inspect_info.Created,                    
                    containers : containers_map,

                })
            }
        })
    ).await?.into_iter().collect();

    Ok(pod_inspect_infos)
}

async fn containers_inspect() -> Result<Vec<ContainerInfo> , Box<dyn Error>> {
    let container_list = get_container_list().await?;
    let container_infos: Vec<ContainerInfo> = try_join_all(
        container_list.iter().map(|container| {
            let container_id = container.Id.clone();
            async move {
                let inspect_info = get_container_inspect(&container_id).await?;
                let mut state_map = HashMap::new();
                state_map.insert("Status".to_string(), inspect_info.State.Status);
                state_map.insert("Running".to_string(), inspect_info.State.Running.to_string());
                state_map.insert("Paused".to_string(), inspect_info.State.Paused.to_string());
                state_map.insert("Restarting".to_string(), inspect_info.State.Restarting.to_string());
                state_map.insert("OOMKilled".to_string(), inspect_info.State.OOMKilled.to_string());
                state_map.insert("Dead".to_string(), inspect_info.State.Dead.to_string());
                state_map.insert("Pid".to_string(), inspect_info.State.Pid.to_string());
                state_map.insert("ExitCode".to_string(), inspect_info.State.ExitCode.to_string());
                state_map.insert("Error".to_string(), inspect_info.State.Error);
                state_map.insert("StartedAt".to_string(), inspect_info.State.StartedAt);
                state_map.insert("FinishedAt".to_string(), inspect_info.State.FinishedAt);

                let mut config_map = HashMap::new();
                config_map.insert("Hostname".to_string(), inspect_info.Config.Hostname);
                config_map.insert("Domainname".to_string(), inspect_info.Config.Domainname);
                config_map.insert("User".to_string(), inspect_info.Config.User);
                config_map.insert("AttachStdin".to_string(), inspect_info.Config.AttachStdin.to_string());
                config_map.insert("AttachStdout".to_string(), inspect_info.Config.AttachStdout.to_string());
                config_map.insert("AttachStderr".to_string(), inspect_info.Config.AttachStderr.to_string());
                config_map.insert("Tty".to_string(), inspect_info.Config.Tty.to_string());
                config_map.insert("OpenStdin".to_string(), inspect_info.Config.OpenStdin.to_string());
                config_map.insert("StdinOnce".to_string(), inspect_info.Config.StdinOnce.to_string());
                config_map.insert("Image".to_string(), inspect_info.Config.Image.clone());
                config_map.insert("WorkingDir".to_string(), inspect_info.Config.WorkingDir);

                Ok::<ContainerInfo, Box<dyn Error>>(ContainerInfo {
                    id: inspect_info.Id,
                    names: vec![inspect_info.Name],
                    image: inspect_info.Config.Image.clone(),
                    state: state_map,
                    config: config_map,
                })
            }
        })
    ).await?.into_iter().collect();

    Ok(container_infos)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {  
    let d = pods_inspect().await?;
    println!("pod inspect info: {:#?}", d);

    Ok(())
}

