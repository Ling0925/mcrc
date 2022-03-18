use crate::mcrc_core::launcher::launcher;
use clap::Parser;
mod mcrc_core;

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    if args.online {
        println!("soon...");
    }
    println!("{}", launcher());

    println!("{:?}", args);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    online: bool,

    #[clap(short, long, default_value = "player")]
    name: String,

    #[clap(short, long, default_value = "./")]
    game_directory: String,
}
