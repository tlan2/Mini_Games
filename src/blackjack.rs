// Tom Lancaster (c) Summer 2019
//
// Mini_Games - Blackjack
/*
References:
1. https://github.com/chrisccerami/rust-blackjack/src
2. https://github.com/seifriedc/blackjack-rust/tree/master/src
-Combined the elements I liked from the above game to create new version
*/

fn main () {



}

fn display_hands(usr: &Player, dlr: &Player) {
    println!("The dealer:\n? + {}\n", dlr.new_card);
    println!("You:\n{} + {} = {}\n", usr.hand, usr.new_card, usr.total):
}


fn get_move() -> String {
    let mut input = String::new();

    //Forces user to choose hit or stand
    loop{
        println!("Would you like to (h)it or (s)tand?");
        io::stdin().read_line(&mut input).ok();
        input = String::from(input.trim()); //Removes newline from input
        match input.as_ref() {
            "h" | "s" => return input,
                    _ => input = String::new(),
        }
    }
}
