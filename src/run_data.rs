pub struct RunData {
    pub time: f64,
}

impl RunData {
    pub fn get_speedup(&self, reference_time: f64) -> f64 {
        reference_time / self.time
    }
}
