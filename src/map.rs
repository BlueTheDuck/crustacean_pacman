use piston_window as pw;
use piston_window::Context;
use piston_window::Transformed;


pub type pos = [f64;2];
pub trait position {
    fn get_pos(&self) -> pos;
}

pub struct Map<T>
where T: position {
    pub nodes: Vec<T>,
}
impl<T> Map<T>
where T: position {
    pub fn new(nodes: Vec<T>) -> Self {
        Map { nodes }
    }
    //Pythagoras with self.nodes[node] and pos
    pub fn calc_distance(&self, node: usize, pos: [f64; 2]) -> f64 {
        let p = self.nodes[node].get_pos();
        let deltas = [(p[0] - pos[0]).abs(), (p[1] - pos[1]).abs()];
        (deltas[0].powf(2.0) + deltas[1].powf(2.0)).sqrt()
    }
    //Pythagoras with self.nodes[0..n] and pos. Returns index and distance
    pub fn get_nearest_node(&self, pos: [f64; 2]) -> (usize, f64) {
        if self.nodes.len() == 0 {
            panic!(
                "Attempted to run Map::get_nearest_node(pos:[f64;2]) without nodes in self.nodes"
            );
        }
        let mut shortest: f64 = std::f64::MAX;
        let mut index: usize = std::u128::MAX as usize; //Hack to get biggest usize
        for i in 0..self.nodes.len() {
            let distance = self.calc_distance(i, pos);
            if distance < shortest {
                shortest = distance;
                index = i;
            }
        }
        (index, shortest)
    }
    //Debugging render. Shows nodes and their directions
    pub fn render(
        &self,
        gl: &mut opengl_graphics::GlGraphics,
        c: Context,
        highlight: Option<usize>,
    ) {
        /* let colors: [[f32; 4]; 2] = [[1.0, 0.0, 0.0, 1.0], [1.0, 0.0, 1.0, 1.0]];
        for i in 0..self.nodes.len() {
            let n = self.nodes[i];
            let color = colors[match highlight {
                Some(h) => {
                    if h == i {
                        1
                    } else {
                        0
                    }
                }
                None => 0,
            }];
            let p = n.get_pos();
            let trans = c.transform.trans(n.pos[0], n.pos[1]);
            let r = piston_window::ellipse::circle(0.0, 0.0, n.weight.unwrap_or(4) as f64);
            piston_window::ellipse(color, r, trans, gl);
            let offsets = [[0.0, -8f64], [8f64, 0.0], [0.0, 8f64], [-8f64, 0.0]];
            for di in 0..4 {
                let d = n.neighs[di];
                if d {
                    pw::line(
                        color,
                        2.0,
                        [0.0, 0.0, offsets[di][0], offsets[di][1]],
                        trans,
                        gl,
                    );
                }
            }
        } */
    }
}
