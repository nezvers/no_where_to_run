use macroquad::prelude::{vec2, Color};
use macroquad::shapes::draw_rectangle;
use macroquad::text::{draw_text_ex, TextParams, Font};

use crate::assets::Assets;
use crate::background::Background;
use crate::player::{Player, PlayerState};
use crate::factory_enemy::FactoryEnemy;
use crate::factory_bullet::FactoryBullet;

pub struct Level{
    assets:Assets,
    background: Background,
    player: Player,
    factory_enemy: FactoryEnemy,
    factory_bullet: FactoryBullet,
    font:Font,
}

impl Level{
    pub fn new(assets:Assets)->Level{
        let background = Background::new(assets.wall.clone(), assets.ground.clone(), assets.foliage.clone());
        let player = Player::new(vec2(64., 64.), assets.sprite_sheet.clone());
        let factory_enemy = FactoryEnemy::new(assets.sprite_sheet.clone());
        let factory_bullet = FactoryBullet::new(assets.sprite_sheet.clone());
        let font = assets.font;
        

        Level{
            assets,
            background,
            player,
            factory_enemy,
            factory_bullet,
            font,
        }
    }

    pub fn update(&mut self){
        self.background.update();
        self.player.update();
        self.factory_enemy.update(&mut self.player, &mut self.background);
        self.factory_bullet.update(&self.player, &mut self.factory_enemy.enemy_list);

        if self.player.state == PlayerState::Remove{
            self.restart();
        }

        self.background.draw();
        self.factory_enemy.draw();
        self.player.draw();
        self.factory_bullet.draw();
        
        
        //set_default_camera();
        //GUI

        let text = format!("HP:{}  Kill:{}", self.player.health, self.factory_enemy.kill_count);
        let s = text.as_str();
        let p = vec2(0., 13.);

        draw_rectangle(p.x, p.y -11., 72., 13., Color::new(0., 0., 0., 1.));
        
        let mut text_params = TextParams{
            font_size: 9 * 10,
            font_scale: 0.1,
            font:self.font,
            color:Color::new(1., 0., 0.5, 1.),
            ..Default::default()
        };
        draw_text_ex(s, p.x, p.y, text_params);

    }

    fn restart(&mut self){
        let level = Level::new(self.assets.clone());
        self.background = level.background;
        self.player = level.player;
        self.factory_enemy = level.factory_enemy;
        self.factory_bullet = level.factory_bullet;
    }
}