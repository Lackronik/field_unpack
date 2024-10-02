use clap::Parser;

/// Simple program to parse C code and print variable types
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the directory containing C project files
    #[arg(short, long)]
    pub dir: String,

    /// Type to unroll
    #[arg(short, long)]
    pub typ: String,
}
