//#![windows_subsystem = "windows"]
#![allow(dead_code, unused_variables, non_snake_case, unused_imports)]
mod background;
mod resolution;
mod sprite_sheet;
mod actor;
mod player;
mod input;
mod enemy;
mod spawner;

use background::Background;
use macroquad::{prelude::*};
use resolution::Resolution;
use sprite_sheet::SpriteSheet;
use actor::Actor;
use player::Player;
use enemy::Enemy;
use spawner::Spawner;

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
    
    let bg = Background::new().await;
    let sprite = SpriteSheet::new("resources/characters.png", 16., 8).await;
    
    let mut player = Player::new(vec2(64., 64.), sprite.clone());
    
    let mut spawner = Spawner::new(sprite.clone());

    loop {
        player.update();
        spawner.update(&player);
        
        bg.clear();
        resolution.update();
        bg.draw();
        
        player.draw();
        spawner.draw();
        set_default_camera();
        
        next_frame().await
    }
}
    



        // let textParams = TextParams{
        //     font_size: 50,
        //     ..Default::default()
        // };
        //let s = format!("{:.2}:{:.2}", player.actor.position.x,player.actor.position.y);
        //draw_text_ex(s.as_str(), 100.0, 1080.0, textParams);