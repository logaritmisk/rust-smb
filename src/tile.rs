use std::iter::repeat;
use std::cmp::{min, max};

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
        let x1 = max(rect.x() / self.tile_width as i32, 0);
        let y1 = max(rect.y() / self.tile_height as i32, 0);
        let x2 = min((rect.x() + rect.width() as i32 - 1) / self.tile_width as i32, self.width as i32 - 1);
        let y2 = min((rect.y() + rect.height() as i32 - 1) / self.tile_height as i32, self.height as i32 - 1);

        if x1 < 0 || x2 >= self.width as i32 {
            None
        }
        else if y1 < 0 || y2 >= self.height as i32 {
            None
        }
        else if x2 - x1 <= 0 || y2 - y1 <= 0 {
            None
        }
        else {
            Some(Rect::new_unwrap(x1, y1, (x2 - x1) as u32, (y2 - y1) as u32))
        }
    }

    pub fn for_each_intersecting<F: FnMut(&T, &Rect)>(&self, rect: &Rect, mut f: F) {
        if let Some(intersect) = self.find_intersecting(rect) {
            for y in intersect.y()..intersect.y() + intersect.height() as i32 + 1 {
                for x in intersect.x()..intersect.x() + intersect.width() as i32 + 1 {
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
