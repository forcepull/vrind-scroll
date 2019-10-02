use std::time::{Duration, Instant};
use graphics::Graphics;
use graphics::Context;
use graphics::text::Text;
use graphics::character::CharacterCache;

use graphics::Transformed;

use super::Renderer;

pub struct FpsCounter {
    last: Instant,
    last_fps: u32,
    framecounter: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        FpsCounter {
            last: Instant::now(),
            last_fps: 0,
            framecounter: 0,
        }
    }
}

impl<G,C> Renderer<G, C> for FpsCounter
where 
    C: CharacterCache,
    G: Graphics<Texture = <C as CharacterCache>::Texture> {
    fn render(&mut self, c: &Context, g: &mut G, glyphs: &mut C)
    {
        let elapsed = self.last.elapsed();

        if elapsed.as_micros() > 100000
        {
            self.last = Instant::now();
            self.last_fps = self.framecounter * 10;
            self.framecounter = 0;
        }
        
        self.framecounter += 1;
        
        let transform = c.transform.trans(1520.0, 870.0);

        let text = format!("{} FPS", self.last_fps);

        Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(&text, glyphs, &c.draw_state, transform, g).ok();
    }
}