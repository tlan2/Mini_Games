//Tom Lancaster (c) Summer 2019
//Guessing Game code

//Reference(s):
//https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

use std::io::{self, Write};
use std::process;
use std::cmp::Ordering;
use rand::Rng;

pub fn guessing_game() {
    
    println!("\n\nGuess a number between 1 and 100!");
    println!("\n You have 5 guesses to get it right. Good luck!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut win = false;
    
    let mut guess_count = 5;

    while guess_count != 0 {

        let mut guess = String::new(); //Use datatype String for user input
        print!("\nGuess: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut guess)
                    .expect("Failed to read line");
            
        //Converts string to number; u32 good for small positive nums
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        guess_count -= 1;

        match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small! Remaining Guesses: {}", guess_count),
        Ordering::Greater => println!("Too big! Remaining Guesses: {}", guess_count),
        Ordering::Equal => {
            win = true;
            println!("You win!");
            println!("   
.*   ..  . *  *
** @()Ooc()*   o  .
(Q@*0CG*O(   )___
|\\_________/|/ _ \\
|  |  |  |  | / | |
|  |  |  |  | | | |
|  |  |  |  | | | |
|  |  |  |  | | | |
|  |  |  |  | | | |
|  |  |  |  | \\_| |
|  |  |  |  |\\___/  
|\\_|__|__|_/|
 \\_________/\n\n");

                        break;}
        }

    }
    if guess_count == 0 && win == true {
            return_menu_gg();
    } else{ 
            println!("The correct number was {}.", secret_number);
            println!("\nYou lost. :-(");
            return_menu_gg();
    }
}


fn return_menu_gg() {
    println!("Return Menu:");
    println!("1. Play Again");
    //println!("2. Back to Main Menu");
    println!("2. Quit Mini-Games");

    print!("\nSelection: ");
    let mut user_input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut user_input)
                .expect("Failed to read line");

    let user_int = user_input.trim().parse::<u32>().unwrap();

    match user_int {
                1 => guessing_game(),
                //2 => main_menu::main_menu(),
                2 => process::exit(1),
                _ => {
                            println!("Error: Enter one of the options above.");
                            return_menu_gg();
                          }
            };
}

