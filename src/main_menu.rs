//Tom Lancaster (c) Summer 2019
//
// main_menu.rs


use std::io::{self, Write};

#[path = "guessing_game.rs"] mod guessing_game;
#[path = "snake/snake.rs"] mod snake;
#[path = "blackjack.rs"] mod blackjack;
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

    let input = &mut String::new();
    print!("Enter a number: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(input).expect("Failed to read line.");


    match input.trim() {
        "1" => guessing_game::guessing_game(),
        "2" => snake::snake(),
        "3" => blackjack::blackjack(),
        "4" => hangman::hangman(),
         _  => {
                println!("Not a valid selection. Please pick another number.");
                main_menu();
                }
    }
}

