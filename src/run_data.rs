pub struct RunData {
    pub time: f64,
    pub num_processors: usize,
}

impl RunData {
    pub fn get_speedup(&self, reference: &RunData) -> f64 {
        reference.time / self.time
    }
}
