use crate::map::{Map, Pos, Position};
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub struct Node {
    pub pos: Pos, //Center [X,Y]
    pub score: u64,
    pub neighs: [bool; 4], //up,right,down,left
    pub weight: Option<u64>,
}
impl Position for Node {
    fn get_pos(&self) -> Pos {
        return self.pos;
    }
}
impl Node {
    #[allow(dead_code)]
    pub fn new(pos: Pos, weight: u64) -> Self {
        Node {
            pos,
            score: 1,
            neighs: [true; 4],
            weight: Some(weight),
        }
    }
}

#[derive(Clone)]
pub struct NodeMap {
    pub nodes: Vec<Node>,
}
impl Map<Node> for NodeMap {
    fn get_nodes(&self) -> &Vec<Node> {
        return &self.nodes;
    }
}

//#region convert::From
impl std::convert::From<&PathBuf> for NodeMap {
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
        Self { nodes }
    }
}
//#endregion
