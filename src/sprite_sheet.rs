use macroquad::prelude::{Texture2D, load_texture, Color, draw_texture_ex, DrawTextureParams, vec2, Vec2, Rect, FilterMode};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteSheet{
    texture: Texture2D,
    sprite_size: f32,
    width: u32, // columns in sprite sheet
    rect:Rect,
}

impl SpriteSheet{
    pub async fn new(path:&str , sprite_size:f32, width:u32)->Self{
        let texture = load_texture(path).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let rect = Rect{x:0., y:0., w: sprite_size, h:sprite_size};
        let drawParam = DrawTextureParams{
            dest_size: Some(vec2(sprite_size, sprite_size)),
            source: Some(rect),
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: Some(Vec2::ZERO),
        };

        SpriteSheet{
            texture,
            sprite_size,
            width,
            rect,
        }
    }

    pub fn draw(&mut self, posX:f32, posY:f32, index: u32, flip_x:bool, color:Color){
        let x = (index % self.width) as f32 * self.sprite_size;
        let y = (index / self.width) as f32 * self.sprite_size;
        //let rect = Rect{x, y, w: self.sprite_size, h: self.sprite_size};
        self.rect.x = x;
        self.rect.y = y;
        let drawParam = DrawTextureParams{
            dest_size: Some(vec2(self.sprite_size, self.sprite_size)),
            source: Some(self.rect),
            rotation: 0.,
            flip_x,
            flip_y: false,
            pivot: Some(Vec2::ZERO),
        };

        draw_texture_ex(self.texture, posX, posY, color, drawParam);
    }
}