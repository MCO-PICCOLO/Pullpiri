/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::gateway::piccolo_gateway_service_server::PiccoloGatewayService;
use common::gateway::{EventName, Reply};
use tokio::sync::mpsc;

pub struct GrpcServer {
    pub grpc_msg_tx: mpsc::Sender<EventName>,
}

#[tonic::async_trait]
impl PiccoloGatewayService for GrpcServer {
    async fn request_event(
        &self,
        request: tonic::Request<EventName>,
    ) -> Result<tonic::Response<Reply>, tonic::Status> {
        println!("Got a request from {:?}", request.remote_addr());
        let req = request.into_inner();
        //println!("req msg : {:#?}", req);
        let _ = self.grpc_msg_tx.send(req).await;

        Ok(tonic::Response::new(Reply { is_ok: true }))
    }
}
