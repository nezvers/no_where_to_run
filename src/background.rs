use macroquad::{prelude::{Texture2D, Color, clear_background, draw_texture}, time::get_frame_time};

pub const FLASH_TIME:f32 = 0.5;
pub const FLASH_COL:Color = Color::new(0.576, 0.219, 0.560, 1.);

pub struct Background{
    pub wall:Texture2D,
    pub ground:Texture2D,
    pub foliage:Texture2D,
    pub bg_col:Color,
    pub wall_col:Color,
    pub ground_col:Color,
    pub folliage_col:Color,
    timer:f32,
}

impl Background{
    pub fn new(wall:Texture2D, ground:Texture2D, foliage:Texture2D)->Self{
        Background{
            wall,
            ground,
            foliage,
            bg_col: Color::new(0., 0., 0., 1.),
            wall_col: Color::new(1., 1., 1., 1.),
            ground_col: Color::new(1., 1., 1., 0.2),
            folliage_col: Color::new(0., 1., 0., 1.),
            timer: 0.,
        }
    }

    pub fn update(&mut self){
        if self.timer > 0.{
            self.timer -= get_frame_time();
            if self.timer < 0.{
                self.timer = 0.;
            }
        }
    }

    pub fn draw(&self){
        let t = self.timer / FLASH_TIME;
        clear_background(self.mix(self.bg_col, FLASH_COL, t * t * t));
        draw_texture(
            self.wall,
            0.0,
            0.0,
            self.wall_col,
        );
        draw_texture(
            self.ground,
            0.0,
            0.0,
            self.ground_col,
        );
        draw_texture(
            self.foliage,
            0.0,
            0.0,
            self.folliage_col,
        );
    }

    pub fn damage(&mut self){
        self.timer = 0.5;
    }

    fn mix(&self, a:Color, b:Color, t:f32)->Color{
        Color::new(
            a.r + (b.r - a.r) * t, 
            a.g + (b.g - a.g) * t, 
            a.b + (b.b - a.b) * t, 
            a.a + (b.a - a.a) * t,
        )
    }
}
