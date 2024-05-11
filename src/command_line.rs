use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author = "zhaowei", version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ServerVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ClientVersion {
    V1,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    Ch0Server {
        #[arg(long)]
        port: u32,
        version: ServerVersion,
    },
    Ch0Client {
        #[arg(long)]
        port: u32,
        version: ClientVersion,
    },
    Ex01 {
        #[arg(short, long)]
        id: String,
    },
    Ex02 {
        #[clap(subcommand)]
        case: ExCase,
    },
    Ex03 {
        case: ValueEnumCase,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ExCase {
    Case01 {
        #[arg(short, long)]
        name: String,
    },
    Case02,
}
#[derive(Debug, Clone, ValueEnum)]
pub enum ValueEnumCase {
    Case01,
    Case02,
    Case03,
}
