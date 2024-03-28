mod cli_parser;
mod file_checker;
mod msg_sender;

fn abnormal_termination<T: std::fmt::Display>(err: T) {
    println!("- FAIL -\n{}", err);
    std::process::exit(1);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli_parser::check(&args).unwrap_or_else(|err| abnormal_termination(err));

    let (cmd, file_path) = (args.get(1).unwrap(), args.get(2).unwrap());
    file_checker::process(cmd, file_path).unwrap_or_else(|err| abnormal_termination(err));

    match msg_sender::send_grpc_msg(cmd).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().desc),
        Err(t) => abnormal_termination(t),
    }
}
