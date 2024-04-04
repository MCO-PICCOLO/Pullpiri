mod cli_parser;
mod msg_sender;

use clap::Parser;
use common::{ControllerCommand, NodeCommand, UpdateMethod};

#[tokio::main]
async fn main() {
    let args = cli_parser::Arguments::parse();
    println!("{:?}", args);

    let req = match &args.command {
        cli_parser::Command::ListNode => {
            common::get_controller_command(ControllerCommand::ListNode)
        }
        cli_parser::Command::DaemonReload => {
            common::get_controller_command(ControllerCommand::DaemonReload)
        }
        cli_parser::Command::ListUnit(n) => {
            common::get_node_command(NodeCommand::ListUnit, &n.node_name)
        }
        cli_parser::Command::Start(u) => {
            common::get_unit_command(UpdateMethod::Start, &u.node_name, &u.unit_name)
        }
        cli_parser::Command::Stop(u) => {
            common::get_unit_command(UpdateMethod::Stop, &u.node_name, &u.unit_name)
        }
        cli_parser::Command::Restart(u) => {
            common::get_unit_command(UpdateMethod::Restart, &u.node_name, &u.unit_name)
        }
        cli_parser::Command::Reload(u) => {
            common::get_unit_command(UpdateMethod::Reload, &u.node_name, &u.unit_name)
        }
        cli_parser::Command::Enable(u) => {
            common::get_unit_command(UpdateMethod::Enable, &u.node_name, &u.unit_name)
        }
        cli_parser::Command::Disable(u) => {
            common::get_unit_command(UpdateMethod::Disable, &u.node_name, &u.unit_name)
        }
    };

    match msg_sender::send_grpc_msg(req).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().response),
        Err(t) => println!("FAIL - {:#?}", t),
    }
}
