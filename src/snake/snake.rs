// Tom Lancaster (c) Summer 2019
//
// Snake Game - Main Function
// Made game for Rust practice using the following tutorial:
// https://www.youtube.com/watch?v=DnT_7M7L7vo

extern crate rand;
extern crate piston_window;

#[path = "draw.rs"] mod draw;
#[path = "snake_wrap.rs"] mod snake_wrap;
#[path = "snake_game.rs"] mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

pub fn snake() {
    let (width, height) = (30, 30);

    let mut window: PistonWindow = 
        WindowSettings::new("Snake",[to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}

