use std::io;
use clap::Parser;
use muslimdb::util::clap::Args;

fn main() -> io::Result<()> {
	let args = Args::parse();
 
	println!("Hello muslimdb.");    
    Ok(())
}
