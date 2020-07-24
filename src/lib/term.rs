use console::Term; 
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
