use macroquad::prelude::{Camera2D, vec2, Rect, screen_width, screen_height, set_camera, render_target, Vec2};


pub struct Resolution{
    width:f32,
    height:f32,
    zoom:f32,
    camera:Camera2D,
}

impl Resolution{
    pub fn new()->Self{
        let gameW = 624.;
        let gameH = 352.;
        let zoom = get_zoom();

        let camera = new_camera(gameW as i32, gameH as i32);
        
        Resolution { 
            width: gameW, 
            height: gameH,
            zoom,
            camera,
        }
    }

    pub fn update(&self){
        // adjust zoom here //
        set_camera(&self.camera);
    }
}

pub fn get_zoom() -> f32 {
    let gameW = 624.;
    let gameH = 352.;
    let zW = screen_width() / gameW;
    let zH = screen_height() / gameH;
    let zoom = if zW < zH && zW > 1.0{
        zW as u32 as f32
    }
    else if zW > zH && zH > 1.0{
        zH as u32 as f32
    }
    else{
        1.
    };
    zoom
}

pub fn new_camera(game_width:i32, game_height:i32)->Camera2D{
    let zoom = (screen_width() as i32 / game_width).min(screen_height() as i32 / game_height);
    let zoomed_w = game_width * zoom;
    let zoomed_h = game_height * zoom;
    let offset_x = (screen_width() as i32 - zoomed_w) / 2;
    let offset_y = (screen_height() as i32 - zoomed_h) / 2;
    Camera2D {
        target: vec2((game_width / 2) as f32, (game_height / 2) as f32),
        zoom: vec2(2. / game_width as f32, -2. / game_height as f32),
        viewport: Some((offset_x, offset_y, game_width * zoom, game_height * zoom)),
        ..Default::default()
    }
}

pub fn new_camera_top_left(game_width:i32, game_height:i32)->Camera2D{
    let zoom = (screen_width() as i32 / game_width).min(screen_height() as i32 / game_height);
    let zoomed_w = game_width * zoom;
    let zoomed_h = game_height * zoom;
    Camera2D {
        target: vec2((game_width / 2) as f32, (game_height / 2) as f32),
        zoom: vec2(2. / game_width as f32, -2. / game_height as f32),
        viewport: Some((0, screen_height() as i32 - zoomed_h, zoomed_w, zoomed_h)),
        ..Default::default()
    }
}

pub fn new_camera_old(game_width:f32, game_height:f32)->Camera2D{
    let zoom = get_zoom();
    let scaleX = screen_width() / (game_width * zoom);
    let scaleY = screen_height() / (game_height * zoom);
    Camera2D::from_display_rect(Rect::new(0., 0., game_width * scaleX, game_height * scaleY))
}