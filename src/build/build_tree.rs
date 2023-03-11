// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

use crate::{utils::{file_exists, folder_exists, json_canon_ok}, errors::*};
use std::path::Path;


/* FILE CHECKER FUNCTIONS */

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a canon JSON). */
pub fn docs_from_canon_check(silent: bool, source: &str, dest: &str) {
    if !file_exists(source) {
        error_11(silent);
        return
    }
    if !folder_exists(dest) {
        error_12(silent);
        return
    }
    if !json_canon_ok(source) {
        error_21(silent);
        return
    }
    let source = Path::new(source);
    let dest = Path::new(dest);
    docs_from_canon(source, dest);
}

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a project). */
pub fn docs_from_project_check(silent: bool, source: &str, dest: &str) {
    if !folder_exists(source) {
        error_11(silent);
        return
    }
    if !folder_exists(dest) {
        error_12(silent);
        return
    }
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