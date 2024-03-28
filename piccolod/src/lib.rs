mod etcd;
mod method_bluechi;
use method_bluechi::{method_controller, method_node, method_unit};

pub mod command {
    tonic::include_proto!("command");
}
use command::command_server::{Command, CommandServer};
use command::{SendReply, SendRequest};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct PiccoloGrpcServer {}

#[tonic::async_trait]
impl Command for PiccoloGrpcServer {
    async fn send(&self, request: Request<SendRequest>) -> Result<Response<SendReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let request = request.into_inner();
        let msg = request.cmd;

        match send_dbus_to_bluechi(&msg).await {
            Ok(v) => Ok(Response::new(SendReply { ans: true, desc: v })),
            Err(e) => Err(Status::new(tonic::Code::Unavailable, e.to_string())),
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

pub async fn run() {
    etcd::init_server();

    let addr = "[::1]:50101".parse().unwrap();
    let piccolo_grpc_server = PiccoloGrpcServer::default();

    println!("Test Server listening on {}", addr);

    let _ = Server::builder()
        .add_service(CommandServer::new(piccolo_grpc_server))
        .serve(addr)
        .await;
}
