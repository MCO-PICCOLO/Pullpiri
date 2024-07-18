use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Container {
    pub Id: String,
    pub Names: Vec<String>,
    pub Image: String,
    pub State: String,
    pub Status: String,
}

#[derive(Deserialize, Debug)]
pub struct ContainerInspect {
    pub Id: String,
    pub Name: String,
    pub State: ContainerState,
    pub Config: ContainerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ContainerState {
    pub Status: String,
    pub Running: bool,
    pub Paused: bool,
    pub Restarting: bool,
    pub OOMKilled: bool,
    pub Dead: bool,
    pub Pid: i32,
    pub ExitCode: i32,
    pub Error: String,
    pub StartedAt: String,
    pub FinishedAt: String,
}

#[derive(Deserialize, Debug)]
pub struct ContainerConfig {
    pub Hostname: String,
    pub Domainname: String,
    pub User: String,
    pub AttachStdin: bool,
    pub AttachStdout: bool,
    pub AttachStderr: bool,
    pub ExposedPorts: Option<HashMap<String, serde_json::Value>>,
    pub Tty: bool,
    pub OpenStdin: bool,
    pub StdinOnce: bool,
    pub Env: Option<Vec<String>>,
    pub Cmd: Option<Vec<String>>,
    pub Image: String,
    pub Volumes: Option<HashMap<String, serde_json::Value>>,
    pub WorkingDir: String,
    pub Entrypoint: String,
    pub OnBuild: Option<Vec<String>>,
    pub Labels: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct Pod {
    pub Id: String,
    pub Name: String,
    pub Namespace: String,
    pub Status: String,
}

#[derive(Deserialize, Debug)]
pub struct PodInspect {
    pub Id: String,
    pub Name: String,
    pub Created: String,
    pub Hostname: String,
    pub State: String,
    pub Containers: Vec<PodContainer>,
}

#[derive(Deserialize, Debug)]
pub struct PodContainer {
    pub Id: String,
    pub Name: String,
    pub State: String,
}
