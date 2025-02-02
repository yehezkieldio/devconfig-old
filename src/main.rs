use clap::Parser;

#[derive(Parser)]
#[command(name = "amaris")]
#[command(author = "elizielx")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Automate opinionated development configurations.", long_about = None)]
struct CLI {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli: CLI = CLI::parse();

    // Use arguments
    if cli.verbose {
        tracing::info!("Verbose mode enabled");
    }

    Ok(())
}
