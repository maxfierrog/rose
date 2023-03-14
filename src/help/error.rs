// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 10

use std::process;


/* 100-ERRORS: BAD FILEPATHS */

/* Source file does not exist at provided filepath. */
pub fn error_101() {
    eprintln!("Error 101: Source file or directory does not exist at the provided filepath. Please verify that the path was formed properly (absolute and relative paths are OK).");
    process::exit(exitcode::NOINPUT);
}

/* Destination folder does not exist at provided filepath. */
pub fn error_102() {
    eprintln!("Error 102: Destination folder does not exist at the provided filepath. Please verify that the path was formed properly (absolute and relative paths are OK).");
    process::exit(exitcode::CANTCREAT);
}


/* 200-ERRORS: BAD JSON */

/* JSON canon structure file does not conform to spec. */
pub fn error_201() {
    eprintln!("Error 201: JSON directory structure specifier does not conform to Rose specification. Please verify that the fields and values in the file are valid by looking at our documentation reference.");
    process::exit(exitcode::CONFIG);
}
