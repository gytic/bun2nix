use bun2nix::convert_lockfile_to_nix_expression;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clap::Parser;

/// Convert Bun (v1.2+) packages to Nix expressions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Bun (v1.2+) lockfile to use to produce the Nix expression.
    #[arg(short, long, default_value = "./bun.lock")]
    lock_file: PathBuf,

    /// The output file to write to -
    /// if no file location is provided, print to stdout instead.
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// The sqlite database to use as the cache -
    /// will be created if it does not exist.
    /// Using `None` will mean no caching will be done
    #[arg(short, long, default_value = Some("~/.cache/bun2nix"))]
    cache: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let lockfile = fs::read_to_string(&args.lock_file)
        .unwrap_or_else(|_| panic!("\nCould not find lockfile at {}.\nTry changing the file path to point to one, or create one with `bun install` on a version of bun above v1.2.\nSee https://bun.sh/docs/install/lockfile to find out more information about the textual lockfile.\n\nTry `bun2nix -h` for help.\n", args.lock_file.to_str().unwrap()));

    let nix = convert_lockfile_to_nix_expression(lockfile, args.cache)
        .await
        .unwrap();

    match args.output_file {
        Some(output_file) => {
            let mut output = File::create(output_file).unwrap();
            write!(output, "{}", nix).unwrap();
        }
        None => println!("{}", nix),
    };
}
