// Tom Lancaster (c) Summer 2019
//
// Mini_Games - Blackjack
/*
References:
1. https://github.com/chrisccerami/rust-blackjack/src
2. https://github.com/seifriedc/blackjack-rust/tree/master/src
-Combined the objected-oriented elements of one with the user I/O of the other.
*/

mod bj;

use std::io;
use std::io::Write;
use std::process;
use bj::deck::Deck;
use bj::player::Player;


pub fn blackjack() {
    menu();
    //Pre-game Setup 
    let mut deck = Deck { cards: vec![] };
    //Create players for game
    let mut dealer = Player{name: "Dealer".into(), hand: vec![], num_aces: 0};
    let mut user = Player{name: "You".into(), hand: vec![], num_aces: 0}; 
    let mut user_turn = true;

    deck.make_deck();
    deck.shuffle();
    deal(&mut deck, &mut user, &mut dealer);
    display_hands(&mut user, &mut dealer);
    check_blackjack(&mut user, &mut dealer);

    let mut input = get_move();
    if input == "stand" { user_turn = false; }
    
    loop {
        while user_turn {
            draw_card(&mut deck,&mut user);
            display_hands(&mut user, &mut dealer);
            check_blackjack(&mut user, &mut dealer);

            if user.score() > 21 {
                println!("BUSTED! \nYou busted. Dealer wins. :-(\n\n");
                return_menu();
            }

        // Get input from user
            input = get_move();
            if input == "stand" { user_turn = false; }
        }


        if dealer.score() >= 17 {
            // Determing who wins if no one busts
            println!("Dealer stands.\n");
            display_hands(&mut user, &mut dealer);

            if dealer.score() > user.score() {
                println!("Dealer wins.\n");
            } else if dealer.score() < user.score() {
                println!("You win!\n");
            } else {
                println!("Tie! (Push)\n");
            }

            return_menu();
        } 
        
            else {
            // Dealer's turn - loop
            while dealer.score() < 17 {
                println!("\nDealer hits.\n");
                draw_card(&mut deck,&mut dealer);

                display_hands(&mut user, &mut dealer);
                
                // The dealer busts
                if dealer.score() > 21 {
                    display_hands(&mut user, &mut dealer);
                    println!("BUSTED!\n\n");
                    println!("The dealer busted. You win!\n");

                    return_menu();
                }
            }
        }
    }
}

fn deal(deck: &mut Deck, usr: &mut Player, dlr: &mut Player) {
    for mut _x in 0..2 {
        let card1 = deck.draw_card();
        usr.hand.push(card1.clone());
        let card2 = deck.draw_card();
        dlr.hand.push(card2.clone());
    }

}

fn draw_card(deck: &mut Deck, player : &mut Player) {
    let dealt_card = deck.draw_card();
    player.hand.push(dealt_card.clone());
}

fn check_blackjack(usr: &mut Player, dlr: &mut Player) {
    if dlr.score() == 21 && usr.score() == 21 {
        println!("\nBLACKJACK FOR BOTH! Push.\n");
        return_menu();
    } else if dlr.score() == 21 {
        println!("\nDEALER BLACKJACK! You lose. :-(\n");
        return_menu(); 
    } else if usr.score() == 21 {
        println!("\nBLACKJACK! You win! :-)\n");
        return_menu();
    }

}

fn display_hands(usr: &mut Player, dlr: &mut Player) {
    println!("\n{}: ", dlr.name);
    for i in 0..dlr.hand.len() {
        println!("{} ",dlr.hand[i].name());
    }
    println!("Total score: {}\n ", dlr.score());
    
    println!("{}: ", usr.name);
    for i in 0..usr.hand.len() {
        println!("{} ",usr.hand[i].name());
    }
    println!("Total score: {}\n ", usr.score());
}

fn get_move() -> String {
    let mut input = String::new();
    loop {
        println!("Would you like to \"hit\" or \"stand\"?");
        io::stdin().read_line(&mut input).ok();
        input = String::from(input.trim());
        match input.as_ref() {
            "hit" | "stand" => return input,
                           _ => input = String::new(),
        }
    }
}


fn menu() {
    println!("

$$$$$$$   $$                      $$                                    $$
$$    $$  $$                      $$                                    $$
$$    $$  $$   $$$$$$    $$$$$$$  $$    $$      $$   $$$$$$    $$$$$$$  $$    $$
$$$$$$$   $$        $$  $$        $$   $$                 $$  $$        $$   $$
$$    $$  $$   $$$$$$$  $$        $$$$$$$$      $$  $$$$$$$$  $$        $$$$$$
$$    $$  $$  $$    $$  $$        $$   $$       $$  $$    $$  $$        $$   $$
$$$$$$$   $$   $$$$$$$   $$$$$$$  $$    $$      $$  $$$$$$$$   $$$$$$$  $$    $$
                                                $$
                                          $$    $$
                                           $$$$$$

");
}

fn return_menu() {
    println!("Return Menu:");
    println!("1. Play Again");
    println!("2. Quit Mini-Games");

    print!("\nSelection: ");
    let mut user_input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut user_input)
                .expect("Failed to read line");

    let user_int = user_input.trim().parse::<u32>().unwrap();

    match user_int {
                1 => blackjack(),
                2 => process::exit(1),
                _ => {
                            println!("Error: Enter one of the options above.");
                            return_menu();
                }
    };
}
