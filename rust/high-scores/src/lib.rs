#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        // unimplemented!(
        //     "Construct a HighScores struct, given the scores: {:?}",
        //     scores
        // )
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        // unimplemented!("Return all the scores as a slice")
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        // unimplemented!("Return the latest (last) score")
        match self.scores.last() {
            Some(&a) => Some(a),
            None => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        // unimplemented!("Return the highest score")
        match self.scores.iter().max() {
            Some(&a) => Some(a),
            None => None,
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // unimplemented!("Return 3 highest scores")
        let mut top = self.scores.to_vec();
        top.sort();
        top.reverse();
        top.truncate(3);
        top
    }
}
