mod grpc_msg_handler;

use crate::grpc_msg_handler::PiccoloGrpcServer;
use common::apiserver::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::apiserver::API_SERVER_OPEN.parse().unwrap();
    let piccolo_grpc_server = PiccoloGrpcServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ConnectionServer::new(piccolo_grpc_server))
        .serve(addr)
        .await;
}
