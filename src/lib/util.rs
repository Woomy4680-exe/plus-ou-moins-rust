use console::style; 
/*
All utils(exit process,...)    
*/

// Prints welcome message
pub fn hello() {
    println!("{}", style("============================").cyan()); 
    println!("{}", style("Jeu plus ou Moins en Rust").green());
    println!("{}", style("Par Woomy4680-exe").green()); 
    println!("{}", style("============================").cyan()); 
}