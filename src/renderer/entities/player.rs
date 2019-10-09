use quicksilver::{
    Result,
    geom::{Rectangle, Transform},
    graphics::{Font, Color},
    lifecycle::{Window, Asset}
};

use crate::entities::Player;
use super::Renderer;

impl Renderer for Player {
    fn render(&mut self, window: &mut Window, _: &mut Asset<Font>) -> Result<()> {
        let player_rectangle = Rectangle::new_sized((16.0, 32.0));
        let (pos_x, pos_y) = self.get_pos();

        let transform = Transform::translate((
            pos_x,
            -pos_y,
        ));
        
        window.draw_ex(&player_rectangle, Color::WHITE, transform, 1);

        Ok(())
    }
}