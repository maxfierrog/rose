// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

use crate::utils::{file_exists, folder_exists, json_canon_ok, print_error_and_exit};


/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a canon JSON). */
pub fn docs_from_canon_check(source: &str, dest: &str) {
    if !file_exists(source) {
        print_error_and_exit(11, "JSON format specifier file does not\nexist at the provided path. Please verify the\nspecified source path");
        return
    }
    if !folder_exists(dest) {
        print_error_and_exit(12, "Destination folder does not exist at\nthe provided path. Please verify the specified\ndestination directory path");
        return
    }
    if !json_canon_ok(source) {
        // TODO: Display descriptive error
        return
    }

}

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a doctree from a project). */
pub fn docs_from_project_check(source: &str, dest: &str) {
    if !folder_exists(source) {
        // TODO: Display descriptive error
        return
    }
    if !folder_exists(dest) {
        // TODO: Display descriptive error
        return
    }
}


/* HELPER METHODS */

