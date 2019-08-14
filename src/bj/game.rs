//Tom Lancaster (c) Summer 2019
//Blackjack - game.rs

/*
use crate::blackjack::bj::player::Player;
use crate::blackjack::bj::deck::Deck;

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub user_turn: bool
}

impl Game {
    pub fn deal(&mut self) {

        //Adds 2 cards to each player to start game
        for player in &mut self.players {
            for mut _x in 0..2 {
                let dealt_card = &self.deck.draw_card();
                player.hand.push(dealt_card.clone());
            }
        }
    }

    pub fn display_hands(&mut self) {
        for player in &mut self.players {
            println!("{}: ", player.name);
            for i in 0..player.hand.len() {
                println!("{} ",player.hand[i].name());
            }
            player.score();
            println!("Total score: {}\n ", player.total_score);
        }
    }

   //Deals card to dealer or user if they "hit" 
    pub fn player_draw_card(&mut self) {
        for player in &mut self.players {
            if player.name == "Dealer" && self.user_turn == false {
                let dealt_card = &self.deck.draw_card();
                player.hand.push(dealt_card.clone());
            } else {
                let dealt_card = &self.deck.draw_card();
                player.hand.push(dealt_card.clone());
            }
        }
    }
    
}

*/
