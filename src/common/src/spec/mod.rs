/*
 * SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */

pub mod package;
pub mod pod;
pub mod scenario;
pub mod workload;

use config::Map;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct MetaData {
    name: String,
    labels: Option<Map<String, String>>,
}
