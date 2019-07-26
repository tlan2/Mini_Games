//Tom Lancaster (c) Summer 2019
//
//Mini_Games - Return Menus

/*
#[path = "guessing_game.rs"] mod guessing_game;
#[path = "main_menu.rs"] mod main_menu;

pub fn return_menu_gg() {
    println!("\nYou lost. :-(");
    println!("\nReturn Menu:");
    println!("1. Play Again");
    println!("2. Back to Main Menu");
    println!("3. Quit Mini-Games");

    println!("\nSelection: ");
    let mut user_input = String::new();
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut user_input)
                .expect("Failed to read line");

    let user_int = user_input.trim().parse::<u32>().unwrap();

    match user_int {
                1 => guessing_game::guessing_game(),
                2 => main_menu::main_menu(),
                3 => process::exit(1),
                _ => {
                            println!("Error: Enter one of the options above.");
                            return_menu_gg();
                          }
            };
}
*/
