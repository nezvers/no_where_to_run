use macroquad::{prelude::{Vec2}, time::get_frame_time};
use crate::resolution::{GAME_WIDTH, GAME_HEIGHT};


pub struct Actor{
    pub position:Vec2,
    pub radius:f32,
    pub on_wall:bool,
}

impl Actor{
    pub fn new(position:Vec2, col_r:f32)->Actor{
        Actor{
            position,
            radius: col_r,
            on_wall: false,
        }
    }

    pub fn actor_move(&mut self, vel:&mut Vec2){
        // Wall horizontal
        let delta = get_frame_time();
        let mut vel_:Vec2 = vel.clone() * delta;

        if self.position.x + vel_.x < 15. + self.radius{
            self.on_wall = true;
            self.position.x = 15. + self.radius;
            vel_.x = 0.;
            vel.x = 0.;
        }
        else if self.position.x + vel_.x > GAME_WIDTH as f32 - 15. - self.radius{
            self.on_wall = true;
            self.position.x = (GAME_WIDTH as f32 - 15.) - self.radius;
            vel_.x = 0.;
            vel.x = 0.;
        }
        else{
            self.on_wall = false;
        }

        // Wall vertical
        if self.position.y + vel_.y < 15. + self.radius{
            self.on_wall = true;
            self.position.y = 15. + self.radius;
            vel_.y = 0.;
            vel.y = 0.;
        }
        else if self.position.y + vel_.y > GAME_HEIGHT as f32 - 15. - self.radius{
            self.on_wall = true;
            self.position.y = (GAME_HEIGHT as f32 - 15.) - self.radius;
            vel_.y = 0.;
            vel.y = 0.;
        }
        else if self.on_wall{
            self.on_wall = false;
        }
        self.position += vel_;

    }


}