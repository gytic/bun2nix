//! Main entry point for the `bun2nix` command line tool, which makes calls to the library for the
//! majority of the actual processing

#![warn(missing_docs)]

use bun2nix::{convert_lockfile_to_nix_expression, Cache, Result};

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::LazyLock,
};

use clap::{Args, Parser, Subcommand};

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

    /// The sqlite database to use as the cache -
    /// will be created if it does not exist.
    ///
    /// Default value of <system cache directory>/bun2nix is assigned when no value is passed to `cache_location`.
    #[arg(short, long, default_value = &*DEFAULT_CACHE_DIR)]
    cache_location: Option<PathBuf>,

    /// Disable usage of the cache entirely
    #[arg(long)]
    disable_cache: bool,

    #[command(subcommand)]
    sub_command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
/// Subcommands for the command line tool
pub enum Commands {
    /// Cache related subcommands
    Cache(CacheSubcommand),
}

#[derive(Debug, Args)]
/// Run queries on the `bun2nix` cache
pub struct CacheSubcommand {
    #[command(subcommand)]
    sub_command: CacheCommands,
}

#[derive(Debug, Subcommand)]
/// Commands which can be ran on the cache
pub enum CacheCommands {
    /// Empty the cache's data
    Clear,
    /// Print json of the data currently in the cache matching a given npm identifier
    #[command(visible_alias = "ls")]
    List {
        /// The npm identifier to search for in the cache
        npm_identifier: Option<String>,
    },
}

/// # Default Cache Directory
///
/// The default directory to use for this program's cache
pub static DEFAULT_CACHE_DIR: LazyLock<String> = LazyLock::new(|| {
    let system_cache_dir = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);

    system_cache_dir
        .join("bun2nix")
        .to_string_lossy()
        .into_owned()
});

#[tokio::main]
async fn main() {
    let mut cli = Cli::parse();

    if cli.disable_cache {
        cli.cache_location = None;
    }

    if let Some(ref cache) = cli.cache_location {
        if !cache.exists() {
            File::create(cache).unwrap();
        }
    }

    match cli.sub_command {
        Some(Commands::Cache(cache_cmd)) => {
            run_cache_command(cache_cmd.sub_command, cli.cache_location)
                .await
                .unwrap();
        }
        _ => run_bun2nix(cli).await,
    }
}

/// Main bun2nix command executor
async fn run_bun2nix(cli: Cli) {
    let lockfile = fs::read_to_string(&cli.lock_file)
        .unwrap_or_else(|_| panic!("\nCould not find lockfile at {}.\nTry changing the file path to point to one, or create one with `bun install` on a version of bun above v1.2.\nSee https://bun.sh/docs/install/lockfile to find out more information about the textual lockfile.\n\nTry `bun2nix -h` for help.\n", cli.lock_file.to_str().unwrap()));

    let nix = convert_lockfile_to_nix_expression(lockfile, cli.cache_location)
        .await
        .unwrap();

    match cli.output_file {
        Some(output_file) => {
            let mut output = File::create(output_file).unwrap();
            write!(output, "{}", nix).unwrap();
        }
        None => println!("{}", nix),
    };
}

/// Cache subcommand executor
async fn run_cache_command(cmd: CacheCommands, cache_location: Option<PathBuf>) -> Result<()> {
    let Some(location) = cache_location else {
        panic!("\nCache location must be set in order to run commands on the cache!\nEnsure `--no-cache` is not being passed as an option.\n");
    };

    let mut cache = Cache::new(location).await?;

    match cmd {
        CacheCommands::List { npm_identifier } => {
            let identifer = npm_identifier.unwrap_or_default();

            let pkgs = cache.list_cached_pkgs_by_npm_identifier(&identifer).await?;
            let out = serde_json::to_string_pretty(&pkgs)?;

            println!("{}", out);
        }
        CacheCommands::Clear => {
            cache.delete_all_cached_packages().await?;

            println!("Successfully deleted all packages from the bun2nix cache!");
        }
    }

    Ok(())
}
