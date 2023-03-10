// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

use terminal_menu::{label, menu, button, run, mut_menu, scroll, has_exited};


/* MAIN MODULE MENU */

/* Allows the user to create a single documentation file, providing the option
of specifying a template and a source code file. If no template is specified,
the generated documentation is placed in an empty file in a standard way. If
a template is specified, the generated documentation is fit into the structure
outlined by the template. */
pub fn builder_menu() {
    let menu = menu(vec![
        label(" "),
        scroll("Build:", vec!["Documentation File", "Documentation Tree"]),
        label(" "),
        button("SELECT")
    ]);
    run(&menu);
    if !has_exited(&menu) {
        let mm = mut_menu(&menu);
        match mm.selection_value("Build:") {
            "Documentation File" => { make_documentation_file() },
            "Documentation Tree" => { make_documentation_tree() },
            _ => panic!("Unsafe prompt input.")
        }
    }
}


/* DELEGATE METHODS */

fn make_documentation_file() {

}

fn make_documentation_tree() {

}