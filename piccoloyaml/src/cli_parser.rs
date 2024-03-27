use crate::cmd_check::command_check;

fn help() -> String {
    println!("Usage - piccoloyaml COMMAND [PARAMETERS]");
    println!("Available command");
    println!("  - apply: make systemd service file");
    println!("    usage: piccoloyaml apply FILE_NAME");
    println!("  - delete: delete systemd service file");
    println!("    usage: piccoloyaml delete FILE_NAME");
    "not support".to_owned()
}

pub fn check(input: &Vec<String>) -> Result<String, String> {
    command_check(input);
    match input.len() {
        2 => Ok(format!("{}", input[1])),
        3 => Ok(format!("{}/{}", input[1], input[2])),
        4 => Ok(format!("{}/{}/{}", input[1], input[2], input[3])),
        _ => Err(help()),
    }
}
