use std::time::Instant;

use clap::Parser;
use dira::{analyze, output, Args};

fn main() {
    let args = Args::parse();
    let start_time = match args.time() {
        true => Some(Instant::now()),
        false => None,
    };
    let analyzed_info = analyze(&args);
    let out = output(&args, analyzed_info);
    match out {
        Ok(out) => println!("{out}"),
        Err(e) => eprintln!("{e}"),
    }
    if let Some(start_time) = start_time {
        let end_time = Instant::now();
        println!("Took {:.1} seconds", (end_time - start_time).as_secs_f64());
    }
}
