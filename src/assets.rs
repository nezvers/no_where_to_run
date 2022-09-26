
use macroquad::prelude::{Texture2D, load_texture, FilterMode};

use crate::sprite_sheet::SpriteSheet;

pub struct Assets{
    pub wall:Texture2D,
    pub ground:Texture2D,
    pub foliage:Texture2D,
    pub sprite_sheet:SpriteSheet,
}

impl Assets{
    pub async fn new()->Assets{
        let wall = load_texture("resources/wall.png").await.unwrap();
        wall.set_filter(FilterMode::Nearest);
        let ground = load_texture("resources/ground.png").await.unwrap();
        ground.set_filter(FilterMode::Nearest);
        let foliage = load_texture("resources/foliage.png").await.unwrap();
        foliage.set_filter(FilterMode::Nearest);
        let sprite_sheet = SpriteSheet::new("resources/characters.png", 16., 8).await;
        
        Assets { 
            wall,
            ground,
            foliage,
            sprite_sheet,
        }
    }
}