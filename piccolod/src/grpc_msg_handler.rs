use crate::etcd;
use crate::method_bluechi::{method_controller, method_node, method_unit};

use api::proto::piccolo::connection_server::Connection;
use api::proto::piccolo::request::RequestContent::{ControllerRequest, NodeRequest};
use api::proto::piccolo::to_server::ToServerContent::{Request, UpdateWorkload};
use api::proto::piccolo::{FromServer, ToServer};

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
        let command = parse_to_server_command(&request);

        match send_dbus_to_bluechi(&command).await {
            Ok(v) => Ok(tonic::Response::new(FromServer { response: v })),
            Err(e) => Err(tonic::Status::new(tonic::Code::Unavailable, e.to_string())),
        }
    }
}

fn parse_to_server_command(req: &ToServer) -> String {
    let mut ret = String::new();
    if let Some(to_server_command) = &req.to_server_content {
        match to_server_command {
            UpdateWorkload(update_workload) => {
                ret = format!(
                    "{}/{}/{}",
                    update_workload.update_method().as_str_name(),
                    &update_workload.node_name,
                    &update_workload.unit_name
                );
            }
            Request(request) => {
                if let Some(request_command) = &request.request_content {
                    match request_command {
                        ControllerRequest(controller_request) => {
                            ret = format!(
                                "{}",
                                controller_request.controller_command().as_str_name()
                            );
                        }
                        NodeRequest(node_request) => {
                            ret = format!(
                                "{}/{}",
                                node_request.node_command().as_str_name(),
                                &node_request.node_name
                            );
                        }
                    }
                }
            }
        }
    }
    ret
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
