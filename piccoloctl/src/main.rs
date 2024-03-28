mod cli_parser;
mod msg_sender;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = cli_parser::check(&args).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    match msg_sender::send_grpc_msg(cmd).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().desc),
        Err(t) => println!("FAIL - {:#?}", t),
    }
}
