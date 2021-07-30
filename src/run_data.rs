use crate::processor::Processor;

pub struct RunData {
    pub time: f64,
    pub num_processors: usize,
    pub time_spent_communicating: f64,
    pub time_spent_waiting: f64,
}

impl RunData {
    pub fn new(processors: &[Processor]) -> Self {
        let time = *processors
                .iter()
                .map(|processor| processor.time)
                .max()
                .unwrap();
        let num_processors = processors.len();
        let time_spent_communicating = processors.iter().map(|processor| processor.time_spent_communicating).sum::<f64>() / num_processors as f64;
        let time_spent_waiting = processors.iter().map(|processor| processor.time_spent_waiting).sum::<f64>() / num_processors as f64;
        RunData {
            time,
            time_spent_communicating,
            time_spent_waiting,
            num_processors,
        }
    }

    pub fn get_speedup(&self, reference: &RunData) -> f64 {
        reference.time / self.time
    }
}
