// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod utils;
pub mod remove;
pub mod build;
pub mod check;
pub mod help;
pub mod add;


use std::env;
use core::panic;
use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
   #[arg(short, long)]
   source: String,
   
   #[arg(short, long)]
   destination: String,

   #[arg(short, long)]
   canon: String,
}



/* GLOBAL VALUES */

/* Command names (should be in plaintext). */
const COMMANDS: [&str; 4] = ["doctree", "docfile", "add", "remove"];

/* Full argument options (should be in the format "--{}="). */
const LONG_OPTIONS: [&str; 3] = ["source", "dest", "canon"];

/* Shorthands for argument options (should be in the format "-{}="). */
const SHORT_OPTIONS: [&str; 3] = ["s", "d", "c"];

/* Full flags (should be prefixed with "--"). */
const LONG_FLAGS: [&str; 3] = ["quiet", "verbose", "yes"];

/* Shorthands for flags (should be prefixed with "-"). */
const SHORT_FLAGS: [&str; 3] = ["q", "v", "y"];


/* ENTRY */

/* Main I/O loop. There are two modes for this program: 1) standard UNIX CLI
command style, where you pass in a command with options and arguments, and 2)
an interactive full-screen interface for ease of use. The interactive mode
is activated when only 'rose' is executed. Otherwise, we assume that the user
wanted to execute a UNIX style command, and display help and whatnot. */
fn main() {
    let args: Vec<_> = env::args().collect();
    let mode: bool = args.len() == 1;
    if mode {
        clear_screen();
        loop {
            action_menu();
            clear_screen();
            if finished_actions() { break; }
        };
    } else {
        let args = Args::parse();
        handle_arguments(args);
    }
}


/* COMMAND MODE ARGUMENT HANDLING */

/* Accepts the arguments handed to the program and executes the appropriate
action without printing anything, or exits with the correct error code should
something go wrong in the process (also without printing anything). */
fn handle_arguments(args: Args) {

}
