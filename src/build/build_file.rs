// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9


/* MENU SPECIFIC CHECKS */

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a docfile from source code). */
pub fn docfile_from_code_check(source: &str, dest: &str) {
    // TODO
}

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a docfile from a JSON template). */
pub fn docfile_from_template_check(source: &str, dest: &str) {
    // TODO
}

/* Checks that all referenced files exist and confirms with user before 
completing action (generating a docfile from source code into a document whose
formatting is specified by JSON). */
pub fn docfile_from_both_check(template: &str, code: &str, dest: &str) {
    // TODO
}


/* EXECUTIONERS */

/* Generates a docfile by reading and parsing the code file, and places it in 
the destination. */
fn docfile_from_code(source: &str, dest: &str) {
    // TODO
}

/* Generates the template file and puts it into the destination. */
fn docfile_from_template(source: &str, dest: &str) {
    // TODO
}

/* Generates a docfile in accordance to the template by reading and parsing 
the code file, and finally places it in the destination. */
fn docfile_from_both(template: &str, code: &str, dest: &str) {
    // TODO
}