// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 10

use crate::utils::clear_screen;
use std::process;


/* 10-ERRORS: BAD FILEPATHS */

/* Source file does not exist at provided filepath. */
pub fn error_11(silent: bool) {
    if !silent {
        clear_screen();
        println!("");
        println!("Error 11: Source file or directory does not exist at the provided filepath. Please verify that the path is formed properly (absolute and relative paths are OK).");
        println!("");
    }
    process::exit(11);
}

/* Destination folder does not exist at provided filepath. */
pub fn error_12(silent: bool) {
    if !silent {
        clear_screen();
        println!("");
        println!("Error 12: Destination folder does not exist at the provided filepath. Please verify that the path is formed properly (absolute and relative paths are OK).");
        println!("");
    }
    process::exit(12);
}


/* 20-ERRORS: BAD JSON */

/* JSON canon structure file does not conform to spec. */
pub fn error_21(silent: bool) {
    if !silent {
        clear_screen();
        println!("");
        println!("Error 21: JSON directory structure specifier does not conform to Rose specification. Please verify that the fields and values are valid by looking at our documentation reference.");
        println!("");
    }
    process::exit(21);
}
