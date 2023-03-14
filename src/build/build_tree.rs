// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

use std::path::Path;


/* FILE CHECKER FUNCTIONS */

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a canon JSON). */
pub fn docs_from_canon_check(silent: bool, source: &str, dest: &str) {
    let source = Path::new(source);
    let dest = Path::new(dest);
    docs_from_canon(source, dest);
}

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a project). */
pub fn docs_from_project_check(silent: bool, source: &str, dest: &str) {
    let source = Path::new(source);
    let dest = Path::new(dest);
    docs_from_project(source, dest);
}


/* EXECUTIONERS */

fn docs_from_canon(source: &Path, dest: &Path) {
    // TODO
}

fn docs_from_project(source: &Path, dest: &Path) {
    // TODO
}