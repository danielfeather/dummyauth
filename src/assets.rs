use std::{collections::HashMap, ops::Deref, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Manifest(HashMap<PathBuf, Chunk>);

impl Deref for Manifest {
    type Target = HashMap<PathBuf, Chunk>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    pub name: String,
    pub file: PathBuf,
    pub src: PathBuf,
    pub is_entry: bool,
    pub css: Vec<PathBuf>,
}
