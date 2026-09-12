use crate::{InputType, TargetType, validate::*};
use clap::*;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "mcp-security")]
#[command(version, author, about = "MCP Server Security Scanner", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Scan(Args),
    Other,
}

#[derive(Args, Debug)]
pub struct Args {
    #[arg(short, long, value_parser = validate_dir)]
    pub target: Option<PathBuf>,

    #[arg(short = 'm', long, default_value = "console")]
    pub command: Option<String>,

    #[arg(short, long, value_parser = validate_conf)]
    pub config: Option<PathBuf>,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(long, value_parser = validate_timeout)]
    pub timeout: Option<u64>,

    #[arg(short, long)]
    pub format: Option<TargetType>,

    #[arg(long)]
    pub no_instrospection: bool,

    #[arg(short, long)]
    pub include: Option<String>,

    #[arg(short, long)]
    pub exclude: Option<String>,

    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    pub fn get_input(&self) -> Vec<(InputType, String)> {
        let mut input = Vec::new();

        if self.target.is_some() {
            let target_str = self.target.clone().unwrap().to_string_lossy().into_owned();
            input.push((InputType::TARGET, target_str));
        }
        if self.command.is_some() {
            input.push((InputType::COMMAND, self.command.clone().unwrap()));
        }
        if self.config.is_some() {
            let config_str = self.config.clone().unwrap().to_string_lossy().into_owned();
            input.push((InputType::CONFIG, config_str));
        }

        input
    }
}

#[cfg(test)]
mod test {
    use clap::Parser;

    #[test]
    fn test_call_cli() {
        let args = vec!["mcp-security", "scan", "--target", "src/test/py"];
        let cli = super::Cli::try_parse_from(args);
        println!("\nTest CLI: {:?}\n", cli);
    }
}
