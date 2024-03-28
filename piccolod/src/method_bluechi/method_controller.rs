use dbus::blocking::Connection;
use dbus::Path;
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

fn reload_all_nodes() -> Result<String, Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let node_name = "nuc-cent";

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));
    node_proxy.method_call("org.eclipse.bluechi.Node", "Reload", ())?;

    Ok(format!("reload node '{}'\n", node_name))
}

pub fn handle_cmd(c: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match c[0] {
        "list-node" => list_nodes(),
        "apply" | "delete" => reload_all_nodes(),
        _ => Err("cannot find command".into()),
    }
}
