use std::env::args;
use std::io;

use clap::Command;
use clap_complete::{Generator, generate};

pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    let bin_name = args()
        .nth(0)
        .unwrap_or_else(|| cmd.get_bin_name().unwrap().to_owned());

    let file_name = bin_name.split('/').last().unwrap_or(&bin_name);

    generate(generator, cmd, file_name, &mut io::stdout());
}
