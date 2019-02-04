extern crate csv;
extern crate find_folder;
extern crate freetype as ft;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use opengl_graphics::{GlGraphics, GlyphCache, Texture as GlTexture};
use piston_window as pw;
use piston_window::{ButtonEvent, RenderEvent, ResizeEvent, UpdateEvent};

type Font = graphics::glyph_cache::rusttype::GlyphCache<'static, (), opengl_graphics::Texture>;

mod app;
mod controls;
mod entity;
mod map;
mod render;
mod sprite;

fn main() {
    let mut window: pw::PistonWindow = pw::WindowSettings::new("Pac-Man", [336, 448])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(pw::OpenGL::V3_2);

    //#region Assets
    let assets = find_folder::Search::ParentsThenKids(1, 1)
        .for_folder("assets")
        .expect("Unable to find folder 'assets'");

    let sprite_sheet = GlTexture::from_path(
        &assets.join("sprite_sheet.png"),
        &pw::TextureSettings::new(),
    )
    .expect("Couldn't create sprite sheet");

    let font: Font =
        GlyphCache::new(assets.join("ARCADEPI.TTF"), (), pw::TextureSettings::new()).unwrap();
    let font = std::cell::RefCell::new(font);

    let board = GlTexture::from_path(&assets.join("board.png"), &pw::TextureSettings::new())
        .expect("Couldn't create board texture");

    let texts = {
        let strings = vec!["1UP", "HIGH SCORE", "2UP", "999999", "9999999", "999999"];
        let mut texts = vec![];
        use piston_window::Window;
        let size = window.size();
        for i in 0..6 {
            texts.push(render::Text::new(
                strings[i].to_string(),
                &font,
                (
                    size.width / 6.0 + size.width / 3.0 * (i % 3) as f64,
                    12.0 * ((i / 3 + 1) as f64),
                ),
            ))
        }
        texts
    };
    //#endregion

    let pacman_map = map::Map::from(&assets.join("nodes.csv"));

    let mut pacman = entity::Entity {
        name: Some("Pacman"),
        sprite: sprite::Sprite::new(&sprite_sheet, [0f64, 0f64, 28f64, 28f64]),
        node: None,
        map: pacman_map,
        direction: entity::Direction::Left,
        speed: 1.0,
        pos: [150f64, 330f64],
    };
    pacman.change_node(64);

    let mut app = app::App {
        board: board,
        entities: vec![pacman],
        player: 0,
        ghosts: [1, 2, 3, 4],
        controls: controls::Gamepad::new(),
        debug: true,
        score: [0; 3],
        texts: texts,
    };

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            app.update();
            app.render(args, &mut gl);
        }
        if let Some(_args) = e.update_args() {
            //println!("{:#?}", args);
        }
        if let Some(args) = e.button_args() {
            match args.button {
                piston_window::Button::Keyboard(key) => {
                    app.controls.update(key, args.state);
                }
                _ => {}
            }
            //app.entities_update(args);
        }
        if let Some(args) = e.resize_args() {}
        //println!("{:#?}", e);
    }
}
