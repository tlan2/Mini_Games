//Tom Lancaster (c) Summer 2019
//Blackjack - game.rs


use crate::blackjack::bj::player::Player;
use crate::blackjack::bj::deck::Deck;

pub struct Game {
    pub players: Vec<Player>,
    pub deck: Deck,
}

impl Game {
    pub fn start_deal(&mut self) {
        let num_cards = 2;
        for player in &mut self.players {
            for mut card in 0..2 {
                let dealt_card = &self.deck.draw_card();
                player.hand.push(dealt_card.clone());
                card += 1;
            }
        }
    }
    
    //pub fn player_draw(&

    pub fn display_hand(&mut self) {
        let dealer = &self.players.pop().unwrap();
        let player = &self.players.pop().unwrap();
        println!("# of cards dealer = {}", dealer.hand.len());
        println!("# of cards user = {}", player.hand.len());
        println!("The dealer: \n   ? + {} \n", dealer.hand[0]);
        println!("You:\n   {} + {} \n", player.hand[0], player.hand[1]);
    }
}


