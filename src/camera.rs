use sdl2::rect::Rect;


pub struct Camera {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    bounding: Rect
}

impl Camera {
    pub fn new(x: i32, y: i32, w: i32, h: i32, bounding: Rect) -> Camera {
        Camera {
            x: x,
            y: y,
            w: w,
            h: h,
            bounding: bounding
        }
    }

    pub fn center(&mut self, object: &Rect) {
        let mut x = (object.x + object.w / 2) - (self.w / 2);
        let mut y = (object.y + object.h / 2) - (self.h / 2);

        if x < self.bounding.x {
            x = self.bounding.x;
        } else if x + self.w > self.bounding.w {
            x = self.bounding.w - self.w;
        }

        if y < self.bounding.y {
            y = self.bounding.y;
        } else if y + self.h > self.bounding.h {
            y = self.bounding.h - self.h;
        }

        self.x = x;
        self.y = y;
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}
