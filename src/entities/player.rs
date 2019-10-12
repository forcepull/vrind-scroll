use crate::common::Pos;

pub struct Player {
    pos: Pos,
}

impl Player {
    pub fn new () -> Self {
        Player {
            pos: Pos::new(10.0, 10.0)
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        self.pos.to_tuple()
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.set(x,y);
    }

    pub fn move_player(&mut self, x: f32, y: f32) {
        self.pos.add(x, y);
    }
}

