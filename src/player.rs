//Tom Lancaster (c) Summer 2019
//
// Blackjack - player.rs
// -Player class object

use blackjack::card::Card;

#[derive(Clone)]
pub struct Player {
    pub hand: Vec<Card>,
    pub total: i32,
}
