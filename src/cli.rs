use clap::Parser;
use crate::log::LogLevel;

/// Simple program to parse C code and print variable types
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the directory containing project files
    #[arg(short, long, required = true)]
    pub dir: String,

    /// Type to unroll
    #[arg(short, long, required = true)]
    pub typ: String,

    /// Log level (optional)
    #[arg(short, long, value_enum, default_value_t = LogLevel::Error)]
    pub log_level: LogLevel,
}
