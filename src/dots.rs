use crate::map::{position,pos,Map};
use std::path::PathBuf;

struct Dot {
    score: f64,
    pos: pos
}
impl position for Dot {
    fn get_pos(&self) -> pos {
        return self.pos
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
            let score = record
                .get(2)
                .unwrap()
                .parse::<f64>()
                .expect("Couldn't parse record [1]");
            nodes.push(Node {
                pos: [x, y],
                score,
            })
        }
        Self::new(nodes)
    }
}
//#endregion
