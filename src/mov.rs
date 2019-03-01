use crate::map::{position,pos,Map};
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub struct Node {
    pub pos: pos, //Center [X,Y]
    pub score: u64,
    pub neighs: [bool; 4], //up,right,down,left
    pub weight: Option<u64>,
}
impl position for Node {
    fn get_pos(&self) -> pos {
        return self.pos
    }
}
impl Node {
    pub fn new(pos: pos, weight: u64) -> Self {
        Node {
            pos,
            score: 1,
            neighs: [true; 4],
            weight: Some(weight),
        }
    }
}
//#region convert::From
impl std::convert::From<&PathBuf> for Map<Node> {
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
