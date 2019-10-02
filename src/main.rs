
extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use opengl_graphics::*;
use opengl_graphics::GlyphCache;
use glutin_window::GlutinWindow as Window;

mod renderer;

use renderer::FpsCounter;
use renderer::Renderer;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
            "vrind-scroll",
            [1600, 900]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut glyphs = GlyphCache::new("assets/fonts/big_shoulders/BigShouldersDisplay-Thin.ttf", (), TextureSettings::new()).unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());

    let mut fps_counter = FpsCounter::new();
    while let Some(e) = events.next(&mut window) {
        use graphics::*;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                let transform = c.transform.trans(10.0, 100.0);

                clear([0.0, 0.0, 0.0, 1.0], g);
                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                    .draw("Hello world!", &mut glyphs, &c.draw_state, transform, g).unwrap();

                fps_counter.render(&c, g, &mut glyphs);
            });
        }
    }
}
