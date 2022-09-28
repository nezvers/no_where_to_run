use crate::{bullet::{Bullet, BulletState}, player::Player, enemy::{Enemy, EnemyState}, sprite_sheet::SpriteSheet};
use macroquad::prelude::{get_frame_time, Vec2};

pub struct FactoryBullet{
    bullet_list: Vec<Bullet>,
    interval:f32, // shooting
    time:f32, // timer
    sprite_sheet:SpriteSheet, // pass to bullets
}

impl FactoryBullet{
    pub fn new(sprite_sheet:SpriteSheet)->FactoryBullet{
        let bullet_list:Vec<Bullet> = Vec::new();
        let interval = 2.0;
        FactoryBullet{
            bullet_list,
            interval,
            time: interval,
            sprite_sheet,
        }
    }

    pub fn update(&mut self, player:&Player, enemy_list:&mut Vec<Enemy>){
        if self.time > 0.{
            self.time -= get_frame_time();
        }
        else{
            self.shoot_logic(player, enemy_list);
        }

        let mut update_list = false;
        for bullet in self.bullet_list.iter_mut(){
            bullet.update(enemy_list);
            if bullet.state == BulletState::Dead{
                update_list = true;
            }
        }

        if update_list{
            self.bullet_list.retain(|bullet| bullet.state != BulletState::Dead);
        }
    }

    pub fn draw(&mut self){
        for bullet in self.bullet_list.iter_mut(){
            bullet.draw();
        }
    }

    pub fn shoot_logic(&mut self, player:&Player, enemy_list:&mut Vec<Enemy>){
        let mut dist = 1000.;
        let mut index = -1;

        // find closest enemy
        for (i, enemy) in enemy_list.iter().enumerate() {
            if enemy.state != EnemyState::Attack{
                continue;
            }
            let dist_current = (player.actor.position - enemy.actor.position).length();
            if dist_current < dist{
                index = i as i32;
                dist = dist_current;
            }
        }
        
        // if there is at least one enemy that's chosen
        if index > -1{
            self.time = self.interval;
            let enemy = enemy_list.get_mut(index as usize).unwrap();
            let position = player.actor.position;
            let dir = (enemy.actor.position - player.actor.position).normalize();
            self.new_bullet(position, dir);
        }
    }
    
    pub fn new_bullet(&mut self, position:Vec2, dir:Vec2){
        // temporary property place
        let speed = 60. * 5.;
        let radius = 4.;
        let damage = 3.;
        let bullet = Bullet::new(position, dir * speed, radius, damage, self.sprite_sheet.clone(), 69);
        self.bullet_list.push(bullet);
    }

}