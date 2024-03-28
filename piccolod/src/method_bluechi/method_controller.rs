use dbus::blocking::Connection;
use std::time::Duration;

fn list_nodes() -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (nodes,): (Vec<(String, dbus::Path, String)>,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "ListNodes", ())?;

    let mut result = String::new();
    for (name, _, status) in nodes {
        result.push_str(&format!("Node: {}, Status: {}\n", name, status));
    }
    Ok(result)
}

pub fn handle_cmd(c: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match c[0] {
        "list-node" => list_nodes(),
        _ => Err("cannot find command".into()),
    }
}
