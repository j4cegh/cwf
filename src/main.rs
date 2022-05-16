use std::env;

mod cli;
mod html;
mod ts;
mod web;
mod css;
mod dist;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: cwf <command>");
        return;
    }

    cli::run_cli(&args);
}
