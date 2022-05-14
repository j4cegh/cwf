use std::env;
use std::process::Command;

mod cli;
mod html;
mod ts;
mod web;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: cwf <command>");
        return;
    }

    cli::run_cli(&args);
}
