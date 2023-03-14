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
    /// Skips prompts for destructive operations
    #[arg(short, long)]
    yes: bool,
    #[command(subcommand)]
    command: Commands
}


#[derive(Subcommand)]
enum Commands {
    /// Create a documentation tree with many individual files
    Doctree(MakeTree),
    /// Create a single documentation file
    Docfile(MakeFile),
    /// Creates a default JSON doctree and docfile specifier in target
    Init(Init)
}

#[derive(Args)]
struct MakeTree {
    /// If provided, generates doctree based on project's source code
    #[arg(short, long)]
    project: Option<String>,

    /// Destination directory for documentation tree
    #[arg(short, long)]
    dest: String
}

#[derive(Args)]
struct MakeFile {
    /// If provided, generates document based on file's source code
    #[arg(short, long)]
    source: Option<String>,

    /// Destination directory for document
    #[arg(short, long)]
    dest: String
}

#[derive(Args)]
struct Init {
    /// Destination directory for JSON architecture spec
    #[arg(short, long)]
    target: String
}


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Doctree(MakeTree) => {

        },
        Commands::Docfile(MakeFile) => {

        },
        Commands::Init(Init) => {

        }
    }
}
