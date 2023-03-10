// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod build_tree;
pub mod build_file;


use terminal_menu::{label, menu, button, string, run, mut_menu, scroll};
use build_tree::{docs_from_canon, docs_from_project};

/* MAIN MODULE MENU */

/* Allows the user to create a single documentation file, providing the option
of specifying a template and a source code file. If no template is specified,
the generated documentation is placed in an empty file in a standard way. If
a template is specified, the generated documentation is fit into the structure
outlined by the template. */
pub fn builder_menu() {
    print_builder_menu_header();
    let menu = menu(vec![
        label(" "),
        scroll("Build:", vec!["Documentation File", "Documentation Tree"]),
        label(" "),
        button("SELECT")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        match mm.selection_value("Build:") {
            "Documentation File" => { make_documentation_file(); },
            "Documentation Tree" => { make_documentation_tree(); },
            _ => panic!("Unsafe input.")
        };
    };
}


/* DELEGATE METHODS */

/* Allows the user to create a single documentation file, providing the option
of specifying a template and a source code file. If no template is specified,
the generated documentation is placed in an empty file in a standard way. If
a template is specified, the generated documentation is fit into the structure
outlined by the template. */
fn make_documentation_file() {
    print_docfile_menu_header();
    let menu = menu(vec![
        label(" "),
        scroll("Source type:", vec!["Project Directory", "Canonical File"]),
        string("Source path:", "<to project or structure file>", false),
        string("Destination:", "<path to destination folder>", false),
        label(" "),
        button("CONFIRM")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        let source = mm.selection_value("Source path:");
        let dest = mm.selection_value("Destination:");
        match mm.selection_value("Source type:") {
            "Project Directory" => { docs_from_project(source, dest); },
            "Canonical File"    => { docs_from_canon(source, dest); },
            _ => panic!("Unsafe input.")
        };
    };
}

/* Allows user to create a directory structure outlined by an interpreted JSON
document, or a documentation tree based on a project's source code. */
fn make_documentation_tree() {
    print_doctree_menu_header();
    let menu = menu(vec![
        label(" "),
        scroll("Source type:", vec!["Project Directory", "Canonical File"]),
        string("Source path:", "<to project or structure file>", false),
        string("Destination:", "<path to destination folder>", false),
        label(" "),
        button("CONFIRM")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        let source = mm.selection_value("Source path:");
        let dest = mm.selection_value("Destination:");
        match mm.selection_value("Source type:") {
            "Project Directory" => { docs_from_project(source, dest); },
            "Canonical File"    => { docs_from_canon(source, dest); },
            _ => panic!("Unsafe input.")
        };
    };
}


/* HELPER METHODS */

/* Prints a pane header for orientation. */
fn print_builder_menu_header() {
    println!("");
    println!("Builder menu...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_doctree_menu_header() {
    println!("");
    println!("Documentation tree builder menu...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_docfile_menu_header() {
    println!("");
    println!("Documentation file builder menu...");
    println!("-------------------------------------------------");
}