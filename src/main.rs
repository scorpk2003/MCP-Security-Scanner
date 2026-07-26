mod risk;
mod model;
mod mcp;
mod targets;
mod severity;
mod reports;
mod rules;
mod cli;

use std::env;

use clap::Parser;
pub use cli::*;
pub use rules::*;
pub use reports::*;
pub use severity::*;
pub use targets::*;
pub use mcp::*;
pub use model::*;
pub use risk::*;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let args = match cli.command {
        Cmd::Scan(req) => {
            req
        },
        Cmd::Other => panic!("Don't reconized command")
    };
    let request = ScanRequest::mapping_args(&args);
    println!("Args: {:?}", request);
}
