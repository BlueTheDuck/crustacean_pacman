use crate::sprite::Sprite;
use crate::map::Map;

pub struct Entity<'a> {
    pub sprite: Sprite<'a>,
    pub node: usize,
    pub map: Map,
}
