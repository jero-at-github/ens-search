// Reference:
// https://docs.ens.domains/contract-api-reference/name-processing
// https://eips.ethereum.org/EIPS/eip-137

use clap::{Parser, Subcommand};
use ens_search::{process_file, process_is_confirm};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file
    File { path: String },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::File { path } => {
            if process_is_confirm() {
                process_file(path.into()).await;
            } else {
                println!("Execution aborted.")
            }
        }
    }
}
