//#![windows_subsystem = "windows"]
#![allow(dead_code, unused_variables)]//, non_snake_case, unused_imports)]
mod background;
mod resolution;
mod sprite_sheet;
mod actor;
mod player;
mod input;
mod enemy;
mod factory_enemy;
mod game;
mod assets;
mod level;
mod bullet;
mod factory_bullet;

use macroquad::window::{Conf, next_frame};
//use macroquad::{prelude::*};
use resolution::Resolution;
use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let resolution = Resolution::new();
    let mut game = Game::new().await;
    
    loop {
        resolution.update();
        game.update();
        
        next_frame().await
    }
}
    



        // let textParams = TextParams{
        //     font_size: 50,
        //     ..Default::default()
        // };
        //let s = format!("{:.2}:{:.2}", player.actor.position.x,player.actor.position.y);
        //draw_text_ex(s.as_str(), 100.0, 1080.0, textParams);