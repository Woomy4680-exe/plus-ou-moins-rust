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