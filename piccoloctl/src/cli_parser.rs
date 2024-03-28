const HELP: &str = r#"
Usage - piccoloctl COMMAND [PARAMETERS]

Available commands:
  list-node         shows node list
  list-unit         shows unit list in node
  start             start unit in node
  stop              stop unit in node
  restart           restart unit in node
  reload            reload unit in node
  enable            enable unit in node
  disable           disable unit in node
  daemon-reload     equivalent to 'systemctl daemon-reload'

Usage:
  piccoloctl list-node
  piccoloctl list-unit NODE_NAME
  piccoloctl start NODE_NAME UNIT_NAME
  piccoloctl stop NODE_NAME UNIT_NAME
  piccoloctl restart NODE_NAME UNIT_NAME
  piccoloctl reload NODE_NAME UNIT_NAME
  piccoloctl enable NODE_NAME UNIT_NAME
  piccoloctl disable NODE_NAME UNIT_NAME
  piccoloctl daemon-reload NODE_NAME
"#;

pub fn check(input: &Vec<String>) -> Result<String, &str> {
    match input.len() {
        2 => Ok(format!("{}", input[1])),
        3 => Ok(format!("{}/{}", input[1], input[2])),
        4 => Ok(format!("{}/{}/{}", input[1], input[2], input[3])),
        _ => Err(HELP),
    }
}
