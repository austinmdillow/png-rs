use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(author="Me", version="1.2", about="Utility for png encoding/decoding", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,

    // NUmber
    #[arg(short, long, group = "input")]
    pub top_level_option: String
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}


#[derive(Parser, Debug)]
pub struct EncodeArgs {
    pub file_path: String,

    pub chunk_type: String,
    pub message: String,

    pub output_file: Option<String>,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub file_path: String,

    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub file_path: String,

    pub chunk_type: String,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub file_path: String,
}