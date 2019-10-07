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
}

