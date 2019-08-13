// Tom Lancaster (c) Summer 2019
//
// Blackjack - player.rs
/* Reference:
1. https://github.com/chrisccerami/rust-blackjack/blob/master/src/blackjack/player.rs
-Used code as a base to make Player object class
2. https://github.com/seifriedc/blackjack-rust/blob/master/src/main.rs
-Gave me the idea to add more fields to the class
*/

use crate::blackjack::bj::card::Card;

#[derive(Clone)]
pub struct Player {
   pub name: String,
   pub hand: Vec<Card>,
   pub total_score: u32,
   pub num_aces: u32
}

impl Player {
    pub fn score(&mut self) {
        self.total_score = 0;
        for card in &self.hand {
            self.total_score += card.score();
        }
    }
}
