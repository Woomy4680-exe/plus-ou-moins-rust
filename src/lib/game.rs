//Imports
use crate::lib::term::{clear, writecyan, writegreen, writered};
use crate::lib::util::{getuserinput, hello, intize};

//Main function
pub fn play() {
    clear();
    hello();
    let random = crate::lib::util::genrandom(); //Generate a random number beetween  1 and 50
    loop {
        let inpt = input();
        if checknum(random, inpt) {
            println!("");
            writegreen("Merci d'avoir joué!");
            break;
        }
    }
}

//Checks if numbers are = > or <
fn checknum(rnd: i32, userin: i32) -> bool {
    if rnd == userin {
        writecyan("Votre nombre est correct! Bravo");
        return true;
    } else if rnd > userin {
        clear();
        hello();
        println!("{}", userin);
        writered("Votre nombre est trop petit!");
        return false;
    } else {
        clear();
        hello();
        println!("{}", userin);
        writered("Votre nombre est trop grand!");
        return false;
    }
}
fn input() -> i32 {
    println!("Veuillez entrer un nombre!");
    let inp = getuserinput();
    let corr = intize(inp);
    return corr;
}
