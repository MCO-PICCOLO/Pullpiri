use super::super::workload::podspec::PodSpec;
use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Model {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    pub spec: PodSpec,
}

impl Model {
    pub fn get_name(&self) -> String {
        self.metadata.name.clone()
    }

    pub fn get_image(&self) -> String {
        self.spec.get_image()
    }

    pub fn get_podspec(&self) -> PodSpec {
        self.spec.clone()
    }
}
