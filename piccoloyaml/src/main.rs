use clap::Parser;

mod cli_parser;
mod file_handler;
mod msg_sender;

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

    match msg_sender::send_grpc_msg(cmd).await {
        Ok(t) => println!("- SUCCESS -\n{}", t.into_inner().desc),
        Err(t) => abnormal_termination(t),
    }
}
