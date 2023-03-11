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
use terminal_menu::{menu, label, button, scroll, run, mut_menu};
use remove::remover_menu;
use build::builder_menu;
use check::checker_menu;
use add::adder_menu;
use utils::{clear_screen};
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


/* MAIN MENU FUNCTIONS */

/* Asks user if they are finished taking actions. */
fn finished_actions() -> bool {
    print_exit_header();
    let menu = menu(vec![
        label(" "),
        label("Are you finished taking actions?"),
        label(" "),
        button("YES"),
        button("NO")
    ]);
    run(&menu);
    clear_screen();
    let mm = mut_menu(&menu);
    match mm.selected_item_name() {
        "YES"   => true,
        "NO"    => false,
        _ => panic!("Unsafe input.")
    }
}

/* Prompts user to either Build, Check, Add, or Remove either code or
documentation. This is the first menu users see. */
fn action_menu() {
    print_welcome_header();
    let menu = menu(vec![
        label(" "),
        scroll("Action:", vec!["Build", "Check", "Add", "Remove"]),
        label(" "),
        button("NEXT")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        match mm.selection_value("Action:") {
            "Build"     => { builder_menu(); },
            "Check"     => { checker_menu(); },
            "Add"       => { adder_menu();   },
            "Remove"    => { remover_menu(); },
            _ => panic!("Unsafe input.")
        };
    };
}


/* HELPER FUNCTIONS */

/* Prints a welcome message and returns the amount of new lines it
printed so they can be deleted later. */
fn print_welcome_header() {
    println!("");
    println!("Welcome!");
    println!("");
    println!("Rose helps you out with your documentation needs.");
    println!("");
    println!("Choose:\tW, A, S, D, ARROWS");
    println!("Select:\tENTER");
    println!("Exit:\tQ, ESC");
    println!("-------------------------------------------------");
}

/* Prints a message asking user if they wish to return to starter
menu or exit the tool. */
fn print_exit_header() {
    println!("");
    println!("Exit or return to starter menu...");
    println!("-------------------------------------------------");
}
