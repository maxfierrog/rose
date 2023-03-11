// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod build_tree;
pub mod build_file;


use terminal_menu::{label, menu, button, string, run, mut_menu, scroll};
use build_tree::{docs_from_canon_check, docs_from_project_check};
use build_file::{docfile_from_code_check, docfile_from_both_check, docfile_from_template_check};


/* BUILDER MENU*/

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
        button("NEXT")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        match mm.selection_value("Build:") {
            "Documentation File" => { docfile_menu(); },
            "Documentation Tree" => { doctree_menu(); },
            _ => panic!("Unsafe input.")
        };
    };
}


/* DOCTREE BUILDER MENU */

/* Allows user to create a directory structure outlined by an interpreted JSON
document, or a documentation tree based on a project's source code. */
fn doctree_menu() {
    print_doctree_menu_header();
    let menu = menu(vec![
        label(" "),
        scroll("Source type:", vec!["Project Directory", "Canonical File"]),
        string("Source path:", "<project folder or canon JSON>", false),
        string("Destination:", "<destination folder>", false),
        label(" "),
        button("CONFIRM")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        let source = mm.selection_value("Source path:");
        let dest = mm.selection_value("Destination:");
        match mm.selection_value("Source type:") {
            "Project Directory" => { 
                docs_from_project_check(false, source, dest); 
            },
            "Canonical File" => { 
                docs_from_canon_check(false, source, dest);   
            },
            _ => panic!("Unsafe input.")
        };
    };
}


/* DOCFILE BUILDER MENUS */

/* Allows the user to create a single documentation file, providing the option
of specifying a template and a source code file. If no template is specified,
the generated documentation is placed in an empty file in a standard way. If
a template is specified, the generated documentation is fit into the structure
outlined by the template. */
fn docfile_menu() {
    print_docfile_menu_header();
    let menu = menu(vec![
        label(" "),
        scroll("From:", vec!["Source Code File", "Template File", "Both"]),
        label(" "),
        button("NEXT")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        match mm.selection_value("From:") {
            "Source Code File" => { docfile_code_menu();     },
            "Template File"    => { docfile_template_menu(); },
            "Both"             => { docfile_both_menu();     },
            _ => panic!("Unsafe input.")
        };
    };
}

/* Specifically for constructing a docfile from a single file, without
considering a template to fit it into. */
fn docfile_code_menu() {
    print_docfile_code_menu_header();
    let menu = menu(vec![
        label(" "),
        string("Source code:", "<filepath>", false),
        string("Destination:", "<destination folder>", false),
        label(" "),
        button("CONFIRM")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        let source = mm.selection_value("Source code:");
        let dest = mm.selection_value("Destination:");
        docfile_from_code_check(source, dest);
    };
}

/* Specifically for constructing a copy of a template, without considering
a source code file to popluate it with. */
fn docfile_template_menu() {
    print_docfile_template_menu_header();
    let menu = menu(vec![
        label(" "),
        string("JSON spec:", "<filepath>", false),
        string("Destination:", "<destination folder>", false),
        label(" "),
        button("CONFIRM")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        let source = mm.selection_value("JSON spec:");
        let dest = mm.selection_value("Destination:");
        docfile_from_template_check(source, dest);
    };
}

/* For documenting source code by fitting it into a template. */
fn docfile_both_menu() {
    print_docfile_both_menu_header();
    let menu = menu(vec![
        label(" "),
        string("JSON spec:", "<filepath>", false),
        string("Source code:", "<filepath>", false),
        string("Destination:", "<destination folder>", false),
        label(" "),
        button("CONFIRM")
    ]);
    run(&menu);
    let mm = mut_menu(&menu);
    if !mm.canceled() {
        let template = mm.selection_value("JSON spec:");
        let code = mm.selection_value("Source code:");
        let dest = mm.selection_value("Destination:");
        docfile_from_both_check(template, code, dest);
    };
}


/* HELPER FUNCTIONS */

/* Prints a pane header for orientation. */
fn print_builder_menu_header() {
    println!("");
    println!("Build a...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_doctree_menu_header() {
    println!("");
    println!("Documentation tree...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_docfile_menu_header() {
    println!("");
    println!("Documentation file...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_docfile_code_menu_header() {
    println!("");
    println!("From a source code file...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_docfile_template_menu_header() {
    println!("");
    println!("In a specific format...");
    println!("-------------------------------------------------");
}

/* Prints a pane header for orientation. */
fn print_docfile_both_menu_header() {
    println!("");
    println!("In a specific format, from a source code file...");
    println!("-------------------------------------------------");
}
