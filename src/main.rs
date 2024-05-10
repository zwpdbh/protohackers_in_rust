mod command_line;
mod protohacker;

use clap::Parser;
use command_line::*;

#[tokio::main()]
async fn main() {
    let _ = setup_simple_tracing();
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Ch0Server { port } => {
            let _ = protohacker::ch0_echo_service::server_run(port).await;
        }
        SubCommand::Ex01 { id } => {
            println!("id: {}", id)
        }
        SubCommand::Ex02 { case } => match case {
            ExCase::Case01 { name } => {
                println!("name: {}", name)
            }
            ExCase::Case02 => {
                println!("case02")
            }
        },
        SubCommand::Ex03 { case: _case } => {
            println!("use ValueEnum trait is useful")
        }
    }
}

/// tracing = {version = "0.1"}
/// tracing-subscriber = { version = "0.3", features = ["json"] }
pub fn setup_simple_tracing() {
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
