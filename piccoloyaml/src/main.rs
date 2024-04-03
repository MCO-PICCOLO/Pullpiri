mod cli_parser;
mod file_handler;
mod msg_sender;

use api::proto::piccolo::request::RequestContent;
use api::proto::piccolo::to_server::ToServerContent;
use api::proto::piccolo::{ControllerCommand, ControllerRequest, Request, ToServer};
use clap::Parser;

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

    let req = ToServer {
        to_server_content: Some(ToServerContent::Request(Request {
            request_content: Some(RequestContent::ControllerRequest(ControllerRequest {
                controller_command: ControllerCommand::DaemonReload.into(),
            })),
        })),
    };

    match msg_sender::send_grpc_msg(req).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().response),
        Err(t) => abnormal_termination(t),
    }
}
