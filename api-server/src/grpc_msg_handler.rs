use common::apiserver::connection_server::Connection;
use common::apiserver::request::RequestContent::{ControllerRequest, NodeRequest};
use common::apiserver::to_server::ToServerContent::{Request, UpdateWorkload};
use common::apiserver::{FromServer, ToServer};
use common::etcd;
use common::statemanager;

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
            Ok(v) => Ok(tonic::Response::new(FromServer {
                response: v.into_inner().response,
            })),
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

async fn send_dbus_to_bluechi(
    msg: &str,
) -> Result<tonic::Response<statemanager::SendResponse>, tonic::Status> {
    println!("sending msg - '{}'\n", msg);
    let _ = etcd::put("asdf", "asdf").await;
    let _ = etcd::get("asdf").await;
    let _ = etcd::delete("asdf").await;

    let mut client = statemanager::connection_client::ConnectionClient::connect(
        statemanager::STATE_MANAGER_CONNECT,
    )
    .await
    .unwrap_or_else(|err| {
        println!("FAIL - {}\ncannot connect to gRPC server", err);
        std::process::exit(1);
    });

    client
        .send(tonic::Request::new(statemanager::SendRequest {
            from: "api-server".to_owned(),
            request: msg.to_owned(),
        }))
        .await
}
