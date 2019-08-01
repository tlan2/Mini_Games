//Tom Lancaster (c) Summer 2019

use std::io::{self, Write};

#[path = "guessing_game.rs"] mod guessing_game;
#[path = "snake/snake.rs"] mod snake;
//#[path = "blackjack.rs"] mod blackjack;
#[path = "hangman.rs"] mod hangman;

pub fn main_menu() {
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
println!("--------------------------------------------------------------------------------");
    println!("Please pick one of the following games:\n");
    println!("1. Guessing Game");
    println!("2. Snake");
    println!("3. Blackjack");
    println!("4. Hangman");
    println!("5. Go Fish");
    println!("6. Shooter");

    let input = &mut String::new();
    print!("Enter a number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(input).expect("Failed to read line.");


    match input.trim() {
        "1" => guessing_game::guessing_game(),
        "2" => snake::snake(),
        "3" => println!("Unimplemented game - Blackjack. Please pick another number."),
        "4" => hangman::hangman(),
        "5" => println!("Unimplemented game - Go Fish. Please pick another number."),
        "6" => println!("Unimplemented game - Shooter. Please pick another number."),
         _  => {
                println!("Not a valid selection. Please pick another number.");
                main_menu();
                }
    }
}

