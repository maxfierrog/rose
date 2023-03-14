// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod utils;
pub mod remove;
pub mod build;
pub mod check;
pub mod help;
pub mod add;


use clap::{Args, Parser, Subcommand};


/* COMMAND LINE INTERFACE */

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Send no output to STDOUT during execution
    #[arg(short, long)]
    quiet: bool,
    /// Provide extra execution information to STDOUT
    #[arg(short, long)]
    verbose: bool,
    /// Skips prompts for confirming destructive operations
    #[arg(short, long)]
    yes: bool,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Uses canon JSON to create a documentation tree with many individual 
    /// files from a project's source code, or otherwise a simple directory 
    /// structure
    Tree(TreeArgs),
    /// Uses canon JSON to create a single file containing either a document
    /// template or documentation for a source code file
    File(FileArgs),
    /// Creates a default JSON doctree and docfile specifier, 'canon.json',
    /// in the target directory
    Init(InitArgs)
}


/* COMMANDS */

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

        }
    }
}
