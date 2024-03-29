use quicksilver::{
    Result,
    lifecycle::Window,
    input::Key,
};

use crate::entities::Player;

pub fn update(player: &mut Player, window: &mut Window) -> Result<()> {
    if window.keyboard()[Key::Right].is_down() {
        player.move_player(2.5, 0.0);
    }

    if window.keyboard()[Key::Left].is_down() {
        player.move_player(-2.5, 0.0);
    }
    
    if window.keyboard()[Key::Up].is_down() {    
        player.move_player(0.0, 2.5);
    }

    if window.keyboard()[Key::Down].is_down() {
        player.move_player(0.0, -2.5);
    }

    Ok(())
}