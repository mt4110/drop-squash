use std::process::ExitCode;

use clap::Parser;

mod args;
mod run;

#[tokio::main]
async fn main() -> ExitCode {
    match run::run(args::Cli::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
