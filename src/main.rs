extern crate csv;
extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use opengl_graphics::GlGraphics;
use opengl_graphics::Texture as GlTexture;
use piston_window as pw;
use piston_window::{ButtonEvent, RenderEvent, UpdateEvent};

mod app;
mod entity;
mod map;
mod sprite;

fn main() {
    let mut window: pw::PistonWindow = pw::WindowSettings::new("Pac-Man", [288, 224])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut gl = GlGraphics::new(pw::OpenGL::V3_2);

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

    //let nodes: Vec<map::Node>; // = vec![map::Node::new([18f64, 55f64], 1)];

    /*
    let mut dots: Vec<map::Node> = vec![];
    let dots_file = BufReader::new(File::open(assets.join("dots.txt")).expect("Couldn't read dots"));
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

    //let pacman_map = map::Map::new(nodes.clone());
    let pacman_map = map::Map::from(&assets.join("nodes.csv"));

    let mut pacman = entity::Entity {
        name: Some("Pacman"),
        sprite: sprite::Sprite::new(&sprite_sheet, [0f64, 0f64, 28f64, 28f64]),
        node: Some(0),
        map: pacman_map,
        direction: entity::Direction::Stop,
        speed: 1.0,
        pos: [17f64,57f64]
    };
    pacman.change_node(0);

    let mut app = app::App {
        board: board,
        entities: vec![pacman],
        player: 0,
        ghosts: [1,2,3,4]
    };

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            app.render(args, &mut gl);
        }
        if let Some(_args) = e.update_args() {
            //println!("{:#?}", args);
        }
        if let Some(args) = e.button_args() {
            app.entities_update(args);
        }
        //println!("{:#?}", e);
        app.update();
    }
}
