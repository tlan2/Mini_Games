// Tom Lancaster (c) - Summer 2019

// Blackjack - deck.rs

use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::blackjack::bj::card::Card;

#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn make_deck(&mut self) {
        let suits = vec!["Clubs", "Diamonds", "Hearts", "Spades"];
        for suit in suits {
            let numbers = vec!["2", "3","4","5","6","7","8","9","10","J","Q","K","A"];
            for number in numbers {
                let card = Card { suit: suit.into(), value: number.to_string() };
                self.cards.push(card);
            }
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw_card(&mut self) -> Card {
        self.cards.pop().unwrap()
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

#[test]
fn test_draw_2cards() {
    let mut deck = Deck { cards: vec![] };
    deck.make_deck();
    deck.shuffle();
    deck.draw_card();
    assert_eq!(deck.cards.len(), 51);
    deck.draw_card();
    assert_eq!(deck.cards.len(), 50);
}

