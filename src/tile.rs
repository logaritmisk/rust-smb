use std::iter::repeat;
use std::cmp::{min, max};

use sdl2::rect::Rect;


pub struct Layer<T> {
    tiles: Vec<T>,
    width: i32,
    height: i32,
    tile_width: i32,
    tile_height: i32
}

impl<T> Layer<T> where T: Clone {
    pub fn new(width: i32, height: i32, tile_width: i32, tile_height: i32, tile: T) -> Layer<T> {
        Layer {
            tiles: repeat(tile).take((width * height) as uint).collect(),
            width: width,
            height: height,
            tile_width: tile_width,
            tile_height: tile_height
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &T {
        let offset = (x + y * self.width) as uint;

        &self.tiles[offset]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: T) {
        let offset = (x + y * self.width) as uint;

        self.tiles[offset] = tile;
    }

    pub fn get_intersecting(&self, rect: &Rect) -> Rect {
        let x1 = max(rect.x / self.tile_width, 0);
        let y1 = max(rect.y / self.tile_height, 0);
        let x2 = min((rect.x + rect.w - 1) / self.tile_width, self.width - 1);
        let y2 = min((rect.y + rect.h - 1) / self.tile_height, self.height - 1);

        Rect::new(x1, y1, x2 - x1, y2 - y1)
    }

    pub fn for_each_intersecting<F>(&self, rect: &Rect, mut f: F) where F: FnMut(&T, i32, i32) {
        let intersect = self.get_intersecting(rect);

        if intersect.x < 0 || intersect.x + intersect.w > self.width {
            return;
        }
        else if intersect.y < 0 || intersect.y + intersect.h > self.height {
            return;
        }

        for y in range(intersect.y, intersect.y + intersect.h + 1) {
            for x in range(intersect.x, intersect.x + intersect.w + 1) {
                f(self.get_tile(x, y), x * self.tile_width, y * self.tile_height);
            }
        }
    }
}
