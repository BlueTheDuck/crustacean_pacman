use opengl_graphics::Texture as GlTexture;

pub struct Sprite<'a> {
    pub sprite_sheet: &'a GlTexture,
    pub frame_count: usize,
    pub frame: usize,
    pub src_rect: [f64; 4],
}

impl<'a> Sprite<'a> {
    pub fn new(sprite_sheet: &'a GlTexture, src_rect: [f64; 4]) -> Self {
        Sprite {
            sprite_sheet,
            frame_count: 4,
            frame: 0,
            src_rect,
        }
    }
    pub fn next_frame(&mut self) {
        self.frame += 1;
        if self.frame == self.frame_count {
            self.frame = 0;
        }
    }
}
