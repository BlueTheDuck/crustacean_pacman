use piston_window::Context;
use piston_window::Transformed;
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub struct Node {
    pub pos: [f64; 2], //Center [X,Y]
    pub score: u64,
    pub neighs: [bool; 4], //up,right,down,left
    pub weight: Option<u64>,
}
impl Node {
    pub fn new(pos: [f64; 2], weight: u64) -> Self {
        Node {
            pos,
            score: 1,
            neighs: [true; 4],
            weight: Some(weight),
        }
    }
}
/* TOTALLY BROKEN! DON'T REMOVE COMMENT
impl std::convert::From<Vec<Option<u64>>> for Node {
    fn from(val: Vec<Option<u64>>) -> Self {
        Self {
            pos: [val[0].unwrap() as f64, val[1].unwrap() as f64],
            score: val[2].unwrap(),
            neighs: [val[3]!=Som1.0, val[4]!=1.0, val[5], val[6]],
            weight: None,
        }
    }
}*/

pub struct Map {
    pub nodes: Vec<Node>,
}
impl Map {
    pub fn new(nodes: Vec<Node>) -> Self {
        Map { nodes }
    }
    pub fn calc_distance(&self, node: usize, pos: [f64; 2]) -> f64 {
        let n = self.nodes[node];
        let deltas = [(n.pos[0] - pos[0]).abs(), (n.pos[1] - pos[1]).abs()];
        (deltas[0].powf(2.0) + deltas[1].powf(2.0)).sqrt()
    }
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
    pub fn render(&self, gl: &mut opengl_graphics::GlGraphics, c: Context) {
        for n in &self.nodes {
            let trans = c.transform.trans(n.pos[0], n.pos[1]);
            let r = piston_window::ellipse::circle(0.0, 0.0, n.weight.unwrap_or(0) as f64);
            piston_window::ellipse([1.0, 0.0, 0.0, 1.0], r, trans, gl);
        }
    }
}
//#region convert::From
/* TOTALLY BROKEN! DON'T REMOVE COMMENT
impl std::convert::From<BufFile> for Map {
    fn from(file: BufFile) -> Self {
        use std::io::BufRead;

        let mut nodes: Vec<Node> = vec![];
        for line in file.lines() {
            let line: String = line.unwrap();
            if line[0..1] == String::from("#") {
                continue;
            }
            let mut props: Vec<Option<u64>> = vec![];
            let line: Vec<&str> = line.split(",").collect();
            for l in line {
                let n = match l.parse::<u64>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                };
                props.push(n);
            }
            nodes.push(Node::from(props));
        }
        Self { nodes }
    }
}*/
impl std::convert::From<&PathBuf> for Map {
    fn from(file: &PathBuf) -> Self {
        let mut nodes: Vec<Node> = vec![];

        let mut reader = csv::Reader::from_path(file).expect("Couldn't open csv file");
        for result in reader.records() {
            let record: csv::StringRecord = result.expect("Error?");
            //println!("{:#?}", record);
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
            let mut neighs: [bool; 4] = [false; 4];
            for i in 2..6 {
                neighs[i - 2] = record.get(i).unwrap() == "True";
            }
            nodes.push(Node {
                pos: [x, y],
                neighs: neighs,
                score: 0,
                weight: None,
            })
        }
        Self::new(nodes)
    }
}
//#endregion
