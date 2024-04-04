use api::proto::piccolo::request::RequestContent;
use api::proto::piccolo::to_server::ToServerContent;
pub use api::proto::piccolo::{
    ControllerCommand, ControllerRequest, NodeCommand, NodeRequest, Request, ToServer,
    UpdateMethod, UpdateWorkload,
};

pub const DEFAULT_API_SERVER_OPEN: &str = "[::1]:50101";
pub const DEFAULT_API_SERVER_CONNECT: &str = "http://[::1]:50101";
pub const DEFAULT_ETCD_ENDPOINT: &str = "127.0.0.1:2379";

/** Followings are defined in api::proto::piccolo module.
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

pub fn get_controller_command(cmd: ControllerCommand) -> ToServer {
    ToServer {
        to_server_content: Some(ToServerContent::Request(Request {
            request_content: Some(RequestContent::ControllerRequest(ControllerRequest {
                controller_command: cmd.into(),
            })),
        })),
    }
}

pub fn get_node_command(cmd: NodeCommand, node_name: &str) -> ToServer {
    ToServer {
        to_server_content: Some(ToServerContent::Request(Request {
            request_content: Some(RequestContent::NodeRequest(NodeRequest {
                node_command: cmd.into(),
                node_name: node_name.to_owned(),
            })),
        })),
    }
}

pub fn get_unit_command(cmd: UpdateMethod, node_name: &str, unit_name: &str) -> ToServer {
    ToServer {
        to_server_content: Some(ToServerContent::UpdateWorkload(UpdateWorkload {
            update_method: cmd.into(),
            node_name: node_name.to_owned(),
            unit_name: unit_name.to_owned(),
        })),
    }
}
