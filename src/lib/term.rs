use console::{
    style,
    Term
}; 
/*
Terminal manager(clear, colorprint)    
*/
pub fn clear() {
    let terminal = Term::stdout();
    match terminal.clear_screen() {
        Ok(_n) => {

        }
        Err(_e) => {
            println!("");
            println!("");
            println!("");
            println!("");
            //Clean du pauvre
        }
    }
}
pub fn writecyan(text: &str) {
    println!("{}", style(text).cyan());
}
pub fn writegreen(text: &str) {
    println!("{}", style(text).green());
}
pub fn writered(text: &str) {
    println!("{}", style(text).on_red()); 
}
