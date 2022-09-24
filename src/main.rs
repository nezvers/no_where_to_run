#![allow(dead_code, unused_variables, non_snake_case, unused_imports)]
mod background;
mod resolution;


use background::Background;
use macroquad::{prelude::*};
use resolution::Resolution;


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

    loop {
        bg.clear();

        resolution.update();
        bg.draw();

        set_default_camera();
        
        next_frame().await
    }
}
