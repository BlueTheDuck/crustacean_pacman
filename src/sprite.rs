use opengl_graphics::Texture as GlTexture;
use std::time::{Duration, Instant};

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
    pub fn new(sprite_sheet: &'a GlTexture, src_rect: [f64; 4]) -> Self {
        Sprite {
            sprite_sheet,
            frame_count: 4,
            frame: 0,
            src_rect,
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
