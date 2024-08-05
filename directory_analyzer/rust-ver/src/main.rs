use clap::Parser;
use dira::{analyze, Args};

fn main() {
    let args = Args::parse();
    let output = analyze(&args);
    match output {
        Ok(out) => println!("{out}"),
        Err(e) => eprintln!("{e}"),
    }
}
