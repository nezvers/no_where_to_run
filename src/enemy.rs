use macroquad::prelude::{vec2, Vec2, Color, get_frame_time, rand};
use crate::actor::Actor;
use crate::player::Player;
use crate::sprite_sheet::SpriteSheet;


pub struct Enemy{
    pub actor:Actor,
    sprite_sheet:SpriteSheet,
    image_index:u32,
    velocity:Vec2,
    speed:f32,
}

pub const RANGE_H:f32 = 624. -16. -8.;
pub const RANGE_V:f32 = 352. -16. -8.;
pub const LOW:f32 = 16. + 8.;

impl Enemy{
    pub fn new(sprite_sheet:SpriteSheet, image_index:u32)->Enemy{
        let rand_x = rand::gen_range(LOW, RANGE_H);
        let rand_y = rand::gen_range(LOW, RANGE_V);
        let position = vec2(rand_x, rand_y);


        let actor = Actor::new(position, 8.);
        Enemy{
            actor,
            sprite_sheet,
            image_index,
            velocity: Vec2::ZERO,
            speed: 60.,
        }
    }

    pub fn update(&mut self, player:&Player){
        let delta = get_frame_time();
        let dir = self.get_dir(player);
        self.velocity.x = self.speed * delta * dir.x;
        self.velocity.y = self.speed * delta * dir.y;

        self.actor.actor_move(&mut self.velocity);
    }

    pub fn draw(&mut self){
        let posX = self.actor.position.x -8.;
        let posY = self.actor.position.y -8.;
        self.sprite_sheet.draw(posX, posY, self.image_index, false, Color::new(1., 1., 1., 1.));
    }

    pub fn get_dir(&self, player:&Player)->Vec2{
        let mut relativePos = player.actor.position - self.actor.position;

        // HIT player logic

        if relativePos.length() < self.speed * get_frame_time(){
            relativePos = Vec2::ZERO;
        }
        // distance logic can go there
        relativePos.normalize_or_zero()
    }

    pub fn damage(&mut self){
        // take tamage
    }
}