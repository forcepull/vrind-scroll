use quicksilver::{
    Result,
    geom::{Transform},
    graphics::{Background::Img, Font, FontStyle, Color},
    lifecycle::{Asset, Window}
};

use super::Renderer;

pub struct FpsCounter {}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {}
    }
}

impl Renderer for FpsCounter {
    fn render(&mut self, window: &mut Window, font: &mut Asset<Font>) -> Result<()>
    {
        font.execute(|font| {
            let text = format!("{:.0} FPS", window.current_fps());

            let style = FontStyle::new(32.0, Color::RED);
            let text_image = font.render(&text, &style)?;
            
            let rectangle = text_image.area();
            let screen_size = window.screen_size();
            let transform = Transform::translate((
                screen_size.x - rectangle.width() - 5.0,
                screen_size.y - rectangle.height() - 5.0,
            ));
            
            window.draw_ex(&rectangle, Img(&text_image), transform, 1);

            Ok(())
        })
    }
}