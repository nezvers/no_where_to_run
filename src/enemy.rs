
use macroquad::prelude::{vec2, Vec2, Color, get_frame_time, rand as macroRand};
use crate::actor::Actor;
use crate::background::Background;
use crate::player::{Player, PlayerState};
use crate::sprite_sheet::SpriteSheet;

pub const RANGE_H:f32 = 624. -16. -8.;
pub const RANGE_V:f32 = 352. -16. -8.;
pub const LOW:f32 = 16. + 8.;

#[derive(PartialEq)]
pub enum EnemyState{
    SpawnPoint,
    Attack,
    Dead,
}

pub struct Enemy{
    pub actor:Actor,
    sprite_sheet:SpriteSheet,
    image_index:u32,
    velocity:Vec2,
    speed:f32,
    spawn_time:f32,
    health:f32,
    pub state:EnemyState,
}

impl Enemy{
    pub fn new(sprite_sheet:SpriteSheet, image_index:u32)->Enemy{
        let rand_x = macroRand::gen_range(LOW, RANGE_H);
        let rand_y = macroRand::gen_range(LOW, RANGE_V);

        Enemy{
            actor: Actor::new(vec2(rand_x, rand_y), 8.),
            sprite_sheet,
            image_index,
            velocity: Vec2::ZERO,
            speed: 45.,
            spawn_time: 2.,
            health: 10.,
            state: EnemyState::SpawnPoint,
        }
    }

    pub fn update(&mut self, player:&mut Player, background:&mut Background){
        if self.state == EnemyState::SpawnPoint{
            self.spawn_time -= get_frame_time();
            if self.spawn_time > 0.{
                return
            }
            self.state = EnemyState::Attack;
        }
        let delta = get_frame_time();
        let dir = if player.state == PlayerState::Active{
            self.get_dir(player, background)
        }
        else{
            Vec2::ZERO
        };
        self.velocity = self.velocity.lerp(self.speed * dir, 5. * delta);

        self.actor.actor_move(&mut self.velocity);
    }

    pub fn draw(&mut self){
        let pos_x = self.actor.position.x -8.;
        let pos_y = self.actor.position.y -8.;
        if self.state == EnemyState::SpawnPoint{
            // draw spawn point
            let icon = 68;
            let mut alpha = 1. - self.spawn_time.fract();
            alpha = 1. - (alpha * alpha * alpha);
            self.sprite_sheet.draw(pos_x, pos_y, icon, false, Color::new(1., 0., 0., alpha));
            return
        }
        else{
            self.sprite_sheet.draw(pos_x, pos_y, self.image_index, false, Color::new(1., 1., 1., 1.));
        }
    }

    pub fn get_dir(&mut self, player:&mut Player, background:&mut Background)->Vec2{
        let mut relative_pos = player.actor.position - self.actor.position;
        // HIT player logic

        if relative_pos.length() < player.actor.radius + self.actor.radius{
            let hit = player.damage(1., relative_pos.normalize_or_zero());
            if hit {
                background.damage();
            }
            relative_pos = Vec2::ZERO;
        }
        // distance logic can go there
        relative_pos.normalize_or_zero()
    }

    pub fn damage(&mut self, value:f32, impulse:&Vec2){
        self.velocity = impulse.clone();
        self.health -= value;
        if self.health <= 0.{
            self.state = EnemyState::Dead;
        }
    }

    // pub fn respawn(&mut self){
    //     *self = Enemy::new(self.sprite_sheet, self.image_index);
    // }
}