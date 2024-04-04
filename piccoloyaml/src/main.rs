mod cli_parser;
mod file_handler;
mod msg_sender;

use clap::Parser;
use common::ControllerCommand;

fn abnormal_termination<T: std::fmt::Display>(err: T) {
    println!("- FAIL -\n{}", err);
    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    let args = cli_parser::Arguments::parse();
    let (cmd, yaml_path) = match &args.command {
        cli_parser::Command::Apply(file) => ("apply", &file.name),
        cli_parser::Command::Delete(file) => ("delete", &file.name),
    };

    file_handler::handle(cmd, yaml_path).unwrap_or_else(|err| abnormal_termination(err));

    let req = common::get_controller_command(ControllerCommand::DaemonReload);

    match msg_sender::send_grpc_msg(req).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().response),
        Err(t) => abnormal_termination(t),
    }
}
