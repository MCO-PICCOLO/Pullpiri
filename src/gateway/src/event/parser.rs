use common::etcd;

pub async fn parse(name: &str, target_dest: i32) -> super::Event {
    let action_key = name.to_string();
    let conditions = etcd::get(&format!("{name}/conditions")).await.unwrap();
    let condition: common::spec::scenario::Condition = serde_yaml::from_str(&conditions).unwrap();

    super::Event {
        name: name.to_string(),
        express: condition.get_express(),
        target_value: condition.get_value(),
        topic: condition.get_operand_value(),
        action_key,
        target_dest,
        life_cycle: 1,
    }
}
