use dbus::blocking::Connection;
use dbus::Path;
use std::time::Duration;

enum Lifecycle {
    Start,
    Stop,
    Restart,
    Reload,
}

fn unit_lifecycle(
    life_cycle: Lifecycle,
    node_name: &str,
    unit_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let method: &str = match life_cycle {
        Lifecycle::Start => "StartUnit",
        Lifecycle::Stop => "StopUnit",
        Lifecycle::Restart => "RestartUnit",
        Lifecycle::Reload => "ReloadUnit",
    };
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    let (job_path,): (Path,) =
        node_proxy.method_call("org.eclipse.bluechi.Node", method, (unit_name, "replace"))?;

    Ok(format!(
        "{method} '{unit_name}' on node '{node_name}': {job_path}\n"
    ))
}

fn enable_unit(node_name: &str, unit_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let unit_vector = vec![unit_name.to_owned()];
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    let (carries_install_info, changes): (bool, Vec<(String, String, String)>) = node_proxy
        .method_call(
            "org.eclipse.bluechi.Node",
            "EnableUnitFiles",
            (unit_vector, false, false),
        )?;

    let mut result: String = match carries_install_info {
        true => format!("The unit files included enablement information\n"),
        false => format!("The unit files did not include any enablement information\n"),
    };

    for (op_type, file_name, file_dest) in changes {
        if op_type == "symlink" {
            result.push_str(&format!("Created symlink {file_name} -> {file_dest}\n"));
        } else if op_type == "unlink" {
            result.push_str(&format!("Removed '{file_name}'\n"));
        }
    }

    Ok(result)
}

fn disable_unit(node_name: &str, unit_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let unit_vector = vec![unit_name.to_owned()];
    let conn = Connection::new_system()?;

    let bluechi = conn.with_proxy(
        "org.eclipse.bluechi",
        "/org/eclipse/bluechi",
        Duration::from_millis(5000),
    );

    let (node,): (Path,) =
        bluechi.method_call("org.eclipse.bluechi.Controller", "GetNode", (node_name,))?;

    let node_proxy = conn.with_proxy("org.eclipse.bluechi", node, Duration::from_millis(5000));

    let (changes,): (Vec<(String, String, String)>,) = node_proxy.method_call(
        "org.eclipse.bluechi.Node",
        "DisableUnitFiles",
        (unit_vector, false),
    )?;

    let mut result = String::new();
    for (op_type, file_name, file_dest) in changes {
        if op_type == "symlink" {
            result.push_str(&format!("Created symlink {file_name} -> {file_dest}\n"));
        } else if op_type == "unlink" {
            result.push_str(&format!("Removed '{file_name}'\n"));
        }
    }
    Ok(result)
}

pub fn handle_cmd(c: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    match c[0] {
        "start" => unit_lifecycle(Lifecycle::Start, c[1], c[2]),
        "stop" => unit_lifecycle(Lifecycle::Stop, c[1], c[2]),
        "restart" => unit_lifecycle(Lifecycle::Restart, c[1], c[2]),
        "reload" => unit_lifecycle(Lifecycle::Reload, c[1], c[2]),
        "enable" => enable_unit(c[1], c[2]),
        "disable" => disable_unit(c[1], c[2]),
        _ => Err("cannot find command".into()),
    }
}
