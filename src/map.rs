pub type Distance = f64;
pub type Pos = [Distance; 2];
pub trait Position {
    fn get_pos(&self) -> Pos;
}
pub trait Map<Child>
where
    Child: Position,
{
    fn get_nodes(&self) -> &Vec<Child>;
    fn calc_distance(&self, node: usize, pos: Pos) -> Distance {
        let p = self.get_nodes()[node].get_pos();
        let deltas = [(p[0] - pos[0]).abs(), (p[1] - pos[1]).abs()];
        (deltas[0].powf(2.0) + deltas[1].powf(2.0)).sqrt()
    }
    fn get_nearest_node(&self, pos: Pos) -> (usize, Distance) {
        let mut shortest: f64 = std::f64::MAX;
        let mut index: usize = std::u128::MAX as usize; //Hack to get biggest usize
        for i in 0..self.get_nodes().len() {
            let distance = self.calc_distance(i, pos);
            if distance < shortest {
                shortest = distance;
                index = i;
            }
        }
        (index, shortest)
    }
}
/* pub struct Map<T>
where T: Position {
    pub nodes: Vec<T>,
}
impl<T> Map<T>
where T: Position {
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
} */
/* impl<T> Render for Map<T>
where T: Position {
    fn draw(&self,gl: &mut opengl_graphics::GlGraphics, c: &pw::Context) {
        for n in &self.nodes {
            let p = n.get_pos();
            let trans = c.transform.trans(p[0], p[1]);
            let r = piston_window::ellipse::circle(0.0, 0.0, 4.0);
            piston_window::ellipse([1.0, 0.0, 0.0, 1.0], r, trans, gl);
        }
    }
} */
