use macroquad::prelude::vec2;

use crate::assets::Assets;
use crate::background::Background;
use crate::player::Player;
use crate::spawner::Spawner;

pub struct Level{
    background:Background,
    player: Player,
    spawner:Spawner
}

impl Level{
    pub fn new(assets:&Assets)->Level{
        let background = Background::new(assets.wall.clone(), assets.ground.clone(), assets.foliage.clone());
        let player = Player::new(vec2(64., 64.), assets.sprite_sheet.clone());
        let spawner = Spawner::new(assets.sprite_sheet.clone());
    

        Level{
            background,
            player,
            spawner,
        }
    }

    pub fn update(&mut self){
        self.background.update();
        self.player.update();
        self.spawner.update(&self.player);

        self.background.draw();
        self.spawner.draw();
        self.player.draw();
    }
}