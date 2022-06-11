mod cmd;

use anyhow::Result;
use colored::Colorize;
use clap::{
    Parser, Subcommand
};

use self::{
    cmd::{
        HelloCommand,
        InstantiateCommand,
    }
};

#[derive(Debug, Parser)]
#[clap(name = "tx_loader", author, about, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Setup and create a new smart contract project
    #[clap(name = "hello")]
    Hello(HelloCommand),
    #[clap(name="instantiate")]
    Instantiate(InstantiateCommand),
}

fn exec(cmd: Command) -> Result<()> {
    match cmd {
        Command::Hello(cmd) => {
            cmd.exec()?;
            Ok(())
        }
        Command::Instantiate(cmd) => {
            cmd.exec()?;
            Ok(())
        }
    }
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();
    match exec(cli.cmd) {
        Ok(()) => {}
        Err(err) => {
            eprintln!(
                "{} {}",
                "ERROR:".bright_red().bold(),
                format!("{:?}", err).bright_red()
            );
            std::process::exit(1);
        }
    }
}
