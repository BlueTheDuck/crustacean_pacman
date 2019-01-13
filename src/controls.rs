use piston_window::keyboard::Key;
use piston_window::ButtonState;
use crate::entity::Direction;

pub struct Gamepad {
    pub Up: bool,
    pub Right: bool,
    pub Down: bool,
    pub Left: bool
}
impl Gamepad {
    pub fn new() -> Self {
        Gamepad {
            Up: false,
            Right: false,
            Down: false,
            Left: false
        }
    }
    pub fn update(&mut self,key: Key,state: ButtonState) {
        let s = match state {
            ButtonState::Press => true,
            ButtonState::Release => false,
        };
        match key {
            Key::Up    => self.Up = s,
            Key::Right => self.Right = s,
            Key::Down  => self.Down = s,
            Key::Left  => self.Left = s,
            _ => {}
        }
    }
    pub fn get_one_direction(&self) -> Direction {
        if self.Up {
            return Direction::Up;
        }
        if self.Right {
            return Direction::Right;
        }
        if self.Down {
            return Direction::Down;
        }
        if self.Left {
            return Direction::Left;
        }
        return Direction::Stop;
    }
}