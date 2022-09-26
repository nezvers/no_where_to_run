use crate::player::Player;
use crate::enemy::Enemy;
use crate::sprite_sheet::SpriteSheet;

pub struct FactoryEnemy{
    pub enemy_list:Vec<Enemy>,
    //max_enemies:usize, // upgradable difficulity
}

impl FactoryEnemy{
    pub fn new(sprite_sheet:SpriteSheet)->FactoryEnemy{
        let max_enemies = 10;
        let mut enemy_list:Vec<Enemy> = Vec::new();
        //enemy_list.reserve(max_enemies);
        
        for (_index, _i) in (0..max_enemies).enumerate(){
            let enemy = Enemy::new(sprite_sheet.clone(), 45 );
            // maybe give id to enemy
            enemy_list.push(enemy);
        }
        FactoryEnemy{
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