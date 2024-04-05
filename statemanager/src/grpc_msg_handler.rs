use crate::method_bluechi::{method_controller, method_node, method_unit};
use common::etcd;
use common::statemanager::connection_server::Connection;
use common::statemanager::{SendRequest, SendResponse};

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

        match send_dbus_to_bluechi(&command).await {
            Ok(v) => Ok(tonic::Response::new(SendResponse { response: v })),
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
