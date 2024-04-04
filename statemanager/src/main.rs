mod etcd;
mod grpc_msg_handler;
mod method_bluechi;

use crate::grpc_msg_handler::StateManagerGrpcServer;
use api::proto::statemanager::connection_server::ConnectionServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let addr = common::DEFAULT_STATE_MANAGER_OPEN.parse().unwrap();
    let state_manager_grpc_server = StateManagerGrpcServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ConnectionServer::new(state_manager_grpc_server))
        .serve(addr)
        .await;
}
