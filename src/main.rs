extern crate quicksilver;

use quicksilver::{
    Result,
    combinators::result,
    geom::Vector,
    graphics::{Color, Font},
    lifecycle::{Asset, State, Window, run, Settings},
    prelude::Future,
};

pub mod common;
pub mod entities;
pub mod terrain;
mod input;
mod renderer;

use entities::Player;

use input::player_controller;

use terrain::{Terrain, terrain_hills};

use renderer::FpsCounter;
use renderer::Renderer;
use renderer::TerrainRenderer;
use renderer::PlayerCamera;

struct GameState {
    fps_counter: FpsCounter,
    player: Player,
    player_camera: PlayerCamera,
    terrain: Terrain,
    terrain_renderer: TerrainRenderer,
    font: Asset<Font>,
}

impl State for GameState {
    fn new() -> Result<GameState> {
        let font = Asset::new(Font::load("fonts/big_shoulders/BigShouldersDisplay-Thin.ttf").and_then(|font| {
            result(Ok(font))
        }));

        let mut terrain = Terrain::new();
        terrain_hills(&mut terrain);

        Ok(GameState {
            fps_counter: FpsCounter::new(),
            player: Player::new(),
            player_camera: PlayerCamera::new(),
            terrain: terrain,
            terrain_renderer: TerrainRenderer::new()?,
            font: font,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        player_controller::update(&mut self.player, window)?;

        self.player_camera.update(window, &self.player)?;

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.terrain_renderer.update_render(window, &self.terrain)?;

        window.clear(Color::BLACK)?;
        self.fps_counter.render(window, &mut self.font)?;
        self.player.render(window, &mut self.font)?;
        self.terrain_renderer.render(window, &mut self.font)?;
        Ok(())
    }
}

fn main() {
    run::<GameState>("vrind-scroll", Vector::new(1600, 900), Settings::default())
}
