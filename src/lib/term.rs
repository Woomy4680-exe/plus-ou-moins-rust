/*
Terminal manager(clear, colorprint)
*/
pub fn writecyan(text: &str) {
    println!("\x1b[96m{}\x1b[m", text);
}
pub fn writegreen(text: &str) {
    println!("\x1b[92m{}\x1b[m", text);
}
pub fn writered(text: &str) {
    println!("\x1b[91m{}\x1b[m", text);
}
