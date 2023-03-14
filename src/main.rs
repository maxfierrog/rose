// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod utils;
pub mod build;
pub mod init;
pub mod help;


use clap::{Args, Parser, Subcommand};
use help::error::error_102;
use utils::folder_exists;
use std::process;


/* COMMAND LINE INTERFACE */

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Send no output to STDOUT during execution
    #[arg(short, long, group = "out")]
    quiet: bool,
    /// Provide extra execution information to STDOUT
    #[arg(short, long, group = "out")]
    verbose: bool,
    /// Skips prompts for confirming destructive operations
    #[arg(short, long)]
    yes: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Create a documentation tree based on a canonical JSON file
    Tree(TreeArgs),
    /// Create a documentation file based on a canonical JSON file
    File(FileArgs),
    /// Create a default JSON doctree and docfile specifier file
    Init(InitArgs)
}


/* BUILD COMMANDS */

#[derive(Args)]
struct TreeArgs {
    /// Project directory path (optional)
    #[arg(short, long)]
    project: Option<String>,
    /// Destination directory
    #[arg(short, long)]
    dest: String
}

#[derive(Args)]
struct FileArgs {
    /// Source code filepath (optional)
    #[arg(short, long)]
    source: Option<String>,
    /// Destination directory
    #[arg(short, long)]
    dest: String
}

#[derive(Args)]
struct InitArgs {
    /// Target directory path
    #[arg(short, long)]
    target: String
}


/* PROGRAM ENTRY POINT */

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Tree(args) => {

        },
        Commands::File(args) => {

        },
        Commands::Init(args) => {
            init(&args.target, cli.quiet, cli.verbose);
        }
    }
    process::exit(exitcode::OK);
}


/* EXECUTORS */

/// Verifies that the target directory passed into init exists and processes
/// initialization request
fn init(path: &String, quiet: bool, verbose: bool) {
    if !folder_exists(path) { error_102(); }

}

