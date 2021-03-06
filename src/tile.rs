use sdl2::rect::Rect;


pub struct Layer<T> {
    tiles: Vec<Option<T>>,
    default: T,
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32
}

impl<T> Layer<T> where T: Clone {
    // TODO Change u32 to usize for width and height?
    pub fn new(width: u32, height: u32, tile_width: u32, tile_height: u32, tile: T) -> Layer<T> {
        Layer {
            tiles: vec![None; (width * height) as usize],
            default: tile,
            width: width,
            height: height,
            tile_width: tile_width,
            tile_height: tile_height
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &T {
        let offset = (x + y * self.width as i32) as usize;

        match *self.tiles.get(offset).unwrap() {
            Some(ref tile) => tile,
            None           => &self.default
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: T) {
        let offset = (x + y * self.width as i32) as usize;

        self.tiles[offset] = Some(tile);
    }

    pub fn find_intersecting(&self, rect: &Rect) -> Option<Rect> {
        if rect.x() + rect.width() as i32 <= 0 || rect.x() >= (self.width * self.tile_width) as i32 {
            return None;
        }
        if rect.y() + rect.height() as i32 <= 0 || rect.y() >= (self.height * self.tile_height) as i32 {
            return None;
        }

        let x1 = rect.x() / self.tile_width as i32;
        let y1 = rect.y() / self.tile_height as i32;

        let x2 = (rect.x() + rect.width() as i32 - 1) / self.tile_width as i32;
        let y2 = (rect.y() + rect.height() as i32 - 1) / self.tile_height as i32;

        Some(Rect::new(x1, y1, (x2 - x1 + 1) as u32, (y2 - y1 + 1) as u32))
    }

    pub fn for_each_intersecting<F: FnMut(&T, &Rect)>(&self, rect: &Rect, mut f: F) {
        if let Some(intersect) = self.find_intersecting(rect) {
            for y in intersect.y()..(intersect.y() + intersect.height() as i32)  {
                for x in intersect.x()..(intersect.x() + intersect.width() as i32) {
                    let position = Rect::new(x * self.tile_width as i32, y * self.tile_height as i32, self.tile_width, self.tile_height);

                    f(self.get_tile(x, y), &position);
                }
            }
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(0, 0, self.width * self.tile_width, self.height * self.tile_height)
    }
}

#[cfg(test)]
mod tests {
    use sdl2::rect::Rect;

    use super::*;

    #[test]
    fn layer_find_intersecting() {
        let layer = Layer::new(3, 3, 3, 3, ());

        // out of bounds.
        assert_eq!(layer.find_intersecting(&Rect::new(-1,  1, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new( 1, -1, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new(-1, -1, 1, 1)), None);

        assert_eq!(layer.find_intersecting(&Rect::new(9, 7, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new(7, 9, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new(9, 9, 1, 1)), None);

        // middle of tile.
        assert_eq!(layer.find_intersecting(&Rect::new(1, 1, 1, 1)), Some(Rect::new(0, 0, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(4, 1, 1, 1)), Some(Rect::new(1, 0, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(7, 1, 1, 1)), Some(Rect::new(2, 0, 1, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new(1, 4, 1, 1)), Some(Rect::new(0, 1, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(4, 4, 1, 1)), Some(Rect::new(1, 1, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(7, 4, 1, 1)), Some(Rect::new(2, 1, 1, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new(1, 7, 1, 1)), Some(Rect::new(0, 2, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(4, 7, 1, 1)), Some(Rect::new(1, 2, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(7, 7, 1, 1)), Some(Rect::new(2, 2, 1, 1)));

        // interlaps 4 tiles.
        assert_eq!(layer.find_intersecting(&Rect::new(2, 2, 2, 2)), Some(Rect::new(0, 0, 2, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new(5, 2, 2, 2)), Some(Rect::new(1, 0, 2, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new(2, 5, 2, 2)), Some(Rect::new(0, 1, 2, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new(5, 5, 2, 2)), Some(Rect::new(1, 1, 2, 2)));

        // interlaps 2 tiles horizontal.
        assert_eq!(layer.find_intersecting(&Rect::new(2, 1, 2, 1)), Some(Rect::new(0, 0, 2, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(5, 1, 2, 1)), Some(Rect::new(1, 0, 2, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new(2, 4, 2, 1)), Some(Rect::new(0, 1, 2, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(5, 4, 2, 1)), Some(Rect::new(1, 1, 2, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new(2, 7, 2, 1)), Some(Rect::new(0, 2, 2, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new(5, 7, 2, 1)), Some(Rect::new(1, 2, 2, 1)));

        // interlaps 2 tiles vertical.
        assert_eq!(layer.find_intersecting(&Rect::new(1, 2, 1, 2)), Some(Rect::new(0, 0, 1, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new(1, 5, 1, 2)), Some(Rect::new(0, 1, 1, 2)));

        assert_eq!(layer.find_intersecting(&Rect::new(4, 2, 1, 2)), Some(Rect::new(1, 0, 1, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new(4, 5, 1, 2)), Some(Rect::new(1, 1, 1, 2)));

        assert_eq!(layer.find_intersecting(&Rect::new(7, 2, 1, 2)), Some(Rect::new(2, 0, 1, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new(7, 5, 1, 2)), Some(Rect::new(2, 1, 1, 2)));

        // exactly one tile.
        assert_eq!(layer.find_intersecting(&Rect::new(0, 0, 3, 3)), Some(Rect::new(0, 0, 1, 1)));
    }
}
