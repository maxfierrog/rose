// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod utils;
pub mod remove;
pub mod build;
pub mod check;
pub mod add;


use core::panic;
use terminal_menu::{menu, label, button, scroll, run, mut_menu, has_exited};
use remove::remover_menu;
use build::builder_menu;
use check::checker_menu;
use add::adder_menu;
use utils::*;


/* MAIN FUNCTION */

/* Main I/O loop -- allows user to do more actions if they are not finished
after taking one. */
fn main() {
    clear_screen();
    loop {
        action_menu();
        if finished_actions() { break; }
    }
}


/* MAIN MENU FUNCTIONS */

/* Asks user if they are finished taking actions. */
fn finished_actions() -> bool {
    let menu = menu(vec![
        label(" "),
        label("Are you finished taking actions?"),
        label(" "),
        button("Yes"),
        button("No")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    match mm.selected_item_name() {
        "Yes"   => true,
        "No"    => false,
        _ => panic!("Unsafe input.")
    }
}

/* Prompts user to either Build, Check, Add, or Remove either code or
documentation. This is the first menu users see. */
fn action_menu() {
    print_welcome();
    let menu = menu(vec![
        label(" "),
        scroll("Action:", vec!["Build", "Check", "Add", "Remove"]),
        label(" "),
        button("SELECT")
    ]);
    run(&menu);
    clear_screen();
    if !has_exited(&menu) {
        let mm = mut_menu(&menu);
        match mm.selection_value("Action:") {
            "Build"     => { builder_menu() },
            "Check"     => { checker_menu() },
            "Add"       => { adder_menu()   },
            "Remove"    => { remover_menu() },
            _ => panic!("Unsafe prompt input.")
        }
    }
}


/* HELPER METHODS */

/* Prints a welcome message and returns the amount of new lines it
printed so they can be deleted later. */
fn print_welcome() {
    println!("");
    println!("Welcome!");
    println!("");
    println!("Rose helps you out with your documentation needs.");
    println!("");
    println!("Choose:\tW, A, S, D, ARROWS");
    println!("Select:\tENTER");
    println!("Exit:\tQ, ESC");
    println!("");
    println!("------------------------------------------");
}
