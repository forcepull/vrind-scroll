extern crate quicksilver;

use quicksilver::{
    Result,
    combinators::result,
    geom::Vector,
    graphics::{Color, Font},
    lifecycle::{Asset, State, Window, run, Settings},
    prelude::Future,
};

mod renderer;

use renderer::FpsCounter;
use renderer::Renderer;

struct GameState {
    fps_counter: FpsCounter,
    font: Asset<Font>,
}

impl State for GameState {
    fn new() -> Result<GameState> {
        let font = Asset::new(Font::load("fonts/big_shoulders/BigShouldersDisplay-Thin.ttf").and_then(|font| {
            result(Ok(font))
        }));

        Ok(GameState {
            fps_counter: FpsCounter::new(),
            font: font,
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        self.fps_counter.render(window, &mut self.font)?;
        Ok(())
    }
}

fn main() {
    run::<GameState>("vrind-scroll", Vector::new(1600, 900), Settings::default())
}
