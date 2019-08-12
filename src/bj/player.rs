// Tom Lancaster (c) Summer 2019
//
// Blackjack - player.rs
/* Reference:
1. https://github.com/chrisccerami/rust-blackjack/blob/master/src/blackjack/player.rs
-Used code as a base to make Player object class
2. https://github.com/seifriedc/blackjack-rust/blob/master/src/main.rs
-Gave me the idea to add more fields to the class
*/

#[derive(Clone, Debug)]
pub struct Player {
   pub name: String,
   pub hand: Vec<String>,
   pub score: u32,
   pub num_aces: u32
}
