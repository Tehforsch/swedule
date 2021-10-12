use std::fs;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParamFile {
    #[serde(default = "default_num_directions")]
    pub num_directions: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    pub send_time_offset: f64,
    pub send_time_per_byte: f64,
    pub recv_time_offset: f64,
    pub recv_time_per_byte: f64,
    pub solve_time_offset: f64,
    pub solve_time_per_task: f64,
    #[serde(default = "default_message_size")]
    pub size_per_message: f64,
}

impl ParamFile {
    pub fn read(file: &Path) -> Result<Self> {
        let data =
            fs::read_to_string(file).context(format!("While reading param file at {:?}", file))?;
        serde_yaml::from_str(&data).context("Reading param file contents")
    }
}

fn default_num_directions() -> usize {
    84
}

fn default_batch_size() -> usize {
    usize::MAX
}

fn default_message_size() -> f64 {
    32.0 * 2.0 + 64.0 * 5.0
}
