mod cli_parser;
mod file_handler;
mod msg_sender;

use clap::Parser;
use common::apiserver::ControllerCommand;

#[tokio::main]
async fn main() {
    let args = cli_parser::Arguments::parse();
    let (cmd, yaml_path) = match &args.command {
        cli_parser::Command::Apply(file) => ("apply", &file.name),
        cli_parser::Command::Delete(file) => ("delete", &file.name),
    };

    file_handler::handle(cmd, yaml_path).unwrap_or_else(|err| {
        println!("- FAIL -\n{:#?}", err);
        std::process::exit(1);
    });

    let req = common::get_controller_command(ControllerCommand::DaemonReload);

    match msg_sender::send_grpc_msg(req).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().response),
        Err(t) => println!("- FAIL -\n{:#?}", t),
    }
}
