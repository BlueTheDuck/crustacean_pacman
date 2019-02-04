use crate::controls::Gamepad;
use crate::entity::Direction;
use crate::entity::Entity;
use crate::render::{traits::Render, Text};
use opengl_graphics::{GlGraphics, Texture as GlTexture};
use piston_window as pw;
use piston_window::Transformed;
use std::cell::RefCell;

type Font = graphics::glyph_cache::rusttype::GlyphCache<'static, (), opengl_graphics::Texture>;

pub struct App<'a> {
    pub board: GlTexture,
    pub entities: Vec<Entity<'a>>,
    pub player: usize,
    pub ghosts: [usize; 4],
    pub controls: Gamepad,
    pub debug: bool,
    //pub font: RefCell<Font>,
    pub score: [u32; 3],
    pub texts: Vec<Text<'a, String>>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: pw::RenderArgs, mut gl: &mut GlGraphics) {
        use piston_window as pw;

        let c = gl.draw_begin(args.viewport());
        let img = pw::Image::new();
        img.draw(&self.board, &pw::DrawState::default(), c.transform, gl);

        for e in &mut self.entities {
            e.draw(&mut gl, &c);

            if self.debug {
                e.map.render(gl, c, e.node);
            }
        }

        for text in &self.texts {
            text.draw(&mut gl, &c);
        }

        /*  const cs: f64 = 11.0;
        const w: f64 = 336.0; // Hardcoded width, TODO: Get value from window
        const wc: f64 = w / cs;

        let texts: Vec<String> = vec![
            "1UP".into(),
            "HIGH SCORE".into(),
            "2UP".into(),
            String::from(self.score[0].to_string()),
            String::from(self.score[1].to_string()),
            String::from(self.score[2].to_string()),
        ];
        let text = pw::text::Text::new_color([1.0, 1.0, 1.0, 1.0], 12);
        let y = cs;
        let offset = 4.0;
        for line in 0..2 {
            for item in 0..3 {
                use graphics::character::CharacterCache;
                let li = line * 3 + item;
                let item_width = self
                    .font
                    .width(cs as u32, texts[li].as_ref())
                    .expect("Coudln't get width size of string");
                let x = w / 6.0 + (w / 3.0 * item as f64) - item_width / 2.0;
                text.draw(
                    texts[li].as_ref(),
                    &mut self.font,
                    &pw::DrawState::default(),
                    c.transform.trans(x, y + ((y + offset) * line as f64)),
                    gl,
                )
                .expect("Error displaying GUI string");
            }
        } */

        gl.draw_end();
    }
    pub fn update(&mut self) {
        let dir = self.controls.get_one_direction();
        if dir != Direction::Stop {
            let player: &mut Entity = &mut self.entities[self.player];
            player.change_direction(dir);
        }
        for e in &mut self.entities {
            e.update_pos();
            let (node, distance) = e.map.get_nearest_node(e.pos);
            if distance < 3.0 {
                let old_node = e.node;
                e.change_node(node);

                if old_node != e.node {
                    println!(
                        "Updated node for {}. Now {}",
                        e.name.unwrap(),
                        e.node.unwrap()
                    );
                    println!(
                        "Changed node to {}. Valid directions now are {:#?}",
                        node, e.map.nodes[node].neighs
                    );
                }
            } else {
                e.node = None;
            }
        }

        self.entities[self.player].sprite.next_frame();

        for i in 0..1 {
            if self.score[i * 2] > self.score[1] {
                self.score[1] = self.score[i * 2];
            }
        }

        for i in 0..3 {
            self.texts[i + 3].text = self.score[i].to_string();
        }
    }
    pub fn entities_update(&mut self, args: pw::ButtonArgs) {
        let player: &mut Entity = &mut self.entities[self.player];
        if args.state == pw::ButtonState::Press {
            if let pw::Button::Keyboard(key) = args.button {
                println!("Changing direction to {:#?}", key);
                let could = player.change_direction(match key {
                    pw::keyboard::Key::Up => Direction::Up,
                    pw::keyboard::Key::Right => Direction::Right,
                    pw::keyboard::Key::Down => Direction::Down,
                    pw::keyboard::Key::Left => Direction::Left,
                    _ => Direction::Stop,
                });
                if !could {
                    println!("Couldn't change direction");
                }
            }
        }
    }
}
