pub type FileExtension = String;

///Allows for much faster equality checks than std::Path
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FastPath {
    path: String,
}

impl FastPath {
    pub fn new(path: &PathBuf) -> Self {
        Self {
            path: path
                .to_string_lossy()
                .trim_end_matches(MAIN_SEPARATOR)
                .to_string(),
        }
    }
}

impl Display for FastPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}

mod args;
mod info;
mod timer;

use std::{
    fmt::Display,
    path::{PathBuf, MAIN_SEPARATOR},
};

pub use args::*;
pub use info::*;
pub use timer::*;
