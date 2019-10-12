extern crate quicksilver;
extern crate nalgebra;
extern crate nphysics2d;

use quicksilver::{
    Result,
    combinators::result,
    geom::Vector,
    graphics::{Color, Font},
    lifecycle::{Asset, State, Window, run, Settings},
    prelude::Future,
};

use nphysics2d::object::DefaultBodyHandle;

pub mod common;
pub mod entities;
pub mod terrain;
mod input;
mod renderer;
mod physics;

use entities::Player;

use input::player_controller;

use terrain::{Terrain, terrain_hills};

use renderer::FpsCounter;
use renderer::Renderer;
use renderer::TerrainRenderer;
use renderer::PlayerCamera;

use physics::PhysicsWorld;
use physics::create_rbody;

struct GameState {
    fps_counter: FpsCounter,
    player: Player,
    player_camera: PlayerCamera,
    player_handle: Option<DefaultBodyHandle>,
    terrain: Terrain,
    terrain_renderer: TerrainRenderer,
    physics_world: PhysicsWorld,
    font: Asset<Font>,
}

impl State for GameState {
    fn new() -> Result<GameState> {
        let font = Asset::new(Font::load("fonts/big_shoulders/BigShouldersDisplay-Thin.ttf").and_then(|font| {
            result(Ok(font))
        }));

        let mut terrain = Terrain::new();
        terrain_hills(&mut terrain);

        let mut physics_world = PhysicsWorld::new();
        let player_handle = physics_world.add(create_rbody());

        Ok(GameState {
            fps_counter: FpsCounter::new(),
            player: Player::new(),
            player_camera: PlayerCamera::new(),
            player_handle: Some(player_handle),
            terrain: terrain,
            terrain_renderer: TerrainRenderer::new()?,
            physics_world: physics_world,
            font: font,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.physics_world.update();

        if let Some(ref player_handle) = self.player_handle {
            let (x, y) = self.physics_world.get_pos(*player_handle);

            self.player.set_pos(x, y);
        }

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
