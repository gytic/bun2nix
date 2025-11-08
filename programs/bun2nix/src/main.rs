//! Main entry point for the `bun2nix` command line tool, which makes calls to the library for the
//! majority of the actual processing

#![warn(missing_docs)]

use bun2nix::convert_lockfile_to_nix_expression;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clap::Parser;
use env_logger::Env;

/// Convert Bun (v1.2+) packages to Nix expressions
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The Bun (v1.2+) lockfile to use to produce the Nix expression.
    #[arg(short, long, default_value = "./bun.lock")]
    lock_file: PathBuf,

    /// The output file to write to -
    /// if no file location is provided, print to stdout instead.
    #[arg(short, long)]
    output_file: Option<PathBuf>,
}

fn main() {
    let log_env = Env::default().default_filter_or("warn");
    env_logger::Builder::from_env(log_env).init();

    let cli = Cli::parse();

    let lockfile = fs::read_to_string(&cli.lock_file)
        .unwrap_or_else(|_| panic!("\nCould not find lockfile at {}.\nTry changing the file path to point to one, or create one with `bun install` on a version of bun above v1.2.\nSee https://bun.sh/docs/install/lockfile to find out more information about the textual lockfile.\n\nTry `bun2nix -h` for help.\n", cli.lock_file.to_str().unwrap()));

    let nix = convert_lockfile_to_nix_expression(lockfile).unwrap();

    match cli.output_file {
        Some(output_file) => {
            let mut output = File::create(output_file).unwrap();
            write!(output, "{}", nix).unwrap();
        }
        None => println!("{}", nix),
    };
}
