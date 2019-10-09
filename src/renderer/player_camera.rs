use quicksilver::{
    Result,
    geom::{Rectangle, Transform},
    graphics::View,
    lifecycle::Window,
};

use crate::entities::Player;

pub struct PlayerCamera {
    view: Rectangle
}

impl PlayerCamera {
    
    pub fn new() -> PlayerCamera {
        PlayerCamera {
            view: Rectangle::new_sized((1600, 900))
        }
    }

    pub fn update(&mut self, window: &mut Window, player: &Player) -> Result<()> {
        let (player_x, player_y) = player.get_pos();

        let camera_x = 800.0 - player_x;
        let camera_y = 450.0 + player_y;

        window.set_view(View::new_transformed(self.view, Transform::translate((camera_x, camera_y))));
        
        Ok(())
    }
}