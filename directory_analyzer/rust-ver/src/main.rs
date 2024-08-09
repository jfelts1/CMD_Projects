use clap::Parser;
use dira::{analyze, output, Args};

fn main() {
    let args = Args::parse();
    let analyzed_info = analyze(&args);
    let out = output(&args, analyzed_info);
    match out {
        Ok(out) => println!("{out}"),
        Err(e) => eprintln!("{e}"),
    }
}
