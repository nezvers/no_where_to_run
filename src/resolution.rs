use macroquad::prelude::{Camera2D, vec2, Rect, screen_width, screen_height, set_camera};

pub const GAME_WIDTH:i32 = 624;
pub const GAME_HEIGHT:i32 = 352;

pub struct Resolution{
    camera:Camera2D,
}

impl Resolution{
    pub fn new()->Self{
        let camera = new_camera();
        
        Resolution {
            camera,
        }
    }

    pub fn update(&self){
        // * place for window dynamic resizing
        set_camera(&self.camera);
    }
}

pub fn get_zoom() -> f32 {
    let z_w = screen_width() / GAME_WIDTH as f32;
    let z_h = screen_height() / GAME_HEIGHT as f32;
    let zoom = if z_w < z_h && z_w > 1.0{
        z_w as u32 as f32
    }
    else if z_w > z_h && z_h > 1.0{
        z_h as u32 as f32
    }
    else{
        1.
    };
    zoom
}

pub fn new_camera()->Camera2D{
    let zoom = (screen_width() as i32 / GAME_WIDTH).min(screen_height() as i32 / GAME_HEIGHT);
    let zoomed_w = GAME_WIDTH * zoom;
    let zoomed_h = GAME_HEIGHT * zoom;
    let offset_x = (screen_width() as i32 - zoomed_w) / 2;
    let offset_y = (screen_height() as i32 - zoomed_h) / 2;
    Camera2D {
        target: vec2((GAME_WIDTH / 2) as f32, (GAME_HEIGHT / 2) as f32),
        zoom: vec2(2. / GAME_WIDTH as f32, -2. / GAME_HEIGHT as f32),
        viewport: Some((offset_x, offset_y, GAME_WIDTH * zoom, GAME_HEIGHT * zoom)),
        ..Default::default()
    }
}

pub fn new_camera_top_left()->Camera2D{
    let zoom = (screen_width() as i32 / GAME_WIDTH).min(screen_height() as i32 / GAME_HEIGHT);
    let zoomed_w = GAME_WIDTH * zoom;
    let zoomed_h = GAME_HEIGHT * zoom;
    Camera2D {
        target: vec2((GAME_WIDTH / 2) as f32, (GAME_HEIGHT / 2) as f32),
        zoom: vec2(2. / GAME_WIDTH as f32, -2. / GAME_HEIGHT as f32),
        viewport: Some((0, screen_height() as i32 - zoomed_h, zoomed_w, zoomed_h)),
        ..Default::default()
    }
}

pub fn olf_camera()->Camera2D{
    let zoom = get_zoom();
    let scale_x = screen_width() / (GAME_WIDTH as f32 * zoom);
    let scale_y = screen_height() / (GAME_HEIGHT as f32 * zoom);
    Camera2D::from_display_rect(Rect::new(0., 0., GAME_WIDTH as f32 * scale_x, GAME_HEIGHT as f32 * scale_y))
}