use macroquad::prelude::{Vec2, Color, get_frame_time, vec2};
use macroquad_particles::{BlendMode, Emitter, EmitterConfig, EmissionShape, ParticleShape, ColorCurve};

use crate::enemy::{Enemy, EnemyState};
use crate::sprite_sheet::SpriteSheet;
use crate::actor::Actor;

#[derive(PartialEq)]
pub enum BulletState{
    Fly,
    Impact,
    Dead,
}

pub struct Bullet{
    pub actor:Actor,
    pub state:BulletState,
    velocity:Vec2,
    tail_emitter:Emitter,
    impact_emitter:Emitter,
    sprite_sheet:SpriteSheet,
    image_index: u32,
    damage:f32,
    timer:f32,
}

const TAIL_EMITTER:EmitterConfig = EmitterConfig{
    local_coords: false,
    emission_shape: EmissionShape::Point,
    one_shot: false,
    lifetime: 0.5,
    lifetime_randomness: 0.3,
    amount: 50,
    shape: ParticleShape::Rectangle,
    explosiveness: 0.0,
    emitting: true,
    initial_direction: vec2(0., -1.),
    initial_direction_spread: 2.,
    initial_velocity: 50.0,
    initial_velocity_randomness: 0.0,
    linear_accel: 0.0,
    size: 0.5,
    size_randomness: 0.0,
    size_curve: None,
    blend_mode: BlendMode::Alpha,
    colors_curve: ColorCurve {
        start: Color::new(1., 1., 1., 1.),
        mid: Color::new(1., 1., 1., 1.),
        end: Color::new(1., 1., 1., 1.),
    },
    gravity: vec2(0.0, 0.0),
    texture: None,
    atlas: None,
    material: None,
    post_processing: None,
};

const IMPACT_EMITTER:EmitterConfig = EmitterConfig{
    local_coords: false,
    emission_shape: EmissionShape::Point,
    one_shot: true,
    lifetime: 0.5,
    lifetime_randomness: 0.3,
    amount: 50,
    shape: ParticleShape::Rectangle,
    explosiveness: 1.0,
    emitting: false,
    initial_direction: vec2(0., -1.),
    initial_direction_spread: 2.,
    initial_velocity: -100.0,
    initial_velocity_randomness: 0.75,
    linear_accel: -1.0,
    size: 0.5,
    size_randomness: 0.0,
    size_curve: None,
    blend_mode: BlendMode::Alpha,
    colors_curve: ColorCurve {
        start: Color::new(1., 1., 1., 1.),
        mid: Color::new(1., 1., 1., 1.),
        end: Color::new(1., 1., 1., 1.),
    },
    gravity: vec2(0.0, 0.0),
    texture: None,
    atlas: None,
    material: None,
    post_processing: None,
};

impl Bullet{
    pub fn new(position:Vec2, velocity:Vec2, radius:f32, damage:f32, sprite_sheet:SpriteSheet, image_index: u32)->Bullet{
        let actor = Actor::new(position, radius);
        
        let mut config = TAIL_EMITTER;
        config.initial_direction = velocity.normalize();
        let tail_emitter = Emitter::new(config);

        config = IMPACT_EMITTER;
        config.initial_direction = velocity.normalize();
        let impact_emitter = Emitter::new(config);

        // spawn particles
        Bullet{
            actor,
            state:BulletState::Fly,
            velocity,
            tail_emitter,
            impact_emitter,
            sprite_sheet,
            image_index,
            damage,
            timer: 0.,
        }
    }

    pub fn update(&mut self, enemy_list:&mut Vec<Enemy>){
        if self.state == BulletState::Fly{
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
                    self.impact();
                    // return on first collision
                    return
                }
            }

            if self.actor.on_wall{
                self.impact();
            }
        }
        else {
            self.timer -= get_frame_time();
            if self.timer <= 0.{
                self.state = BulletState::Dead;
            }
        }
        
    }
    
    pub fn draw(&mut self){
        self.tail_emitter.draw(self.actor.position);
        if self.state == BulletState::Fly{
            self.sprite_sheet.draw(self.actor.position.x -8., self.actor.position.y -8., self.image_index, false, Color::new(0., 1., 1., 1.));
        }
        else{
            self.impact_emitter.draw(self.actor.position);
        }
    }

    fn impact(&mut self){
        self.state = BulletState::Impact;
        self.tail_emitter.config.emitting = false;
        //self.impact_emitter.config.emitting = true;
        self.impact_emitter.emit(self.actor.position, 25);
        self.timer = 0.5;
    }
}