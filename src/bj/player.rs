// Tom Lancaster (c) Summer 2019
//
// Blackjack - player.rs

use crate::main_menu::blackjack::bj::card::Card;

#[derive(Clone)]
pub struct Player {
   pub name: String,
   pub hand: Vec<Card>,
   pub num_aces: u32
}

impl Player {
    pub fn score(&mut self) -> u32 {
        let mut score = 0;
        for card in &self.hand {
            score += card.score();
        }
        score
    }
}
