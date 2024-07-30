use super::workload::podspec::PodSpec;
use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Pod {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: PodSpec,
}

impl Pod {
    pub fn new(name: &str, podspec: PodSpec) -> Pod {
        Pod {
            apiVersion: "v1".to_string(),
            kind: "Pod".to_string(),
            metadata: MetaData {
                name: name.to_string(),
                labels: None,
            },
            spec: podspec,
        }
    }
}
