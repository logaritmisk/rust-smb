use sdl2::rect::Rect;


pub struct Player {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
    pub dx: f32,
    pub dy: f32,
    pub gravity: f32,
    pub on_ground: bool
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            x: x,
            y: y,
            w: 32,
            h: 32,
            dx: 0.0,
            dy: 0.0,
            gravity: 0.3,
            on_ground: false
        }
    }

    pub fn update(&mut self) {
        self.dy += self.gravity;

        if self.dy > 8.0 {
            self.dy = 8.0;
        } else if self.dy < -8.0 {
            self.dy = -8.0;
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new_unwrap(self.x as i32, self.y as i32, self.w, self.h)
    }
}
