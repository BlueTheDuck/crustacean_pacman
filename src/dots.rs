use crate::map::{Position,Pos,Map};
use std::path::PathBuf;

pub struct Dot {
    pub score: f64,
    pos: Pos
}
impl Position for Dot {
    fn get_pos(&self) -> Pos {
        return self.pos
    }
}

//#region convert::From
impl std::convert::From<&PathBuf> for Map<Dot> {
    fn from(file: &PathBuf) -> Self {
        let mut nodes: Vec<Dot> = vec![];
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
            nodes.push(Dot {
                pos: [x, y],
                score,
            })
        }
        Self::new(nodes)
    }
}
//#endregion
