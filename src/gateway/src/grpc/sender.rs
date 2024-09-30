/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

use common::statemanager;

#[allow(dead_code)]
pub async fn to_statemanager(
    msg: &str,
) -> Result<tonic::Response<statemanager::SendResponse>, tonic::Status> {
    println!("sending msg - '{}'\n", msg);

    let mut client = match statemanager::connection_client::ConnectionClient::connect(
        statemanager::connect_server(),
    )
    .await
    {
        Ok(c) => c,
        Err(_) => {
            return Err(tonic::Status::new(
                tonic::Code::Unavailable,
                "cannot connect statemanager",
            ))
        }
    };

    client
        .send(tonic::Request::new(statemanager::SendRequest {
            from: common::constants::PiccoloModuleName::Gateway.into(),
            request: msg.to_owned(),
        }))
        .await
}
