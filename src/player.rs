use macroquad::prelude::{vec2, Vec2, Color, get_frame_time};
use crate::actor::Actor;
use crate::sprite_sheet::SpriteSheet;
use crate::input::PlayerInput;


pub struct Player{
    pub actor:Actor,
    sprite_sheet:SpriteSheet,
    offset:Vec2,
    image_index:f32,
    velocity:Vec2,
    speed:f32,
}

impl Player{
    pub fn new(position:Vec2, sprite_sheet:SpriteSheet)->Player{
        let actor = Actor::new(position, 8.);
        Player{
            actor,
            sprite_sheet,
            offset: vec2(-8., -8.),
            image_index: 0.,
            velocity: Vec2::ZERO,
            speed: 120.,
        }
    }

    pub fn update(&mut self){
        let delta = get_frame_time();
        let dir = PlayerInput::get_dir();
        self.velocity.x = self.speed * delta * dir.0;
        self.velocity.y = self.speed * delta * dir.1;
        let vx = dir.0;


        self.actor.actor_move(&mut self.velocity);
    }

    pub fn draw(&mut self){
        let x = self.actor.position.x + self.offset.x;
        let y = self.actor.position.y + self.offset.y;
        self.sprite_sheet.draw(x, y, 0, false, Color::new(1., 1., 1., 1.));
    }
}