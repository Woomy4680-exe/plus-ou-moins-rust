use crate::lib::term::{
    writecyan, 
    writegreen, 
    clear
};
use rand::prelude::*;
use std::io; 
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
pub fn genrandom() -> i16 {
    return rand::thread_rng().gen_range(1, 50); 
}
 
pub fn getuserinput() -> String {
    let mut input = String::new(); 
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
          return input; 
        }
        Err(e) => {
          println!("Erreur lors de la récupération de l'entrée! ({:?})", e ); 
          std::process::exit(1);
       }
    }
}
 