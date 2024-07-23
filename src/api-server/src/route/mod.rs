pub mod package;
pub mod scenario;

pub struct RestRequest {
    pub action: Action,
    pub resource: Resource,
}

enum Action {
    LAUNCH,
    UPDATE,
    DELETE,
}

pub enum Resource {
    Package(TempPackage),
    Scenario(TempScenario),
}

pub struct TempPackage {
    pub pac_name: String,
}
pub struct TempScenario {
    pub sce_name: String,
}
