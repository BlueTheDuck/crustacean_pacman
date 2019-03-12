use crate::controls::Gamepad;
use crate::dots::DotMap;
use crate::entity::Direction;
use crate::entity::Entity;
use crate::map::Map;
use crate::render::{Render, Text};
use opengl_graphics::{GlGraphics, Texture as GlTexture};
use piston_window as pw;

pub struct App<'a> {
    pub board: GlTexture,
    pub entities: Vec<Entity<'a>>,
    pub player: usize,
    pub ghosts: [usize; 4],
    pub controls: Gamepad,
    pub debug: bool,
    pub score: [u32; 3],
    pub texts: Vec<Text<'a, String>>,
    pub dots: DotMap<'a>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: pw::RenderArgs, mut gl: &mut GlGraphics) {
        let c = gl.draw_begin(args.viewport());
        self.draw(&mut gl, &c);
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
            if e.name == Some("Pacman") {
                let nearest = self.dots.get_nearest_node(e.pos);
                if nearest.1 < 8.0 {
                    let node = self.dots.dots.remove(nearest.0);
                    self.score[0] += node.score as u32;
                }
            }
            if e.name == Some("Red") {
                use crate::entity::Direction;
                if e.direction == Direction::Stop {
                    let d: usize = e.direction.into();
                    for o in [1, 2, 3].iter() {
                        let nd = (d + o) % 4;
                        if e.change_direction(crate::entity::Direction::from(nd)) == true {
                            if *o != 0 as usize {
                                println!("Now going {}", crate::entity::Direction::from(nd));
                            }
                            break;
                        }
                    }
                }
            }
        }

        for i in &[0, 2] {
            if self.score[*i] > self.score[1] {
                self.score[1] = self.score[*i];
            }
        }
        for i in 0..3 {
            self.texts[i + 3].text = self.score[i].to_string();
        }

        self.entities_update();
    }
    pub fn entities_update(&mut self) {
        // Actual AI
        use crate::map::{Pos, Position};
        let pacman_node_pos: Pos = {
            let pacman = &self.entities[self.player];
            match pacman.node {
                Some(id) => self.entities[self.player].map.get_nodes()[id].get_pos(),
                None => pacman.pos,
            }
            /* let id = pacman.node.unwrap();
            self.entities[self.player].map.get_nodes()[id].get_pos() */
        };
        let gi = 0;
        let ghost: &mut Entity = &mut self.entities[self.ghosts[gi]];
        let ghost_node_pos = {
            let id = ghost.node.unwrap();
            /* ghost.map.get_nodes()[id].get_pos() */
        };
        /* if ghost_node_pos[0] == pacman_node_pos[0] || ghost_node_pos[1] == pacman_node_pos[1] {
            println!("{}: I SEE PACMAN", ghost.name.unwrap());
        } */

        self.entities[self.player].sprite.animate();
    }
}
impl<'a> Render for App<'a> {
    fn draw(&self, gl: &mut opengl_graphics::GlGraphics, c: &pw::Context) {
        let img = pw::Image::new();
        img.draw(&self.board, &pw::DrawState::default(), c.transform, gl);

        self.dots.draw(gl, &c);

        for e in &self.entities {
            e.draw(gl, &c);
        }

        for text in &self.texts {
            text.draw(gl, &c);
        }
    }
}
