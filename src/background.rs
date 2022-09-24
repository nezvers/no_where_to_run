use macroquad::prelude::{Texture2D, load_texture, Color, clear_background, draw_texture, FilterMode};

pub struct Background{
    pub wall:Texture2D,
    pub ground:Texture2D,
    pub foliage:Texture2D,
    pub bgColor:Color,
    pub wallColor:Color,
    pub groundColor:Color,
    pub foliageColor:Color,
}

impl Background{
    pub async fn new()->Self{
        let wall = load_texture("resources/wall.png").await.unwrap();
        wall.set_filter(FilterMode::Nearest);
        let ground = load_texture("resources/ground.png").await.unwrap();
        ground.set_filter(FilterMode::Nearest);
        let foliage = load_texture("resources/foliage.png").await.unwrap();
        foliage.set_filter(FilterMode::Nearest);

        Background{
            wall: wall,
            ground: ground,
            foliage: foliage,
            bgColor: Color::new(0., 0., 0., 1.),
            wallColor: Color::new(1., 1., 1., 1.),
            groundColor: Color::new(1., 1., 1., 0.2),
            foliageColor: Color::new(0., 1., 0., 1.),
        }
    }

    pub fn clear(&self){
        clear_background(self.bgColor);
    }

    pub fn draw(&self){
        draw_texture(
            self.wall,
            0.0,
            0.0,
            self.wallColor,
        );
        draw_texture(
            self.ground,
            0.0,
            0.0,
            self.groundColor,
        );
        draw_texture(
            self.foliage,
            0.0,
            0.0,
            self.foliageColor,
        );
    }
}
