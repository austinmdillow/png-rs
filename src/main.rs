


mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Cli;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count:u8
}

fn main() -> Result<()> {

    let args = Cli::parse();

    

    Ok(())
}
