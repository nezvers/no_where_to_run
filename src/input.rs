use macroquad::prelude::{is_key_down, is_mouse_button_down, KeyCode, MouseButton, mouse_position, Vec2, vec2};

pub struct PlayerInput{}

impl PlayerInput{
    pub fn get_dir()->(f32, f32){
        let right = (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) as i32;
        let left = (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) as i32;
        let up = (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) as i32;
        let down = (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) as i32;
        let x = (right - left) as f32;
        let y = (down - up) as f32;
        (x,y)
    }
}