use sdl2::rect::Rect;


pub struct Camera {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    bounding: Rect
}

impl Camera {
    pub fn new(x: i32, y: i32, w: u32, h: u32, bounding: Rect) -> Camera {
        Camera {
            x: x,
            y: y,
            w: w,
            h: h,
            bounding: bounding
        }
    }

    pub fn center(&mut self, object: &Rect) {
        let mut x = (object.x() + object.width() as i32 / 2) - (self.w as i32 / 2);
        let mut y = (object.y() + object.height() as i32 / 2) - (self.h as i32 / 2);

        if x < self.bounding.x() {
            x = self.bounding.x();
        } else if x + self.w as i32 > self.bounding.width() as i32 {
            x = (self.bounding.width() - self.w) as i32;
        }

        if y < self.bounding.y() {
            y = self.bounding.y();
        } else if y + self.h as i32 > self.bounding.height() as i32 {
            y = (self.bounding.height() - self.h) as i32;
        }

        self.x = x;
        self.y = y;
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new_unwrap(self.x, self.y, self.w, self.h)
    }
}
