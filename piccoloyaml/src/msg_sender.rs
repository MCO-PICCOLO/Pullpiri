use common::apiserver::connection_client::ConnectionClient;
use common::apiserver::{FromServer, ToServer};
use tonic::{Request, Response, Status};

pub async fn send_grpc_msg(req: ToServer) -> Result<Response<FromServer>, Status> {
    println!("sending msg - '{:?}'\n", req);

    let mut client = ConnectionClient::connect(common::apiserver::API_SERVER_CONNECT)
        .await
        .unwrap_or_else(|err| {
            println!("FAIL - {}\ncannot connect to gRPC server", err);
            std::process::exit(1);
        });

    client.send(Request::new(req)).await
}
