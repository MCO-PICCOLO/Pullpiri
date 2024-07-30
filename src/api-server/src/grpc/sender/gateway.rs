/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::gateway;

pub async fn send(
    event: gateway::EventName,
) -> Result<tonic::Response<gateway::Reply>, tonic::Status> {
    let mut client =
        match gateway::piccolo_gateway_service_client::PiccoloGatewayServiceClient::connect(
            gateway::connect_server(),
        )
        .await
        {
            Ok(c) => c,
            Err(_) => {
                return Err(tonic::Status::new(
                    tonic::Code::Unavailable,
                    "cannot connect gateway",
                ))
            }
        };

    client.request_event(tonic::Request::new(event)).await
}
