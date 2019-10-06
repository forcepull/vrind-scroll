use quicksilver::{
    Result,
    graphics::Font,
    lifecycle::{Asset, Window}
};

pub trait Renderer {
    fn render(&mut self, window: &mut Window, font: &mut Asset<Font>) -> Result<()>;
}