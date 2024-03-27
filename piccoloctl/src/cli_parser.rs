fn help() {
    println!("Usage - piccoloctl COMMAND [PARAMETERS]");
    println!("Available command");
    println!("  - list-node: shows node list");
    println!("    usage: piccoloctl list-node");
    println!("  - list-unit: shows unit list in node");
    println!("    usage: piccoloctl list-unit NODE_NAME");
    println!("  - start: start unit in node");
    println!("    usage: piccoloctl start NODE_NAME UNIT_NAME");
    println!("  - stop: stop unit in node");
    println!("    usage: piccoloctl stop NODE_NAME UNIT_NAME");
    println!("  - restart: restart unit in node");
    println!("    usage: piccoloctl restart NODE_NAME UNIT_NAME");
    println!("  - reload: reload unit in node");
    println!("    usage: piccoloctl reload NODE_NAME UNIT_NAME");
    println!("  - enable: enable unit in node");
    println!("    usage: piccoloctl enable NODE_NAME UNIT_NAME");
    println!("  - disable: disable unit in node");
    println!("    usage: piccoloctl disable NODE_NAME UNIT_NAME");
    println!("  - daemon-reload: equivalent to 'systemctl daemon-reload'");
    println!("    usage: piccoloctl daemon-reload NODE_NAME");
}

pub fn check(input: &Vec<String>) -> Result<String, String> {
    match input.len() {
        2 => Ok(format!("{}", input[1])),
        3 => Ok(format!("{}/{}", input[1], input[2])),
        4 => Ok(format!("{}/{}/{}", input[1], input[2], input[3])),
        _ => {
            help();
            std::process::exit(1);
        }
    }
}
