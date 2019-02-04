use crate::entity::Direction;
use piston_window::keyboard::Key;
use piston_window::ButtonState;

pub struct Gamepad {
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}
impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }
    pub fn update(&mut self, key: Key, state: ButtonState) {
        let s = match state {
            ButtonState::Press => true,
            ButtonState::Release => false,
        };
        match key {
            Key::Up => self.up = s,
            Key::Right => self.right = s,
            Key::Down => self.down = s,
            Key::Left => self.left = s,
            _ => {}
        }
    }
    pub fn get_one_direction(&self) -> Direction {
        if self.up {
            return Direction::Up;
        }
        if self.right {
            return Direction::Right;
        }
        if self.down {
            return Direction::Down;
        }
        if self.left {
            return Direction::Left;
        }
        return Direction::Stop;
    }
}
