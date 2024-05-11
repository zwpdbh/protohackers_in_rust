mod client;
mod command_line;
mod server;

use clap::Parser;
use command_line::*;
use tracing::info;

#[tokio::main()]
async fn main() {
    let _ = setup_simple_tracing();
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Ch0Server { port } => {
            let _ = server::ch0_echo::server_run(port).await;
        }
        SubCommand::Ch0Client { port } => {
            let _ = client::client_run_v2(port).await;
        }
        SubCommand::Ex01 { id } => {
            info!("id: {}", id)
        }
        SubCommand::Ex02 { case } => match case {
            ExCase::Case01 { name } => {
                info!("name: {}", name)
            }
            ExCase::Case02 => {
                info!("case02")
            }
        },
        SubCommand::Ex03 { case: _case } => {
            info!("use ValueEnum trait is useful")
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
