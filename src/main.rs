mod test_contract;
mod storage;
use clap::Parser;

// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    // Name of the person to greet
    #[clap(short, long)]
    name: String,

    // Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    test_contract::init();

    // TODO: Create PAC token

    // TODO: Deploy contract

    // TODO: Start server

    // TODO: Connect to other servers

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
