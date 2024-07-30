/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

//use crate::grpc::sender::gateway;
use common::apiserver::scenario::{Response, Scenario};
use common::apiserver::scenario_connection_server::ScenarioConnection;

#[derive(Default)]
pub struct GrpcUpdateServer {}

#[tonic::async_trait]
impl ScenarioConnection for GrpcUpdateServer {
    async fn send(
        &self,
        request: tonic::Request<Scenario>,
    ) -> Result<tonic::Response<Response>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());

        //let scenario = request.into_inner();

        /*match gateway::send_msg_to_gateway(scenario).await {
            Ok(_) => Ok(tonic::Response::new(Response {
                resp: true.to_string(),
            })),
            Err(e) => Err(e),
        }*/
        Ok(tonic::Response::new(Response {
            resp: true.to_string(),
        }))
    }
}
