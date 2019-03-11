use opengl_graphics::Texture as GlTexture;
use std::time::{Duration, Instant};

pub mod units {
    pub const ONE_FRAME_IN_NANOS: f64 = (1.0 / 60.0) * 1e9;
    pub const DEFAULT_SPRITE_SIZE: f64 = 24.0;
}

#[allow(dead_code)]
pub enum AnimationType {
    SECS(Duration, Instant), //Time to complete on cicle // Last time frame was updated
    FRAMECOUNT(usize),
    NONE,
}

pub struct Sprite<'a> {
    pub sprite_sheet: &'a GlTexture,
    pub frame_count: usize,
    pub frame: usize,
    pub src_rect: [f64; 4],
    pub animation: AnimationType,
}

impl<'a> Sprite<'a> {
    pub fn new(sprite_sheet: &'a GlTexture, src_rect_pos: [f64; 2]) -> Self {
        Sprite {
            sprite_sheet,
            frame_count: 4,
            frame: 0,
            src_rect: [
                src_rect_pos[0],
                src_rect_pos[1],
                units::DEFAULT_SPRITE_SIZE,
                units::DEFAULT_SPRITE_SIZE,
            ],
            animation: AnimationType::NONE,
        }
    }
    pub fn animate(&mut self) {
        match self.animation {
            AnimationType::SECS(frame_duration, last_update) => {
                if last_update.elapsed() >= frame_duration {
                    self.frame += 1;
                    self.animation = AnimationType::SECS(frame_duration, Instant::now());
                    if self.frame == self.frame_count {
                        self.frame = 0;
                    }
                }
            }
            AnimationType::FRAMECOUNT(frames) => {
                self.frame += 1;
                if self.frame == std::cmp::min(self.frame_count, frames) {
                    self.frame = 0;
                }
            }
            _ => {}
        }
    }
}
