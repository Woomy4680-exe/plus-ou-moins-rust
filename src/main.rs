use console::style; //For colors
use std::io; //For get STDIN
use rand::prelude::*;  //For generateRandom
use console::Term; 
fn main() {
    game();
}
fn game() {
    cleanterm();
    hello(); 
    let num = gen_number(); 
    let mut input = prompt_num();
    loop {
     
        if input == num {
            println!("Bravo! Vous avez gagné!");
            exit(false);
        } else if input > num {
            writered("Votre nombre est trop grand!");
            input = prompt_num();
        } else {
            writered("Votre nombre est trop petit!");
            input = prompt_num();
        }
    } 
}
fn gen_number() -> i32 {
    let n = rand::thread_rng().gen_range(1, 50);
    return n; 
}
fn prompt_num() -> i32 {
    let mut num = String::new();
    println!("Veuillez entrer un nombre!");
    match io::stdin().read_line(&mut num) {
        Ok(_n) => {
        } 
        Err(e) => {
            println!("Erreur lors de la lecture du nombre: {:?}", e); 
            exit(true);
        }
    }
    let finalnum;
   
    match num.trim().parse::<i32>() {
        Ok(n) => {
            finalnum = n;
        }
        Err(_e) =>  {
            println!("Merci d'entrer un nombre valide!");
            finalnum = 0;
            exit(true); 
        }
    };
    return finalnum; 
}
fn exit(err: bool) {
    println!(""); 
    println!("Merci d'avoir joué!"); 
    println!("\n====================");
    if err {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}
fn cleanterm() {
    let term = Term::stdout();
    match term.clear_screen() {
        Ok(_n) => {

        }
        Err(_e) => {
            println!("");
            println!("");
            println!("");
            println!("");
        }
    }
}
fn hello() {
    println!("===========");
    println!("Jeu du plus ou moins");
}
fn writered(text: &str) {
    println!("{}", style(text).red());
}