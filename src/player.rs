use macroquad::prelude::{vec2, Vec2, Color, get_frame_time};
use crate::actor::Actor;
use crate::sprite_sheet::SpriteSheet;
use crate::input::PlayerInput;


pub struct Player{
    pub actor:Actor,
    sprite_sheet:SpriteSheet,
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
            image_index: 0.,
            velocity: Vec2::ZERO,
            speed: 120.,
        }
    }

    pub fn update(&mut self){
        let delta = get_frame_time();
        let dir = PlayerInput::get_dir().normalize_or_zero();
        self.velocity.x = self.speed * delta * dir.x;
        self.velocity.y = self.speed * delta * dir.y;

        self.actor.actor_move(&mut self.velocity);
    }

    pub fn draw(&mut self){
        let posX = self.actor.position.x -8.;
        let posY = self.actor.position.y -8.;
        self.sprite_sheet.draw(posX, posY, 0, false, Color::new(1., 1., 1., 1.));
    }

    pub fn damage(&mut self){
        // take tamage
    }
}