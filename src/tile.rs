use sdl2::rect::Rect;


pub struct Layer<T> {
    tiles: Vec<T>,
    width: i32,
    tile_width: i32,
    tile_height: i32
}

impl<T> Layer<T> where T: Clone {
    pub fn new(width: i32, height: i32, tile_width: i32, tile_height: i32, tile: T) -> Layer<T> {
        Layer {
            tiles: Vec::from_elem((width * height) as uint, tile),
            width: width,
            tile_width: tile_width,
            tile_height: tile_height
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &T {
        let offset = (x + y * self.width) as uint;

        self.tiles.index(&offset)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: T) {
        let offset = (x + y * self.width) as uint;

        *self.tiles.index_mut(&offset) = tile;
    }

    pub fn get_intersecting(&self, rect: &Rect) -> Rect {
        let x = rect.x / self.tile_width;
        let w = rect.w / self.tile_width;

        let y = rect.y / self.tile_height;
        let h = rect.h / self.tile_height;

        Rect::new(x, y, w, h)
    }
}
