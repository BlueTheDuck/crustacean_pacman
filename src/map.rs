use piston_window::Context;
use piston_window::Transformed;

#[derive(Copy,Clone)]
pub struct Node {
    pub pos: [f64;2], //Center [X,Y]
    pub score: u64,
    pub neighs: [Option<u64>;4],//up,right,down,left
    pub weight: Option<u64>,
}
impl Node {
    pub fn new(pos:[f64;2],weight: u64) -> Self {
        Node {
            pos,
            score: 1,
            neighs: [None;4],
            weight: Some(weight),
        }
    }
}
impl std::convert::From<Vec<Option<u64>>> for Node {
    fn from(val: Vec<Option<u64>>) -> Self {
        Self {
            pos: [
                val[0].unwrap() as f64,
                val[1].unwrap() as f64
            ],
            score: val[2].unwrap(),
            neighs: [
                val[3],
                val[4],
                val[5],
                val[6],
            ],
            weight: None
        }
    }
}

pub struct Map {
    pub nodes: Vec<Node>
}
impl Map {
    pub fn new(nodes: Vec<Node>) -> Self {
        Map {
            nodes
        }
    }
    pub fn render(&self, gl: &mut opengl_graphics::GlGraphics, c: Context) {
        for n in &self.nodes {
            let trans = c.transform.trans(n.pos[0],n.pos[1]);
            let r = piston_window::ellipse::circle(0.0,0.0,n.weight.unwrap_or(0) as f64);
            piston_window::ellipse([1.0,0.0,0.0,1.0], r, trans, gl);
        }
    }
}