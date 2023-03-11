// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod utils;
pub mod remove;
pub mod build;
pub mod check;
pub mod add;
pub mod errors;

use std::env;
use core::panic;
use terminal_menu::{menu, label, button, scroll, run, mut_menu};
use remove::remover_menu;
use build::builder_menu;
use check::checker_menu;
use add::adder_menu;
use utils::*;


/* GLOBAL VALUES */

/* Full command options */
const LONG_OPTIONS: [&str; 1] = ["-silent"];

/* Shorthands for command options */
const SHORT_OPTIONS: [&str; 1] = ["-S"];


/* ENTRY */

/* Main I/O loop -- allows user to do more actions if they are not finished
after taking one. */
fn main() {
    let args: Vec<String> = env::args().collect();
    let mode: bool = args.len() == 1;
    if mode {
        clear_screen();
        loop {
            action_menu();
            clear_screen();
            if finished_actions() { break; }
        };
    } else {
        handle_arguments(args);
    }
}


/* COMMAND MODE ARGUMENT HANDLING */

/* Accepts the arguments handed to the program and executes the appropriate
action without printing anything, or exits with the correct error code should
something go wrong in the process (also without printing anything). */
fn handle_arguments(args: Vec<String>) {

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
