use bun2nix::convert_lockfile_to_nix_expression;

use std::fs;

use clap::Parser;

/// Convert Bun (v1.2+) packages to Nix expressions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Bun (v1.2+) lockfile to use to produce the Nix expression
    #[arg(short, long, default_value_t = String::from("./bun.lock"))]
    lockfile: String,
}

fn main() {
    let args = Args::parse();

    let lockfile = fs::read_to_string(&args.lockfile)
        .unwrap_or_else(|_| panic!("Could not find lockfile at {}. Try changing the file path to point to one, or create one with `bun install` on a version of bun above v1.2. See https://bun.sh/docs/install/lockfile to find out more information about the textual lockfile.", args.lockfile));

    let nix = convert_lockfile_to_nix_expression(lockfile);
    println!("nix: {:#?}", nix);
}
