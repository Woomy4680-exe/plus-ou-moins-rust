use rand::prelude::*;
/*
game functions    
*/
pub fn genrandom() -> i16 {
   return rand::thread_rng().gen_range(1, 50); 
}