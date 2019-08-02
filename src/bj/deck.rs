// Tom Lancaster (c) - Summer 2019

//Blackjack - deck.rs

use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone,Debug)]
pub struct Deck {
    pub cards: Vec<String>,
}

impl Deck {
    pub fn make_deck(&mut self) {
        // C = Clubs, D = Diamonds, H = Hearts, S = Spades
        let suits = vec!["C", "D", "H", "S"];
        for suit in suits {
            let numbers = vec!["2", "3","4","5","6","7","8","9","10","J","Q","K","A"];
            for number in numbers {
                let card = number.to_owned() + suit;
                self.cards.push(card);
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

#[test]
fn test_make_mult_deck() {
    let mut deck = Deck { cards: vec![] };
    let mut x = 0;
    while x != 6 {
        deck.make_deck();
        x += 1;
    }
    assert_eq!(deck.cards.len(),312);
}

