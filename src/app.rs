use crate::controls::Gamepad;
use crate::entity::Direction;
use crate::entity::Entity;
//use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window as pw;
use piston_window::Transformed;

pub struct App<'a> {
    pub board: GlTexture,
    pub entities: Vec<Entity<'a>>,
    pub player: usize,
    pub ghosts: [usize; 4],
    pub controls: Gamepad,
    pub debug: bool,
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
            let pos = e.pos;
            let transform = c
                .transform
                .trans(pos[0] - src_rect[2] / 2.0, pos[1] - src_rect[3] / 2.0);

            img.src_rect(src_rect).draw(
                e.sprite.sprite_sheet,
                &pw::DrawState::default(),
                transform,
                gl,
            );

            if self.debug {
                e.map.render(gl, c, e.node);
            }
        }

        gl.draw_end();
    }
    pub fn update(&mut self) {
        {
            let dir = self.controls.get_one_direction();
            if dir != Direction::Stop {
                let player: &mut Entity = &mut self.entities[self.player];
                player.change_direction(dir);
            }
        }
        for e in &mut self.entities {
            e.update_pos();
            let (node, distance) = e.map.get_nearest_node(e.pos);
            if distance < 3.0 {
                //println!("Found valid node");
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
