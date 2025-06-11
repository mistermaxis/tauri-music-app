use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MetaData {
  title: String,
  artist: String,
  album: String
}

impl MetaData {
  pub fn new(title: String, artist: String, album: String) -> Self {
    MetaData { title, artist, album }
  }
}