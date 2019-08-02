// Tom Lancaster (c) Summer 2019
//
// Mini_Games - Blackjack
/*
References:
1. https://github.com/chrisccerami/rust-blackjack/src
2. https://github.com/seifriedc/blackjack-rust/tree/master/src
-Combined the elements I liked from the above game to create new version
*/

mod bj;

use bj::deck::Deck;

pub fn blackjack() {
    menu();
    let mut deck = Deck { cards: vec![] };
    let mut x = 0;
    /*
    while x != 6 {
        deck.make_deck();
        x += 1;
    }
    */
    deck.make_deck();
    //println!("# cards in deck = {}", deck.cards.len());
    //println!("Cards before shuffle:\n{:?}",deck.cards);
    deck.shuffle();
    //println!("Cards after shuffle:\n{:?}",deck.cards);

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
