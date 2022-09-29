use macroquad::prelude::{get_frame_time};


use crate::assets::Assets;
use crate::background::Background;
use crate::player::Player;
use crate::enemy::{Enemy, EnemyState};

pub struct FactoryEnemy{
    pub enemy_list:Vec<Enemy>,
    max_enemies:usize,
    pub kill_count:usize,
    assets:Assets,
    interval:f32,
    time:f32,
}

impl FactoryEnemy{
    pub fn new(assets:Assets)->FactoryEnemy{
        let max_enemies = 1;
        let enemy_list:Vec<Enemy> = Vec::new();
        let interval = 2.;
        
        FactoryEnemy{
            enemy_list,
            max_enemies,
            kill_count: 0,
            assets,
            interval,
            time: interval,
        }
    }
    /// Update tick - interval counting, enemy update tick, enemy list update, kill counting
    pub fn update(&mut self, player:&mut Player, background:&mut Background){
        if self.time > 0.{
            self.time -= get_frame_time();
        }
        else{
            self.spawn_logic();
        }
        
        let mut update_list = false;
        for enemy in self.enemy_list.iter_mut(){
            enemy.update(player, background);
            if enemy.state == EnemyState::Dead{
                update_list = true;
            }
        }

        if update_list{
            let mut count = self.enemy_list.len();
            self.enemy_list.retain(|enemy| enemy.state != EnemyState::Dead);

            if self.enemy_list.len() < count{
                count = count - self.enemy_list.len();
                self.kill_count += count;
                self.max_enemies = (self.kill_count / 3) + 1;
            }
        }
        
    }

    /// Draw tick passed to all enemy instances
    pub fn draw(&mut self){
        for enemy in self.enemy_list.iter_mut(){
            enemy.draw();
        }
    }
    /// Check if active enemy count is at maximum
    fn spawn_logic(&mut self){
        if self.enemy_list.len() < self.max_enemies{
            self.time = self.interval;
            self.new_enemy();
        }
    }
    
    /// Create new enemy instance and add to an enemy list
    fn new_enemy(&mut self){
        let enemy = Enemy::new(self.assets.clone(), 45 );
        self.enemy_list.push(enemy);
    }

}