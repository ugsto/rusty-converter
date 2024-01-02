use args::{Args, Commands};
use clap::Parser;
use commands::{convert::convert, list::list};

mod args;
mod commands;
mod currency_code;
mod errors;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args {
        Args {
            command: Commands::Convert { amount, from, to },
        } => convert(amount, from, to).await,
        Args {
            command: Commands::List,
        } => list(),
    }
}
