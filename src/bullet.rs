use macroquad::prelude::{Vec2, Color, get_frame_time};

use crate::enemy::{Enemy, EnemyState};
use crate::sprite_sheet::SpriteSheet;
use crate::actor::Actor;

pub struct Bullet{
    pub actor:Actor,
    velocity:Vec2,
    sprite_sheet:SpriteSheet,
    image_index: u32,
    damage:f32,
    pub hit:bool,
}

impl Bullet{
    pub fn new(position:Vec2, velocity:Vec2, radius:f32, damage:f32, sprite_sheet:SpriteSheet, image_index: u32)->Bullet{
        let actor = Actor::new(position, radius);
        // spawn particles
        Bullet{
            actor,
            velocity,
            sprite_sheet,
            image_index,
            damage,
            hit:false,
        }
    }

    pub fn update(&mut self, enemy_list:&mut Vec<Enemy>){
        let delta = get_frame_time();
        let mut vel = self.velocity.clone();
        self.actor.actor_move(&mut vel);

        // check collision agains enemy
        for enemy in enemy_list.iter_mut(){
            if enemy.state != EnemyState::Attack{
                continue;
            }
            let dist = (enemy.actor.position - self.actor.position).length();
            let contact_distance = enemy.actor.radius + self.actor.radius;
            if dist <= contact_distance{
                enemy.damage(self.damage);
                self.hit = true;
                // return on first collision
                return
            }
        }

        if self.actor.on_wall{
            self.hit = true; // if hit wall mark as hit
        }
    }
    
    pub fn draw(&mut self){
        self.sprite_sheet.draw(self.actor.position.x -8., self.actor.position.y -8., self.image_index, false, Color::new(1., 1., 1., 1.));
    }
}