use crate::map::Map;
use crate::sprite::Sprite;

pub struct Entity<'a> {
    pub name: Option<&'a str>,
    pub sprite: Sprite<'a>,
    pub node: usize,
    pub map: Map,
    pub speed: Option<[f64; 2]>,
    pub pos: Option<[f64; 2]>,
}
