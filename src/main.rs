// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod constants;
pub mod utils;
pub mod build;
pub mod help;


use std::process;
use build::{
    build_file::*, 
    build_tree::*, 
    init::*
};
use clap::{
    Args, 
    Parser, 
    Subcommand
};
use help::error::{
    error_102, 
    error_101, 
    error_201, 
    error_202
};
use utils::{
    folder_exists, 
    file_exists, 
    json_canon_ok, 
    json_canon_exists
};


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
    /// Generate documentation or templates
    #[command(subcommand)]
    Build(BuildCommands),
    /// Create a default JSON doctree and docfile specifier file
    Init(InitArgs)
}

#[derive(Subcommand)]
enum BuildCommands {
    /// Create a documentation tree based on a canonical JSON file
    Tree(BuildTreeArgs),
    /// Create a documentation file based on a canonical JSON file
    File(BuildFileArgs),
}


/* GENERATIVE COMMANDS */

#[derive(Args)]
struct BuildTreeArgs {
    /// Project directory path (optional)
    #[arg(short, long)]
    project: Option<String>,
    /// Destination directory
    #[arg(short, long)]
    dest: String
}

#[derive(Args)]
struct BuildFileArgs {
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


/* NEUTRAL COMMANDS */

// TODO


/* DESTRUCTIVE COMMANDS */

// TODO


/* PROGRAM ENTRY POINT */

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build(cmd) => {
            match cmd {
                BuildCommands::Tree(args) => {
                    tree(&args.project, &args.dest, cli.quiet, cli.verbose);
                },
                BuildCommands::File(args) => {
                    file(&args.source, &args.dest, cli.quiet, cli.verbose);
                }
            }
        },
        Commands::Init(args) => {
            init(&args.target, cli.quiet, cli.verbose);
        }
    }
    process::exit(exitcode::OK);
}


/* CHECKING AND HANDLING FOR GENERATIVE COMMANDS */

/// Verifies that the target directory passed into init exists and processes
/// initialization request by inserting a JSON canon template into target
fn init(target: &String, quiet: bool, verbose: bool) {
    if !folder_exists(target) { error_102(); }
    if json_canon_exists(target) { error_202(); }
    create_canon(target, quiet, verbose);
}

/// Verifies that input source and destination are valid, and handles different
/// use cases (creating document from source code or as template)
fn file(source: &Option<String>, dest: &String, quiet: bool, verbose: bool) {
    if !folder_exists(dest) { error_102(); }
    if !json_canon_ok() { error_201(); }
    if let Some(source) = source {
        if !file_exists(source) { error_101(); }
        file_from_code(source, dest, quiet, verbose);
    }
    file_from_canon(dest, quiet, verbose);
}

/// Verifies that input source and destination are valid, and handles different
/// use cases (creating tree from project directory or as empty structure)
fn tree(project: &Option<String>, dest: &String, quiet: bool, verbose: bool) {
    if !folder_exists(dest) { error_102(); }
    if !json_canon_ok() { error_201(); }
    if let Some(source) = project {
        if !file_exists(source) { error_101(); }
        tree_from_project(source, dest, quiet, verbose);
    }
    tree_from_canon(dest, quiet, verbose);
}


/* CHECKING AND HANDLING FOR NEUTRAL COMMANDS */

// TODO


/* CHECKING AND HANDLING FOR DESTRUCTIVE COMMANDS */

// TODO

