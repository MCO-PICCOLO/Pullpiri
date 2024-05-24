/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/constants.proto")?;
    tonic_build::compile_protos("proto/apiserver.proto")?;
    tonic_build::compile_protos("proto/apiserver/request.proto")?;
    tonic_build::compile_protos("proto/apiserver/updateworkload.proto")?;
    tonic_build::compile_protos("proto/apiserver/scenario.proto")?;
    tonic_build::compile_protos("proto/statemanager.proto")?;
    tonic_build::compile_protos("proto/gateway.proto")?;
    tonic_build::compile_protos("proto/yamlparser.proto")?;
    Ok(())
}