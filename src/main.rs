//Tom Lancaster (c) Summer 2019

//Reference(s):
//https://doc.rust-lang.org/stable/book/ch02-00-guessing-game-tutorial.html

use std::io;
use std::process;
use std::cmp::Ordering;
use rand::Rng;

fn guessing_game() {
    
    println!("\n\nGuess a number between 1 and 100!");
    println!("\n You have 4 guesses to get it right. Good luck!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    
    let mut guess_count = 4;

    while guess_count != 0 {

        println!("\nPlease input your guess.");

            let mut guess = String::new(); //Use datatype String for user input

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
                        println!("You win!");
                    println!("   
.   *   ..  . *  *
*  * @()Ooc()*   o  .
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
                        break;
                    }
                }
        }
    if guess_count == 0 {
            return_menu_gg();
    }
}


fn return_menu_gg() {
    println!("You lost. :-(");
    println!("\nReturn Menu:");
    println!("1. Play Again");
    println!("2. Back to Main Menu");
    println!("3. Quit Mini-Games");
    
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)
                .expect("Failed to read line");
    
    let user_int = user_input.trim().parse::<u32>().unwrap();

    match user_int {
                1 => guessing_game(),
                //Ok(2) => main(),
                2 => {
                            println!("Unimplemented function - main menu");
                            return_menu_gg()
                         },
                3 => process::exit(1),
                _ => {
                            println!("Error: Enter one of the options above.");
                            return_menu_gg();
                          }
            };
}


fn main() {
    guessing_game();
}

/*
fn main() {
    println!(
    "
    ███╗   ███╗██╗███╗   ██╗██╗       █████╗ ██████╗  ██████╗ █████╗ ██████╗ ███████╗
    ████╗ ████║██║████╗  ██║██║      ██╔══██╗██╔══██╗██╔════╝██╔══██╗██╔══██╗██╔════╝
    ██╔████╔██║██║██╔██╗ ██║██║█████╗███████║██████╔╝██║     ███████║██║  ██║█████╗  
    ██║╚██╔╝██║██║██║╚██╗██║██║╚════╝██╔══██║██╔══██╗██║     ██╔══██║██║  ██║██╔══╝  
    ██║ ╚═╝ ██║██║██║ ╚████║██║      ██║  ██║██║  ██║╚██████╗██║  ██║██████╔╝███████╗
    ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═╝      ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚═════╝ ╚══════╝
    "                                                                       
    );

    println!("Please pick one of the following games:\n");
    println!("1. Guessing Game");
    println!("2. Snake");
    println!("3. Blackjack");
    println!("4. Hangman");
    println!("5. Go Fish");
    println!("6. Shooter");

    let mut input = String::new();
    let
    match
*/
