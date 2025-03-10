use bun2nix::convert_lockfile_to_nix_expression;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::LazyLock,
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
    #[arg(short, long, default_value = &*DEFAULT_CACHE_DIR)]
    cache: PathBuf,

    /// Disable creating or writing to the cache
    #[arg(long, default_value_t = false)]
    no_cache: bool,
}

static DEFAULT_CACHE_DIR: LazyLock<String> = LazyLock::new(|| {
    let system_cache_dir = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);

    system_cache_dir
        .join("bun2nix")
        .to_string_lossy()
        .into_owned()
});

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let lockfile = fs::read_to_string(&args.lock_file)
        .unwrap_or_else(|_| panic!("\nCould not find lockfile at {}.\nTry changing the file path to point to one, or create one with `bun install` on a version of bun above v1.2.\nSee https://bun.sh/docs/install/lockfile to find out more information about the textual lockfile.\n\nTry `bun2nix -h` for help.\n", args.lock_file.to_str().unwrap()));

    if !args.no_cache && !args.cache.exists() {
        File::create(&args.cache).unwrap();
    }

    let cache = match args.no_cache {
        false => Some(args.cache),
        true => None,
    };

    let nix = convert_lockfile_to_nix_expression(lockfile, cache)
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
