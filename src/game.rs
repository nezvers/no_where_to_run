
use macroquad::prelude::set_default_camera;

use crate::assets::Assets;
use crate::level::Level;

pub struct Game{
    assets:Assets,
    level:Level,
}

impl Game{
    pub async fn new()->Game{
        let assets = Assets::new().await;
        let level = Level::new(&assets);
        Game { 
            assets,
            level,
        }
    }

    pub fn update(&mut self){
        self.level.update();
        
        set_default_camera();
        //GUI
    }
}