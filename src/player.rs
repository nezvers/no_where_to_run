use macroquad::prelude::{Vec2, Color, get_frame_time};
use crate::actor::Actor;
use crate::sprite_sheet::SpriteSheet;
use crate::input::PlayerInput;

const INVINC_TIME:f32 = 1.;

#[derive(PartialEq)]
pub enum PlayerState{
    Active,
    Dead,
    Remove,
}

pub struct Player{
    pub actor:Actor,
    sprite_sheet:SpriteSheet,
    image_index:f32,
    velocity:Vec2,
    speed:f32,
    pub health:f32,
    timer:f32,
    pub state: PlayerState,
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
            health: 3.,
            timer: 0.,
            state:PlayerState::Active,
        }
    }

    pub fn update(&mut self){
        match self.state{
            PlayerState::Active => self.moving(),
            PlayerState::Dead => self.be_dead(),
            PlayerState::Remove => {},
        }
    }

    pub fn draw(&mut self){
        let pos_x = self.actor.position.x -8.;
        let pos_y = self.actor.position.y -8.;
        if self.state == PlayerState::Active{
            self.sprite_sheet.draw(pos_x, pos_y, 0, false, Color::new(1., 1., 1., 1.));
        }
        else{
            self.sprite_sheet.draw(pos_x, pos_y, 3, false, Color::new(1., 1., 1., 1.));
        }
    }

    pub fn moving(&mut self){
        let delta = get_frame_time();
        if self.timer > 0.{
            self.timer -= delta;
        }

        let dir = PlayerInput::get_dir().normalize_or_zero();
        self.velocity = self.velocity.lerp(self.speed * dir, 15. * delta);

        self.actor.actor_move(&mut self.velocity);
    }

    pub fn be_dead(&mut self){
        let delta = get_frame_time();
        self.timer -= delta;
        if self.timer <= 0.{
            println!("Become REMOVE");
            self.state = PlayerState::Remove;
        }
    }

    pub fn damage(&mut self, value:f32, dir:Vec2)->bool{
        if self.timer > 0. {return false}
        self.timer = INVINC_TIME;
        self.health -= value;
        self.velocity = 750. * dir;
        if self.health <= 0.{
            self.health = 0.;
            self.state = PlayerState::Dead;
        }

        true
    }
}