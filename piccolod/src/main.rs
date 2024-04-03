mod etcd;
mod grpc_msg_handler;
mod method_bluechi;

use crate::grpc_msg_handler::PiccoloGrpcServer;
use api::proto::piccolo::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::DEFAULT_API_SERVER_ENDPOINT.parse().unwrap();
    let piccolo_grpc_server = PiccoloGrpcServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ConnectionServer::new(piccolo_grpc_server))
        .serve(addr)
        .await;
}
