use std::env;
use std::process;

mod cli_parser;
mod cmd_check;
pub mod command {
    tonic::include_proto!("command");
}
use command::command_client::CommandClient;
use command::SendRequest;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let config = cli_parser::check(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("sending msg - '{config}'\n");

    let mut client = CommandClient::connect("http://[::1]:50101")
        .await
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });
    let request = tonic::Request::new(SendRequest { cmd: config });
    let response = client.send(request).await;
    match response {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().desc),
        Err(t) => println!("FAIL - {:#?}", t),
    }
}
