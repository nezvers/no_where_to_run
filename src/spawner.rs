use macroquad::prelude::{vec2, Vec2, Color, get_frame_time};
use crate::player::Player;
use crate::enemy::Enemy;
use crate::sprite_sheet::SpriteSheet;

pub struct Spawner{
    max_enemies:usize,
    enemy_list:Vec<Enemy>,
}

impl Spawner{
    pub fn new(sprite_sheet:SpriteSheet)->Spawner{
        let max_enemies = 10;
        let mut enemy_list:Vec<Enemy> = Vec::new();
        //enemy_list.reserve(max_enemies);
        
        

        for (_index, i) in (0..max_enemies).enumerate(){
            let enemy = Enemy::new(sprite_sheet.clone(), 45 );
            // maybe give id to enemy
            enemy_list.push(enemy);
        }
        Spawner{
            max_enemies,
            enemy_list,
        }
    }

    pub fn update(&mut self, player:&Player){
        for enemy in self.enemy_list.iter_mut(){
            enemy.update(player);
        }
    }

    pub fn draw(&mut self){
        for enemy in self.enemy_list.iter_mut(){
            enemy.draw();
        }
    }

}