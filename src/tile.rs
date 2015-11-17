use std::iter::repeat;
//use std::cmp::{min, max};

use sdl2::rect::Rect;


pub struct Layer<T> {
    tiles: Vec<T>,
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32
}

impl<T> Layer<T> where T: Clone {
    pub fn new(width: u32, height: u32, tile_width: u32, tile_height: u32, tile: T) -> Layer<T> {
        Layer {
            tiles: repeat(tile).take((width * height) as usize).collect(),
            width: width,
            height: height,
            tile_width: tile_width,
            tile_height: tile_height
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&T> {
        let offset = (x + y * self.width as i32) as usize;

        if offset < self.tiles.len() {
            Some(&self.tiles[offset])
        } else {
            None
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: T) {
        let offset = (x + y * self.width as i32) as usize;

        self.tiles[offset] = tile;
    }

    pub fn find_intersecting(&self, rect: &Rect) -> Option<Rect> {
        if rect.x() + rect.width() as i32 <= 0 {
            return None;
        }
        if rect.y() + rect.height() as i32 <= 0 {
            return None;
        }

        let x1 = rect.x() / self.tile_width as i32;
        let y1 = rect.y() / self.tile_height as i32;
        let x2 = (rect.x() + rect.width() as i32) / self.tile_width as i32;
        let y2 = (rect.y() + rect.height() as i32) / self.tile_height as i32;

        if x1 < 0 || x2 >= self.width as i32 {
            None
        }
        else if y1 < 0 || y2 >= self.height as i32 {
            None
        }
        else {
            Some(Rect::new_unwrap(x1, y1, (x2 - x1 + 1) as u32, (y2 - y1 + 1) as u32))
        }
    }

    pub fn for_each_intersecting<F: FnMut(&T, &Rect)>(&self, rect: &Rect, mut f: F) {
        if let Some(intersect) = self.find_intersecting(rect) {
            for y in intersect.y()..intersect.height() as i32 + 1 {
                for x in intersect.x()..intersect.x() as i32 + 1 {
                    let position = Rect::new_unwrap(x * self.tile_width as i32, y * self.tile_height as i32, self.tile_width, self.tile_height);

                    f(self.get_tile(x, y).unwrap(), &position);
                }
            }
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new_unwrap(0, 0, self.width * self.tile_width, self.height * self.tile_height)
    }
}


#[cfg(test)]
mod tests {
    use super::Layer;

    use sdl2::rect::Rect;

    #[test]
    fn layer_find_intersecting() {
        let layer = Layer::new(3, 3, 3, 3, ());

        // out of bounds.
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(-1,  1, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap( 1, -1, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(-1, -1, 1, 1)), None);

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(9, 7, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(7, 9, 1, 1)), None);
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(9, 9, 1, 1)), None);

        // middle of tile.
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(1, 1, 1, 1)), Some(Rect::new_unwrap(0, 0, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(4, 1, 1, 1)), Some(Rect::new_unwrap(1, 0, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(7, 1, 1, 1)), Some(Rect::new_unwrap(2, 0, 1, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(1, 4, 1, 1)), Some(Rect::new_unwrap(0, 1, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(4, 4, 1, 1)), Some(Rect::new_unwrap(1, 1, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(7, 4, 1, 1)), Some(Rect::new_unwrap(2, 1, 1, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(1, 7, 1, 1)), Some(Rect::new_unwrap(0, 2, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(4, 7, 1, 1)), Some(Rect::new_unwrap(1, 2, 1, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(7, 7, 1, 1)), Some(Rect::new_unwrap(2, 2, 1, 1)));

        // interlaps 4 tiles.
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(2, 2, 2, 2)), Some(Rect::new_unwrap(0, 0, 2, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(5, 2, 2, 2)), Some(Rect::new_unwrap(1, 0, 2, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(2, 5, 2, 2)), Some(Rect::new_unwrap(0, 1, 2, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(5, 5, 2, 2)), Some(Rect::new_unwrap(1, 1, 2, 2)));

        // interlaps 2 tiles horizontal.
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(2, 1, 2, 1)), Some(Rect::new_unwrap(0, 0, 2, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(5, 1, 2, 1)), Some(Rect::new_unwrap(1, 0, 2, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(2, 4, 2, 1)), Some(Rect::new_unwrap(0, 1, 2, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(5, 4, 2, 1)), Some(Rect::new_unwrap(1, 1, 2, 1)));

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(2, 7, 2, 1)), Some(Rect::new_unwrap(0, 2, 2, 1)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(5, 7, 2, 1)), Some(Rect::new_unwrap(1, 2, 2, 1)));

        // interlaps 2 tiles vertical.
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(1, 2, 1, 2)), Some(Rect::new_unwrap(0, 0, 1, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(1, 5, 1, 2)), Some(Rect::new_unwrap(0, 1, 1, 2)));

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(4, 2, 1, 2)), Some(Rect::new_unwrap(1, 0, 1, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(4, 5, 1, 2)), Some(Rect::new_unwrap(1, 1, 1, 2)));

        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(7, 2, 1, 2)), Some(Rect::new_unwrap(2, 0, 1, 2)));
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(7, 5, 1, 2)), Some(Rect::new_unwrap(2, 1, 1, 2)));

        // exactly one tile.
        assert_eq!(layer.find_intersecting(&Rect::new_unwrap(0, 0, 3, 3)), Some(Rect::new_unwrap(0, 0, 1, 1)));
    }
}
