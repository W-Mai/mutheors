use crate::composition::measure::Measure;

#[derive(Clone)]
pub struct Track {
    measures: Vec<Measure>,
}

impl Track {
    pub fn new() -> Self {
        Track { measures: vec![] }
    }
    
    pub fn push(&mut self, measure: Measure) {
        self.measures.push(measure);
    }

    pub fn get_measures(&self) -> &[Measure] {
        &self.measures
    }
}
