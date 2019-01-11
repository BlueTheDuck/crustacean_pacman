use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window as pw;
use piston_window::Transformed;
use crate::entity::Entity;

pub struct App<'a> {
    pub board: GlTexture,
    pub entities: Vec<Entity<'a>>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: pw::RenderArgs, gl: &mut opengl_graphics::GlGraphics) {
        let c = gl.draw_begin(args.viewport());
        let img = pw::Image::new();
        img.draw(&self.board, &pw::DrawState::default(), c.transform, gl);

        for e in &mut self.entities {
            let mut src_rect = e.sprite.src_rect;
            let frame = e.sprite.frame;
            src_rect[1] = src_rect[1] + src_rect[3] * frame as f64;
            let pos = e.map.nodes[e.node].pos;
            let transform = c
                .transform
                .trans(pos[0] - src_rect[2] / 2.0, pos[1] - src_rect[3] / 2.0);

            img.src_rect(src_rect).draw(
                e.sprite.sprite_sheet,
                &pw::DrawState::default(),
                transform,
                gl,
            );

            //e.sprite.next_frame();

            e.map.render(gl,c);
        }

        gl.draw_end();
    }
}
