use api::proto::apiserver::request::RequestContent;
use api::proto::apiserver::to_server::ToServerContent;

pub mod apiserver {
    pub use api::proto::apiserver::*;
}

pub mod statemanager {
    pub use api::proto::statemanager::*;
}

pub const DEFAULT_API_SERVER_OPEN: &str = "[::1]:50101";
pub const DEFAULT_API_SERVER_CONNECT: &str = "http://[::1]:50101";
pub const DEFAULT_STATE_MANAGER_OPEN: &str = "[::1]:50010";
pub const DEFAULT_STATE_MANAGER_CONNECT: &str = "http://[::1]:50010";
pub const DEFAULT_ETCD_ENDPOINT: &str = "127.0.0.1:2379";

/** Followings are defined in api::proto::apiserver module.
pub enum UpdateMethod {
    START = 0,
    STOP = 1,
    RESTART = 2,
    RELOAD = 3,
    ENABLE = 4,
    DISABLE = 5,
}
pub enum ControllerCommand {
    ListNode = 0,
    DaemonReload = 1,
}
pub enum NodeCommand {
    ListUnit = 0,
}
**/

pub fn get_controller_command(cmd: apiserver::ControllerCommand) -> apiserver::ToServer {
    apiserver::ToServer {
        to_server_content: Some(ToServerContent::Request(apiserver::Request {
            request_content: Some(RequestContent::ControllerRequest(
                apiserver::ControllerRequest {
                    controller_command: cmd.into(),
                },
            )),
        })),
    }
}

pub fn get_node_command(cmd: apiserver::NodeCommand, node_name: &str) -> apiserver::ToServer {
    apiserver::ToServer {
        to_server_content: Some(ToServerContent::Request(apiserver::Request {
            request_content: Some(RequestContent::NodeRequest(apiserver::NodeRequest {
                node_command: cmd.into(),
                node_name: node_name.to_owned(),
            })),
        })),
    }
}

pub fn get_unit_command(
    cmd: apiserver::UpdateMethod,
    node_name: &str,
    unit_name: &str,
) -> apiserver::ToServer {
    apiserver::ToServer {
        to_server_content: Some(ToServerContent::UpdateWorkload(apiserver::UpdateWorkload {
            update_method: cmd.into(),
            node_name: node_name.to_owned(),
            unit_name: unit_name.to_owned(),
        })),
    }
}
