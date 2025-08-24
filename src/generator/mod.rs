use std::env::args;
use std::io;

use clap::Command;
use clap_complete::{Generator, generate};

pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        args()
            .nth(0)
            .unwrap_or_else(|| cmd.get_bin_name().unwrap().to_owned()),
        &mut io::stdout(),
    );
}
