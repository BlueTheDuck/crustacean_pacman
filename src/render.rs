use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;

pub mod traits {
    use piston_window as pw;
    pub trait Render {
        fn draw(&self, gl: &mut opengl_graphics::GlGraphics, c: &pw::Context);
    }
}
