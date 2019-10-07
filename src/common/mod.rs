pub struct Pos {
    x: f32,
    y: f32
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Pos {
            x: x,
            y: y
        }
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}