/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

mod grpc;
mod manager;
mod route;

use axum::Router;
use common::apiserver::scenario_connection_server::ScenarioConnectionServer;
use tonic::transport::Server;

async fn running_grpc() {
    let addr = common::apiserver::open_server()
        .parse()
        .expect("api-server address parsing error");
    let scenario_server = grpc::receiver::scenario_handler::GrpcUpdateServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ScenarioConnectionServer::new(scenario_server))
        .serve(addr)
        .await;
}

async fn running_rest() {
    let app = Router::new()
        .merge(route::package::get_route())
        .merge(route::scenario::get_route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:47099")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    let f_grpc = running_grpc();
    let f_rest = running_rest();

    tokio::join!(f_grpc, f_rest);
}
