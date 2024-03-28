pub mod command {
    tonic::include_proto!("command");
}
use command::command_client::CommandClient;
use command::SendRequest;
use tonic::{Request, Response, Status};

pub async fn send_grpc_msg(msg: &str) -> Result<Response<command::SendReply>, Status> {
    println!("sending msg - '{msg}'\n");

    let mut client = CommandClient::connect("http://[::1]:50101")
        .await
        .unwrap_or_else(|err| {
            println!("FAIL - {}\ncannot connect to gRPC server", err);
            std::process::exit(1);
        });

    client
        .send(Request::new(SendRequest {
            cmd: msg.to_owned(),
        }))
        .await
}
