extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window as pw;
use piston_window::*;
use std::fs::File;
use std::io::{BufRead,BufReader};

mod app;
mod entity;
mod map;
mod sprite;

fn main() {
    let mut window: pw::PistonWindow = pw::WindowSettings::new("Pac-Man", [288, 224])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(OpenGL::V3_2);

    let assets = find_folder::Search::ParentsThenKids(1, 1)
        .for_folder("assets")
        .expect("Unable to find folder 'assets'");

    let sprite_sheet = GlTexture::from_path(
        &assets.join("sprite_sheet.png"),
        &pw::TextureSettings::new(),
    )
    .expect("Couldn't create sprite sheet");
    let board = GlTexture::from_path(&assets.join("board.png"), &pw::TextureSettings::new())
        .expect("Couldn't create board texture");

    let nodes: Vec<map::Node> = vec![map::Node::new([18f64, 55f64], 1)];
    /*let mut dots: Vec<map::Node> = vec![];
    let dots_file = File::open(assets.join("dots.txt")).expect("Couldn't read dots");
        
    for line in BufReader::new(dots_file).lines() {
        let line:String = line.unwrap();
        if line[0..1] == String::from("#") {
            continue;
        }
        let mut props:Vec<Option<u64>> = vec![];
        let line:Vec<&str> = line.split(",").collect();
        for l in line {
            let n = match l.parse::<u64>() {
                Ok(v) => Some(v),
                Err(_) => None,
            };
            props.push(n);
        }
        nodes.push(map::Node::from(props));
    }*/

    let pacman_map = map::Map::new(nodes.clone());

    let mut pacman = entity::Entity {
        sprite: sprite::Sprite::new(&sprite_sheet, [0f64, 0f64, 28f64, 28f64]),
        node: 0,
        map: pacman_map,
    };

    let mut app = app::App {
        board: board,
        entities: vec![pacman],
    };

    while let Some(e) = window.next() {
        if let Some(mut args) = e.render_args() {
            app.render(args, &mut gl);
        }
        if let Some(mut args) = e.update_args() {}
        //println!("{:#?}", e);
    }
}
