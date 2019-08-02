//Tom Lancaster (c) Summer 2019
//
//Mini_Games - Main

//#[path = "main_menu.rs"] mod main_menu;
#[path = "blackjack.rs"] mod blackjack;
//#[path = "hangman.rs"] mod hangman;

fn main() {
    //main_menu::main_menu();
    blackjack::blackjack();
    //hangman::hangman();
}
