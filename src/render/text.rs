pub type Font = graphics::glyph_cache::rusttype::GlyphCache<'static, (), opengl_graphics::Texture>;

use std::cell::{RefCell, RefMut};

pub struct Text<'a, T>
where
    T: AsRef<str>,
{
    pub text: T,
    font: &'a RefCell<Font>,
    pos: (f64, f64),
}
impl<'a, T> Text<'a, T>
where
    T: AsRef<str>,
{
    pub fn new(text: T, font: &'a RefCell<Font>, pos: (f64, f64)) -> Self {
        Self { text, font, pos }
    }
}
impl<'a, T> super::traits::Render for Text<'a, T>
where
    T: AsRef<str>,
{
    fn draw(&self, gl: &mut opengl_graphics::GlGraphics, c: &piston_window::Context) {
        use graphics::character::CharacterCache;
        use piston_window::Transformed;
        let mut font: RefMut<Font> = self.font.borrow_mut();
        let text = piston_window::text::Text::new_color([1.0, 1.0, 1.0, 1.0], 12);
        let text_width = (*font)
            .width(11, self.text.as_ref())
            .expect("Couldn't get text width used to center text");
        text.draw(
            self.text.as_ref(),
            &mut *font,
            &piston_window::DrawState::default(),
            c.transform.trans(self.pos.0 - text_width / 2.0, self.pos.1),
            gl,
        )
        .expect("Couldn't draw text");
    }
}