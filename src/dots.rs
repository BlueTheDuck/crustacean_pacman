use crate::map::{Map, Pos, Position};
use crate::render::Render;
use crate::sprite::Sprite;
use piston_window as pw;
use piston_window::Context;
use piston_window::Transformed;
use std::path::PathBuf;

pub struct Dot {
    pub score: f64,
    pos: Pos,
}
impl Position for Dot {
    fn get_pos(&self) -> Pos {
        return self.pos;
    }
}
pub struct DotMap<'a> {
    pub dots: Vec<Dot>,
    pub sprite: Option<Sprite<'a>>,
}
impl<'a> Map<Dot> for DotMap<'a> {
    fn get_nodes(&self) -> &Vec<Dot> {
        return &self.dots;
    }
}
impl<'a> Render for DotMap<'a> {
    fn draw(&self, gl: &mut opengl_graphics::GlGraphics, c: &pw::Context) {
        use pw::DrawState;

        let img = pw::Image::new();
        let sprite = match &self.sprite {
            Some(sp) => sp,
            None => panic!("No sprite set for dots"),
        };

        for n in &self.dots {
            let pos = n.get_pos();
            let src_rect = sprite.src_rect;
            let transform = c
                .transform
                .trans(pos[0] - src_rect[2] / 2.0, pos[1] - src_rect[3] / 2.0);

            img.src_rect(src_rect)
                .draw(sprite.sprite_sheet, &DrawState::default(), transform, gl);
        }
    }
}
//#region convert::From
impl<'a> std::convert::From<&PathBuf> for DotMap<'a> {
    fn from(file: &PathBuf) -> Self {
        let mut dots: Vec<Dot> = vec![];
        let mut reader = csv::Reader::from_path(file).expect("Couldn't open csv file");
        for result in reader.records() {
            let record: csv::StringRecord = result.expect("Error: No value to read");
            let x = record
                .get(0)
                .unwrap()
                .parse::<f64>()
                .expect("Couldn't parse record [0]");
            let y = record
                .get(1)
                .unwrap()
                .parse::<f64>()
                .expect("Couldn't parse record [1]");
            let score = record
                .get(2)
                .unwrap()
                .parse::<f64>()
                .expect("Couldn't parse record [1]");
            dots.push(Dot { pos: [x, y], score })
        }

        Self { dots, sprite: None }
    }
}
//#endregion
