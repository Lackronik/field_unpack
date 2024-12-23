mod cli;
mod file_manager;
mod parser;
mod unroll;
mod log;

use cli::Args;
use log::init_logger;
use clap::Parser;
use file_manager::collect_files;
use parser::parse_files;
use unroll::unroll_type;

fn main() {
    let args = Args::parse();

    // Set the log level.
    init_logger(args.log_level);
    LOG_INF!("Program started with log level: {:?}", args.log_level);

    // Collect all .c and .h files in the specified directory
    let files = collect_files(&args.dir);
    LOG_INF!("Found files: {:?}", files);

    // Parse the contents of the files
    let parsed_types = parse_files(files);

    // Unroll the specified type
    if let Some(unrolled) = unroll_type(&args.typ, &parsed_types) {
        println!("{}", unrolled);
    } else {
        println!("Type {} not found", args.typ);
    }
}
