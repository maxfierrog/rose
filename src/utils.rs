// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9


use std::path::Path;


/* TERMINAL FUNCTIONS */

/* Deletes a specified amount of printed lines. */
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}


/* FILE FUNCTIONS */

/* Returns true iff the file exists and it is not a directory. It assumes
that it is absolute if it starts with '/', otherwise relative. */
pub fn file_exists(path: &str) -> bool {
    let p = Path::new(path);
    p.exists() && p.is_file()
}

/* Returns true iff the file exists and it is not a file. It assumes
that it is absolute if it starts with '/', otherwise relative. */
pub fn folder_exists(path: &str) -> bool {
    let p = Path::new(path);
    p.exists() && p.is_dir()
}

/* Ensures that JSON file formatting template is not malformed. */
pub fn json_template_ok(path: &str) -> bool {
    let p = Path::new(path);
    // TODO
    true
}

/* Ensures that JSON directory structure canon is not malformed. */
pub fn json_canon_ok(path: &str) -> bool {
    let p = Path::new(path);
    // TODO
    true
}
