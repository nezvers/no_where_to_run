use macroquad::prelude::vec2;

use crate::assets::Assets;
use crate::background::Background;
use crate::player::Player;
use crate::factory_enemy::FactoryEnemy;
use crate::factory_bullet::FactoryBullet;

pub struct Level{
    background:Background,
    player: Player,
    factory_enemy:FactoryEnemy,
    factory_bullet:FactoryBullet,
}

impl Level{
    pub fn new(assets:&Assets)->Level{
        let background = Background::new(assets.wall.clone(), assets.ground.clone(), assets.foliage.clone());
        let player = Player::new(vec2(64., 64.), assets.sprite_sheet.clone());
        let factory_enemy = FactoryEnemy::new(assets.sprite_sheet.clone());
        let factory_bullet = FactoryBullet::new(assets.sprite_sheet.clone());

        Level{
            background,
            player,
            factory_enemy,
            factory_bullet,
        }
    }

    pub fn update(&mut self){
        self.background.update();
        self.player.update();
        self.factory_enemy.update(&self.player);
        self.factory_bullet.update(&self.player, &mut self.factory_enemy.enemy_list);

        self.background.draw();
        self.factory_enemy.draw();
        self.player.draw();
        self.factory_bullet.draw();
    }
}