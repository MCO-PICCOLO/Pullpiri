/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

mod grpc;
mod rest;

//use common::apiserver::scenario_connection_server::ScenarioConnectionServer;
//use tonic::transport::Server;

use axum::routing::{delete, get, post};
use axum::Router;

#[tokio::main]
async fn main() {
    /*let addr = common::apiserver::open_server()
        .parse()
        .expect("api-server address parsing error");
    let scenario_server = grpc::receiver::scenario_handler::GrpcUpdateServer::default();

    println!("Piccolod api-server listening on {}", addr);

    let _ = Server::builder()
        .add_service(ScenarioConnectionServer::new(scenario_server))
        .serve(addr)
        .await;*/

    let app = Router::new()
        .route("/scenario/:name", get(rest::inspect_scenario))
        .route("/list", get(rest::list_scenario))
        .route("/create-scenario/:name", post(rest::make_scenario))
        .route("/delete-scenario/:name", delete(rest::delete_scenario));

    let listener = tokio::net::TcpListener::bind("10.157.19.218:9090")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
