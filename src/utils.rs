// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9


use std::path::Path;
use crate::constants::JSON_CANON_NAME;


/* FILE UTILS */

/// Returns true iff the file exists and it is not a directory. It assumes
/// that it is absolute if it starts with '/', otherwise relative.
pub fn file_exists(filepath: &str) -> bool {
    let p = Path::new(filepath);
    p.exists() && p.is_file()
}

/// Returns true iff the file exists and it is not a file. It assumes
/// that it is absolute if it starts with '/', otherwise relative.
pub fn folder_exists(path: &str) -> bool {
    let p = Path::new(path);
    p.exists() && p.is_dir()
}


/* JSON CANON UTILS */

/// Ensures that JSON directory structure canon is not malformed.
pub fn json_canon_ok() -> bool {
    let p = Path::new(JSON_CANON_NAME);
    if !p.exists() { return true }
    check_json_structure(p)
}

/// Returns true if json canon file is already present.
pub fn json_canon_exists(path: &str) -> bool {
    let p = Path::new(path).join(JSON_CANON_NAME);
    p.exists() && p.is_file()
}


/* HELPER FUNCTIONS */

fn check_json_structure(p: &Path) -> bool {
    // TODO
    true
}
