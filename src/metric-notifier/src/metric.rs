use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct ContainerInspect {
    pub id: String,
    pub name: String,
    pub state: ContainerState,
    pub config: ContainerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ContainerState {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    pub oom_killed: bool,
    pub dead: bool,
    pub pid: i32,
    pub exit_code: i32,
    pub error: String,
    pub started_at: String,
    pub finished_at: String,
}

#[derive(Deserialize, Debug)]
pub struct ContainerConfig {
    pub hostname: String,
    pub domainname: String,
    pub user: String,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub attach_stderr: bool,
    pub exposed_ports: Option<HashMap<String, serde_json::Value>>,
    pub tty: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub env: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub image: String,
    pub volumes: Option<HashMap<String, serde_json::Value>>,
    pub working_dir: String,
    pub entrypoint: String,
    pub on_build: Option<Vec<String>>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct Pod {
    pub id: String,
    pub name: String,
    pub namespace: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct PodInspect {
    pub id: String,
    pub name: String,
    pub created: String,
    pub hostname: String,
    pub state: String,
    pub containers: Vec<PodContainer>,
}

#[derive(Deserialize, Debug)]
pub struct PodContainer {
    pub id: String,
    pub name: String,
    pub state: String,
}
