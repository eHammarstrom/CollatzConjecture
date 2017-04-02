
extern crate collatzconjecture;

use std::env;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    if args.len() != 3 {
        writeln!(stderr, "Usage: collatzconjecture METHOD NUMBER [OPTION]")
            .expect("Unable to write to stderr");
        std::process::exit(1);
    }

    let x = args[2].parse::<u32>().unwrap_or_else(|err| {
        writeln!(stderr, "2nd argument failed with: {}", err)
            .expect("Unable to write to stderr");
        std::process::exit(1);
    });

    if args[1] == "iterate" {
        println!("{:?}", collatzconjecture::up_to(x).len());
    } else if args[1] == "calculate" {
        println!("{:?}", collatzconjecture::number(x).len());
    }
}
