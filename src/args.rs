use crate::currency_code::CurrencyCode;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Convert {
        amount: f64,
        from: CurrencyCode,
        to: CurrencyCode,
    },
    List,
}
