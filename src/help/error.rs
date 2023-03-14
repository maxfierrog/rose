// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 10

use std::process;


/* 100-ERRORS: BAD FILEPATHS */

/* Source file does not exist at provided filepath. */
pub fn error_101(silent: bool) {
    eprintln!("Error: Source file or directory does not exist at the provided filepath. Please verify that the path was formed properly (absolute and relative paths are OK).");
    process::exit(101);
}

/* Destination folder does not exist at provided filepath. */
pub fn error_102(silent: bool) {
    eprintln!("Error: Destination folder does not exist at the provided filepath. Please verify that the path was formed properly (absolute and relative paths are OK).");
    process::exit(102);
}


/* 200-ERRORS: BAD JSON */

/* JSON canon structure file does not conform to spec. */
pub fn error_201(silent: bool) {
    eprintln!("Error: JSON directory structure specifier does not conform to Rose specification. Please verify that the fields and values in the file are valid by looking at our documentation reference.");
    process::exit(201);
}
