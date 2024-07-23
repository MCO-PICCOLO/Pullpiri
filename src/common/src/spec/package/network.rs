use crate::spec::workload::podspec;

use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Network {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: Option<NetworkSpec>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct NetworkSpec {
    spec: Option<Vec<podspec::Port>>,
}
