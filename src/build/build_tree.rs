// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

use crate::utils::{file_exists, folder_exists, json_canon_ok, print_error_and_exit};


/* FILE CHECKER FUNCTIONS */

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a canon JSON). */
pub fn docs_from_canon_check(silent: bool, source: &str, dest: &str) {
    if !file_exists(source) {
        print_error_and_exit(silent, 11, "JSON structure specifier file does not\nexist at the provided path. Please verify the\nspecified source path");
        return
    }
    if !folder_exists(dest) {
        print_error_and_exit(silent, 12, "Destination folder does not exist at\nthe provided path. Please verify the specified\ndestination directory path");
        return
    }
    if !json_canon_ok(source) {
        print_error_and_exit(silent, 13, "JSON structure specifier file does not\nconform to Rose specification. Please verify that\nsthe fields and values are valid");
        return
    }

}

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a project). */
pub fn docs_from_project_check(silent: bool, source: &str, dest: &str) {
    if !folder_exists(source) {
        print_error_and_exit(silent, 21, "Specified project directory does not\nexist at the provided path. Please verify the\nsource path");
        return
    }
    if !folder_exists(dest) {
        print_error_and_exit(silent, 12, "Destination folder does not exist at\nthe provided path. Please verify the specified\ndestination directory path");
        return
    }

}


/* EXECUTIONERS */

