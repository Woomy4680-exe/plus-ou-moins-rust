use crate::lib::term::{
    writecyan, 
    writegreen, 
    clear
};
 
/*
All utils(exit process,...)    
*/

// Prints welcome message
pub fn hello() {
    clear(); 
    writecyan("============================");
    writegreen("Jeu plus ou Moins en Rust");
    writegreen("Par Woomy4680-exe");
    writecyan("============================");
}
pub fn intize(input: String) -> i32 {
    match input.trim().parse::<i32>() {
       Ok(i) => {
          return i; 
       }
       Err(_e) => {
          println!("Veuillez entrer un nombre valide!"); 
          std::process::exit(1); 
       }
    } 
 }