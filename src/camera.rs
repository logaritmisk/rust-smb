use sdl2::rect::Rect;


pub struct Camera {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32
}

impl Camera {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Camera {
        Camera {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}
