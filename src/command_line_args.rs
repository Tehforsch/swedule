use std::path::PathBuf;

use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0")]
pub struct CommandLineArgs {
    pub grid_files: Vec<PathBuf>,
    #[clap(short)]
    pub domain_decomposition: Option<usize>,
    #[clap(short)]
    pub quiet: bool,
}
