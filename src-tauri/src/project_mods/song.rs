use serde::{Deserialize, Serialize};

use super::metadata::MetaData;

#[derive(Serialize, Deserialize)]
pub struct Song {
  name: String,
  path: String,
  duration: String,
  metadata: Option<super::metadata::MetaData>
}

impl Song {
  pub fn new(name: String, path: String, duration: String) -> Song {
    Song { name, path, duration, metadata: None }
  }
  pub fn set_metadata(&mut self, m: MetaData) {
    self.metadata = Some(m);
  }
}