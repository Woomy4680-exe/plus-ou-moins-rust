use rand::prelude::*;
use std::io; 

/*
game functions    
*/
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