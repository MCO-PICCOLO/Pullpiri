use api::proto::piccolo::connection_client::ConnectionClient;
use api::proto::piccolo::{FromServer, ToServer};
use tonic::{Request, Response, Status};

pub async fn send_grpc_msg(req: ToServer) -> Result<Response<FromServer>, Status> {
    println!("sending msg - '{:?}'\n", req);

    let mut client = ConnectionClient::connect(common::DEFAULT_API_SERVER_ENDPOINT)
        .await
        .unwrap_or_else(|err| {
            println!("FAIL - {}\ncannot connect to gRPC server", err);
            std::process::exit(1);
        });

    client.send(Request::new(req)).await
}
