mod parser;
mod database;

use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
	#[arg(short, long)]
	verbose: bool,
	options: String,
	file: String,
}

fn main() -> io::Result<()> {
	let args = Args::parse();
 
	println!("options: {}\nfile: {}\nverbose: {}", args.options, args.file, args.verbose);    
    Ok(())
}
