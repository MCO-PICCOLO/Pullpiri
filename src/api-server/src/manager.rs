use common::gateway;
use importer::parser::package::Package;
use importer::parser::scenario::Scenario;

pub async fn handle_package_msg(p: Package) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("package/{}", p.name);
    common::etcd::put(&format!("{key_origin}/models"), &p.models).await?;
    common::etcd::put(&format!("{key_origin}/network"), &p.network).await?;
    common::etcd::put(&format!("{key_origin}/volume"), &p.volume).await?;
    Ok(())
}

pub async fn handle_scenario_msg(s: Scenario) -> Result<(), Box<dyn std::error::Error>> {
    let key_origin = format!("scenario/{}", s.name);
    common::etcd::put(&format!("{key_origin}/actions"), &s.actions).await?;
    common::etcd::put(&format!("{key_origin}/conditions"), &s.conditions).await?;
    common::etcd::put(&format!("{key_origin}/targets"), &s.targets).await?;
    common::etcd::put(&format!("{key_origin}/full"), &s.scene).await?;

    let event = gateway::EventName {
        id: gateway::FuncId::Enable.into(),
        name: format!("scenario/{}", &s.name),
        target: gateway::Target::StateManager.into(),
    };

    crate::grpc::sender::gateway::send(event).await?;

    Ok(())
}
