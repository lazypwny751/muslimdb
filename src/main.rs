use std::io;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
	#[]
	options: String,
	file: String,
}


fn main() -> io::Result<()> {
	let args = Args::parse();
 
	println!("Hello muslimdb.");    
    Ok(())
}
