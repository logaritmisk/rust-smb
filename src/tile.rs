pub struct Layer<T> {
    tiles: Vec<T>,
    width: i32
}

impl<T> Layer<T> where T: Clone {
    pub fn new(width: i32, height: i32, tile: T) -> Layer<T> {
        Layer {
            tiles: Vec::from_elem((width * height) as uint, tile),
            width: width
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
}
