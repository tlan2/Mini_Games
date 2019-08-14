//Tom Lancaster (c) Summer 2019
//
//Mini_Games - Hangman
/*
References:
1.https://www.youtube.com/watch?v=omLBlUWfxO0&list=PLVvjrrRCBy2Igh_kCtvRr2Np4fMRawn6x
I used this tutorial as my base and added some new functionalities such as letters_guessed array, hangman visual aid, and menu.
*/


use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;
use rand::Rng;


const ALLOWED_ATTEMPTS: u8 = 5;

struct Letter {
    character: char,
    revealed: bool,
}

enum GameProgress {
    InProgress,
    Won,
    Lost,
}

pub fn hangman() {
    let mut turns_left = ALLOWED_ATTEMPTS;
    let selected_word = select_word();
    let mut letters = create_letters(&selected_word);
    let mut letters_guessed = Vec::new();

    println!(" 
 _                                             
| |                                            
| |__   __ _ _ __   __ _ _ __ ___   __ _ _ __  
| '_ \\ / _` | '_ \\ / _` | '_ ` _ \\ / _` | '_ \\ 
| | | | (_| | | | | (_| | | | | | | (_| | | | |
|_| |_|\\__,_|_| |_|\\__, |_| |_| |_|\\__,_|_| |_|
                    __/ |                      
                   |___/      ");
println!("-----------------------------------------------");
    loop {
        hangman_display(turns_left);
        println!("You have {} turns left.", turns_left);
        display_progress(&letters);
        letters_guessed.sort();
        println!("Letters Guessed: {:?}", letters_guessed);
        println!("Please enter a letter to guess:");
        let user_char = read_user_input_character(&mut letters_guessed);

        // Exit if user enters an asterisk
        if user_char == '*' {
            break;
        }
        
        /* Update the "revealed" state of each letter. If the user
        has guessed a correct letter, at least one revealed is changed 
        to true */
        let mut at_least_one_revealed = false;
        for letter in letters.iter_mut() {
            if letter.character == user_char {
                letter.revealed = true;
                at_least_one_revealed = true;
            }
        }

        // If they have guessed incorrectly, lose a turn
        if !at_least_one_revealed {
            turns_left -= 1;
        }
        
        //Check game progress
        match check_progress(turns_left, &letters) {
            GameProgress::InProgress => continue,
            GameProgress::Won => {
                println!("\nCongrats, you won! The word was {}\n\n", selected_word);
                break;
            }
            GameProgress::Lost => {
                  println!(" 
       _______
     |/      |
     |      (_)
     |      \\|/
     |       |
     |      / \\
     |
  ___|___             DEAD!
");        
                println!("\nSorry, you lost! The word was {}\n\n", selected_word);
                break;
            }
        }
    }

    return_menu();
}

fn select_word() -> String {
    // Open file
    let mut file = File::open("words.txt").expect("Could not open file!");
    
    // Load file contents
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("An error has occurred while reading the file!");

    // Get individual words
    let available_words: Vec<&str> = file_contents.trim()
                                                .split(',')
                                                .collect();
    // Generate random index
    let ind = rand::thread_rng().gen_range(0, available_words.len());

    String::from(available_words[ind])
}

fn create_letters(word: &str) -> Vec<Letter> {
    // Create empty vector for letters
    let mut letters: Vec<Letter> = Vec::new();

    // Wrap each character in a letter struct
    for c in word.chars() {
        letters.push(Letter {
            character: c,
            revealed: false,
        });
    }
    letters
}

fn display_progress(letters: &[Letter]) {
    let mut display_string = String::from("Progress:");
    
    //Display appropriate character (letter or _) for each letter
    for letter in letters {
        display_string.push(' ');

        if letter.revealed {
            display_string.push(letter.character);
        } else {
            display_string.push('_');
        }
    
    display_string.push(' ');
    }

    println!("{}", display_string);
}

fn read_user_input_character(letters_guessed:&mut Vec<char>) -> char {
    let mut user_input = String::new();

    // Get user input
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => {
            match user_input.chars().next() {
                Some(c) => { letters_guessed.push(c);
                             c }
                None => { '*' }
            }
        }
        Err(_) => { '*' }
    }
}

fn check_progress(turns_left: u8, letters:&[Letter]) -> GameProgress {
    //Determine if all letters have been revealed
    let mut all_revealed = true;
    for letter in letters {
        if !letter.revealed {
            all_revealed = false;
        }
    }

    if all_revealed {
        return GameProgress::Won;
    }

    if turns_left > 0 {
        return GameProgress::InProgress;
    }

    GameProgress::Lost
}

fn hangman_display(turns_left:u8) { 
    match turns_left {
            5 =>{ 
                    println!(" 
       _______
     |/      |
     |      
     |  
     |       
     |      
     |
  ___|___
");    
            }
            4 => {
                println!(" 
       _______
     |/      |
     |      (_)
     |       |
     |       |
     |       
     |
  ___|___
");
            }
            3 => {
                    println!(" 
       _______
     |/      |
     |      (_)
     |      \\|
     |       |
     |      
     |
  ___|___
");
            }
            2 => {
                println!(" 
       _______
     |/      |
     |      (_)
     |      \\|/
     |       |
     |  
     |
  ___|___
");
            }
            1 => {
                println!(" 
       _______
     |/      |
     |      (_)
     |      \\|/
     |       |
     |      / 
     |
  ___|___
");
            }
            0 => {
                println!(" 
       _______
     |/      |
     |      (_)
     |      \\|/
     |       |
     |      / \\
     |
  ___|___             DEAD!
");
            }
            _ => {
                println!("Oh no! Hangman Display Error!");
            }
        }
}

fn return_menu() {
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
                1 => hangman(),
                //2 => main_menu::main_menu(),
                2 => process::exit(1),
                _ => {
                            println!("Error: Enter one of the options above.");
                            return_menu();
                }
    };
}
