use graphics::Graphics;
use graphics::Context;
use graphics::character::CharacterCache;

pub trait Renderer<G, C> 
where 
    C: CharacterCache,
    G: Graphics<Texture = <C as CharacterCache>::Texture> {
    fn render(&mut self, c: &Context, g: &mut G, c: &mut C);
}