use std::io;

use clap::Command;
use clap_complete::{generate, Generator};


pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}
