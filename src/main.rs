mod cli;
mod file_manager;
mod parser;
mod unroll;

use cli::Args;
use clap::*;
use file_manager::collect_files;
use parser::parse_files;
use unroll::unroll_type;

fn main() {
    let args = Args::parse();

    // Collect all .c and .h files in the specified directory
    let files = collect_files(&args.dir);

    // Print found files for debugging
    println!("Found files: {:?}", files);

    // Parse the contents of the files
    let parsed_types = parse_files(files);

    // Unroll the specified type
    if let Some(unrolled) = unroll_type(&args.typ, &parsed_types) {
        println!("{}", unrolled);
    } else {
        println!("Type {} not found", args.typ);
    }
}
