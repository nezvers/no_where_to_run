use macroquad::prelude::{Texture2D, Color, clear_background, draw_texture};

pub struct Background{
    pub wall:Texture2D,
    pub ground:Texture2D,
    pub foliage:Texture2D,
    pub bg_col:Color,
    pub wall_col:Color,
    pub ground_col:Color,
    pub folliage_col:Color,
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
        }
    }

    pub fn update(&self){}

    pub fn draw(&self){
        clear_background(self.bg_col);
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
}
