use crate::etcd;
use crate::method_bluechi::{method_controller, method_node, method_unit};

use api::proto::piccolo::connection_server::Connection;
use api::proto::piccolo::request::RequestContent::{ControllerRequest, NodeRequest};
use api::proto::piccolo::to_server::ToServerContent::{Request, UpdateWorkload};
use api::proto::piccolo::{ControllerCommand, FromServer, NodeCommand, ToServer, UpdateMethod};

#[derive(Default)]
pub struct PiccoloGrpcServer {}

#[tonic::async_trait]
impl Connection for PiccoloGrpcServer {
    async fn send(
        &self,
        request: tonic::Request<ToServer>,
    ) -> Result<tonic::Response<FromServer>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let request = request.into_inner();
        //let msg = request.to_server_content;
        if let Some(to_server_command) = request.to_server_content {
            match to_server_command {
                UpdateWorkload(update_workload) => match update_workload.update_method() {
                    UpdateMethod::Start => todo!(),
                    UpdateMethod::Stop => todo!(),
                    UpdateMethod::Restart => todo!(),
                    UpdateMethod::Reload => todo!(),
                    UpdateMethod::Enable => todo!(),
                    UpdateMethod::Disable => todo!(),
                },
                Request(request) => {
                    if let Some(request_command) = request.request_content {
                        match request_command {
                            ControllerRequest(controller_request) => {
                                match controller_request.controller_command() {
                                    ControllerCommand::ListNode => todo!(),
                                    ControllerCommand::DaemonReload => todo!(),
                                }
                            }
                            NodeRequest(node_request) => match node_request.node_command() {
                                NodeCommand::ListUnit => todo!(),
                            },
                        }
                    }
                }
            }
        }

        let asdf = "asdf";
        match send_dbus_to_bluechi(asdf).await {
            Ok(v) => Ok(tonic::Response::new(FromServer { response: v })),
            Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
        }
    }
}

async fn send_dbus_to_bluechi(msg: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("recv msg: {}\n", msg);
    let cmd: Vec<&str> = msg.split("/").collect();
    // put-get test command for etcd operation
    etcd::put(msg, msg).await?;
    etcd::get(msg).await?;

    match cmd.len() {
        1 => method_controller::handle_cmd(cmd),
        2 => method_node::handle_cmd(cmd),
        3 => method_unit::handle_cmd(cmd),
        _ => {
            etcd::delete(msg).await?;
            Err("support only 1 ~ 3 parameters".into())
        }
    }
}
