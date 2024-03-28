const HELP: &str = r#"
Usage - piccoloyaml COMMAND [PARAMETERS]

Available commands:
  apply         make systemd service file
  delete        delete systemd service file

Usage:
  piccoloyaml apply FILE_NAME
  piccoloyaml delete FILE_NAME
"#;

pub fn check(input: &Vec<String>) -> Result<(), &str> {
    if input.len() != 3 {
        return Err(HELP);
    }
    let command = input[1].as_str();
    match command {
        "apply" | "delete" => Ok(()),
        _ => Err(HELP),
    }
}
