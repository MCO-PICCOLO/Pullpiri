mod cli_parser;
mod msg_sender;

use api::proto::piccolo::request::RequestContent;
use api::proto::piccolo::to_server::ToServerContent;
use api::proto::piccolo::{
    ControllerCommand, ControllerRequest, NodeCommand, NodeRequest, Request, ToServer,
    UpdateMethod, UpdateWorkload,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _cmd = cli_parser::check(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    let _req = ToServer {
        to_server_content: Some(ToServerContent::Request(Request {
            request_content: Some(RequestContent::ControllerRequest(ControllerRequest {
                controller_command: ControllerCommand::DaemonReload.into(),
            })),
        })),
    };

    let _req2 = ToServer {
        to_server_content: Some(ToServerContent::Request(Request {
            request_content: Some(RequestContent::NodeRequest(NodeRequest {
                node_command: NodeCommand::ListUnit.into(),
                node_name: "".to_owned(),
            })),
        })),
    };

    let req3 = ToServer {
        to_server_content: Some(ToServerContent::UpdateWorkload(UpdateWorkload {
            update_method: UpdateMethod::Start.into(),
            node_name: "nuc-cent".to_owned(),
            unit_name: "test-service".to_owned(),
        })),
    };

    match msg_sender::send_grpc_msg(req3).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().response),
        Err(t) => println!("FAIL - {:#?}", t),
    }
}
