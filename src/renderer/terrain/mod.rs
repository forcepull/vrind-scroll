use quicksilver::{
    Result,
    geom::{Rectangle, Transform},
    graphics::{Font, Color, Surface, Background::Img},
    lifecycle::{Window, Asset}
};

use crate::terrain::Terrain;
use super::Renderer;

pub struct TerrainRenderer {
    cache: Option<Surface>
}

impl TerrainRenderer {
    pub fn new() -> Result<Self> {
        Ok(
            TerrainRenderer {
                cache: None
            }
        )
    }

    pub fn update_render(&mut self, window: &mut Window, terrain: &Terrain) -> Result<()> {
        if let Some(_) = self.cache {
            return Ok(());
        }

        let surface = Surface::new(256 * 16, 256 * 16)?;
        let terrain_rectangle = Rectangle::new_sized((16.0, 32.0));

        let mut transform;
        
        for y in 0..32 {
            for x in 0..32 {
                if terrain.get(x, y) {
                    transform = Transform::translate((16.0 * (x as f32), 900.0 - 16.0 * (y as f32)));

                    surface.render_to(window, |window| {
                        window.draw_ex(&terrain_rectangle, Color::GREEN, transform, 0);

                        Ok(())
                    })?;
                }
            }
        }

        self.cache = Some(surface);

        Ok(())
    }

}

impl Renderer for TerrainRenderer {
    
    fn render(&mut self, window: &mut Window, _: &mut Asset<Font>) -> Result<()> {
        if let Some(ref surface) = self.cache {
            window.draw_ex(&Rectangle::new((0,0), (256 * 16, 256 * 16)), Img(surface.image()), Transform::IDENTITY, 0);
        }

        Ok(())
    }
}