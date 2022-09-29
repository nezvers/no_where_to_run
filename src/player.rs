use macroquad::audio::{Sound, play_sound_once};
use macroquad::prelude::{Vec2, Color, get_frame_time};
use crate::actor::Actor;
use crate::assets::Assets;
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
    pub state: PlayerState,
    pub actor:Actor,
    pub health:u32,
    sprite_sheet:SpriteSheet,
    image_index:f32,
    velocity:Vec2,
    speed:f32,
    timer:f32,
    player_damage:Sound,
    player_death:Sound,
}

impl Player{
    pub fn new(position:Vec2, assets:Assets)->Player{
        let actor = Actor::new(position, 8.);

        Player{
            state:PlayerState::Active,
            actor,
            health: 3,
            sprite_sheet:assets.sprite_sheet,
            image_index: 0.,
            velocity: Vec2::ZERO,
            speed: 120.,
            timer: 0.,
            player_damage:assets.player_damage,
            player_death:assets.player_death,
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
            self.state = PlayerState::Remove;
        }
    }

    pub fn damage(&mut self, value:f32, dir:Vec2)->bool{
        if self.timer > 0. {return false}
        self.timer = INVINC_TIME;
        self.health -= value as u32;
        self.velocity = 750. * dir;
        if self.health <= 0{
            self.health = 0;
            self.state = PlayerState::Dead;
            play_sound_once(self.player_death);
        }
        else{
            play_sound_once(self.player_damage);
        }

        true
    }
}