use clap::Parser;
use std::io::Write;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    // Print the greeting to a file.
    let mut file = std::fs::File::create("greeting.txt").unwrap();
    for _ in 0..args.count {
        writeln!(file, "Hello, {}!", args.name).unwrap();
    }
}
