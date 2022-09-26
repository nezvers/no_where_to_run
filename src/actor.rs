use macroquad::prelude::{Vec2};
use crate::resolution::{GAME_WIDTH, GAME_HEIGHT};


pub struct Actor{
    pub position:Vec2,
    col_r:f32,
    on_wall:bool,
}

impl Actor{
    pub fn new(position:Vec2, col_r:f32)->Actor{
        Actor{
            position,
            col_r,
            on_wall: false,
        }
    }

    pub fn actor_move(&mut self, vel:&mut Vec2){
        // Wall horizontal
        if self.position.x + vel.x < 15. + self.col_r{
            self.on_wall = true;
            self.position.x = 15. + self.col_r;
            vel.x = 0.;
        }
        else if self.position.x + vel.x > GAME_WIDTH as f32 - 15. - self.col_r{
            self.on_wall = true;
            self.position.x = (GAME_WIDTH as f32 - 15.) - self.col_r;
            vel.x = 0.;
        }
        else{
            self.on_wall = false;
        }

        // Wall vertical
        if self.position.y + vel.y < 15. + self.col_r{
            self.on_wall = true;
            self.position.y = 15. + self.col_r;
            vel.y = 0.;
        }
        else if self.position.y + vel.y > GAME_HEIGHT as f32 - 15. - self.col_r{
            self.on_wall = true;
            self.position.y = (GAME_HEIGHT as f32 - 15.) - self.col_r;
            vel.y = 0.;
        }
        else if self.on_wall{
            self.on_wall = false;
        }
        self.position.x += vel.x;
        self.position.y += vel.y;

    }


}