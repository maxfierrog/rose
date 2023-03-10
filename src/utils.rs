// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

/* Deletes a specified amount of printed lines. */
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}