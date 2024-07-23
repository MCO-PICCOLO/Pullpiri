use super::super::workload::podspec;
use super::MetaData;

#[allow(non_snake_case)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Volume {
    apiVersion: String,
    kind: String,
    metadata: MetaData,
    spec: Option<VolumeSpec>,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VolumeSpec {
    volumes: Option<Vec<podspec::Volume>>,
}

impl Volume {
    pub fn get_spec(&self) -> &Option<VolumeSpec> {
        &self.spec
    }
}

impl VolumeSpec {
    pub fn get_volume(&self) -> &Option<Vec<podspec::Volume>> {
        &self.volumes
    }
}