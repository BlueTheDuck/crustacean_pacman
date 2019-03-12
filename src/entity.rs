use crate::mov::NodeMap;
use crate::render::Render;
use crate::sprite::{units::DEFAULT_SPRITE_SIZE as SPRITE_SIZE, Sprite};

use piston_window as pw;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Stop,
}
impl Direction {
    fn opposite(&self) -> Self {
        match self {
            &Direction::Stop => Direction::Stop,
            &Direction::Up => Direction::Down,
            &Direction::Right => Direction::Left,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
        }
    }
}
impl std::convert::Into<usize> for Direction {
    fn into(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
            Direction::Stop => 4,
        }
    }
}
impl std::convert::From<usize> for Direction {
    fn from(n: usize) -> Self {
        let n = n % 5;
        match n {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            4 => Direction::Stop,
            _ => Direction::Stop,
        }
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                Direction::Up => "Up",
                Direction::Right => "Right",
                Direction::Down => "Down",
                Direction::Left => "Left",
                Direction::Stop => "Stop",
            }
        )
    }
}

pub struct Entity<'a> {
    pub name: Option<&'a str>,
    pub sprite: Sprite<'a>,
    pub node: Option<usize>,
    pub map: NodeMap,
    pub direction: Direction,
    pub speed: f64,
    pub pos: [f64; 2],
}
impl<'a> Entity<'a> {
    fn get_speed_vector(&self) -> [f64; 2] {
        let speed = self.speed;
        let speed_vec = match self.direction {
            Direction::Up => [0.0, -speed],
            Direction::Right => [speed, 0.0],
            Direction::Down => [0.0, speed],
            Direction::Left => [-speed, 0.0],
            Direction::Stop => [0.0, 0.0],
        };
        speed_vec
    }
    pub fn update_pos(&mut self) {
        let speed_vec = self.get_speed_vector();
        let pos = self.pos;
        self.pos = [pos[0] + speed_vec[0], pos[1] + speed_vec[1]];
    }
    pub fn change_node(&mut self, new_node: usize) {
        println!("Setting node {} for {}", new_node, self.name.unwrap());
        if Some(new_node) == self.node {
            return;
        };
        if new_node >= self.map.nodes.len() {
            panic!(
                "Attempted to set new node as {}, when it doesn't exists on self.map",
                new_node
            );
        }
        self.pos = self.map.nodes[new_node].pos;
        self.update_sprite();
        self.node = Some(new_node);
        if self.direction == Direction::Stop {
            return;
        }
        let dir: usize = self.direction.into();
        if self.map.nodes[new_node].neighs[dir] == false {
            self.direction = Direction::Stop;
        }
    }
    pub fn change_direction(&mut self, new_dir: Direction) -> bool {
        if new_dir == Direction::Stop {
            return true;
        }
        if new_dir == self.direction {
            return true;
        }
        if let Some(node) = self.node {
            if self.map.nodes[node].neighs[new_dir as usize] == false {
                println!("Node {} forbids going in that direction", node);
                return false;
            } else {
                self.direction = new_dir;
                self.update_sprite();
                return true;
            }
        } else {
            println!("No node is set");
            if new_dir == self.direction || new_dir == self.direction.opposite() {
                self.direction = new_dir;
                self.update_sprite();
                return true;
            } else {
                println!("Can only change between this and opposite directions");
                return false;
            }
        }
    }
    fn update_sprite(&mut self) {
        self.sprite.src_rect[0] = (SPRITE_SIZE * (self.direction as usize as f64)) as f64;
    }
}
impl<'a> Render for Entity<'a> {
    fn draw(&self, gl: &mut opengl_graphics::GlGraphics, c: &pw::Context) {
        use pw::DrawState;
        use pw::Transformed;

        let img = pw::Image::new();
        let mut src_rect = self.sprite.src_rect;
        let frame = self.sprite.frame;

        src_rect[1] = src_rect[1] + SPRITE_SIZE * frame as f64;
        let pos = self.pos;
        let scale = /* 0.8 */1.0;
        let displacement = 0.0 /* (1.0 - scale) * SPRITE_SIZE / 2.0 */;
        let transform = c
            .transform
            .trans(pos[0] - SPRITE_SIZE / 2.0, pos[1] - SPRITE_SIZE / 2.0)
            .trans(displacement, displacement)
            .scale(scale, scale);

        img.src_rect(src_rect).draw(
            self.sprite.sprite_sheet,
            &DrawState::default(),
            transform,
            gl,
        );
    }
}
