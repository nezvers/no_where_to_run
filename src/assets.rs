
use macroquad::{prelude::{Texture2D, load_texture, FilterMode}, text::{Font, load_ttf_font}, audio::{self, Sound}};

use crate::sprite_sheet::SpriteSheet;
#[derive(Clone)]
pub struct Assets{
    pub wall:Texture2D,
    pub ground:Texture2D,
    pub foliage:Texture2D,
    pub sprite_sheet:SpriteSheet,
    pub font:Font,
    pub bullet_shoot:Sound,
    pub bullet_impact:Sound,
    pub enemy_created:Sound,
    pub enemy_spawned:Sound,
    pub enemy_death:Sound,
    pub player_damage:Sound,
    pub player_death:Sound,
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
        
        let font = load_ttf_font("resources/pixellocale-v-1-4.ttf").await.unwrap();

        let bullet_shoot = audio::load_sound("resources/bullet_shoot_2.wav").await.unwrap();
        let bullet_impact = audio::load_sound("resources/bullet_impact.wav").await.unwrap();
        let enemy_created = audio::load_sound("resources/enemy_created.wav").await.unwrap();
        let enemy_spawned = audio::load_sound("resources/enemy_spawned.wav").await.unwrap();
        let enemy_death = audio::load_sound("resources/enemy_death.wav").await.unwrap();
        let player_damage = audio::load_sound("resources/player_damage.wav").await.unwrap();
        let player_death = audio::load_sound("resources/player_death.wav").await.unwrap();

        Assets { 
            wall,
            ground,
            foliage,
            sprite_sheet,
            font,
            bullet_shoot,
            bullet_impact,
            enemy_created,
            enemy_spawned,
            enemy_death,
            player_damage,
            player_death,
        }
    }
}