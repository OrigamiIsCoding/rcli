
use std::{fs::File};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args{
    #[arg(index = 1)]
    name: String,
}

fn main() {
    let args = Args::parse();
    
    if let Err(err) = File::create(args.name) {
        eprintln!("Error creating file: {:?}", err);
    }
}
