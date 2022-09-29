use macroquad::prelude::{get_frame_time, Vec2};


use crate::background::Background;
use crate::player::Player;
use crate::enemy::{Enemy, EnemyState};
use crate::sprite_sheet::SpriteSheet;

pub struct FactoryEnemy{
    pub enemy_list:Vec<Enemy>,
    max_enemies:usize,
    pub kill_count:usize,
    sprite_sheet:SpriteSheet,
    interval:f32,
    time:f32,
    //kill_emitter:Emitter,
    kill_pos:Vec2,
}

impl FactoryEnemy{
    pub fn new(sprite_sheet:SpriteSheet)->FactoryEnemy{
        let max_enemies = 1;
        let enemy_list:Vec<Enemy> = Vec::new();
        let interval = 2.;
        // let kill_emitter = Emitter::new(EmitterConfig {
        //     lifetime: 0.5,
        //     one_shot: true,
        //     amount: 5,
        //     initial_direction_spread: 0.0,
        //     initial_velocity: -50.0,
        //     size: 2.0,
    
        //     blend_mode: BlendMode::Alpha,
        //     ..Default::default()
        // });
        
        FactoryEnemy{
            enemy_list,
            max_enemies,
            kill_count: 0,
            sprite_sheet,
            interval,
            time: interval,
            //kill_emitter,
            kill_pos: Vec2::ZERO,
        }
    }

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
            self.enemy_list.retain(|enemy| 
                if enemy.state != EnemyState::Dead{
                    true
                }
                else{
                    self.kill_pos = enemy.actor.position;
                    //self.kill_emitter.emit(Vec2::ZERO, 5);
                    false
                }
            );

            if self.enemy_list.len() < count{
                count = count - self.enemy_list.len();
                self.kill_count += count;
                self.max_enemies = (self.kill_count / 3) + 1;
            }
        }
        
    }

    pub fn draw(&mut self){
        for enemy in self.enemy_list.iter_mut(){
            enemy.draw();
        }

        //self.kill_emitter.draw(self.kill_pos);
    }

    fn spawn_logic(&mut self){
        if self.enemy_list.len() < self.max_enemies{
            self.time = self.interval;
            self.new_enemy();
        }
    }

    fn new_enemy(&mut self){
        let enemy = Enemy::new(self.sprite_sheet.clone(), 45 );
            // maybe give id to enemy
        self.enemy_list.push(enemy);
    }

}