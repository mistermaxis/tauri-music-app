use std::{borrow::Cow, fs::{self, DirEntry}, path::PathBuf};
//use dirs;
use lofty::{file::{AudioFile, TaggedFile, TaggedFileExt}, read_from_path, tag::Accessor};

use crate::project_mods::{format::{DurationFormat, DurationFormatTrait}, metadata::MetaData, song::Song, utils::capitalize_words};

pub fn read_directory(dir: &PathBuf) -> Option<fs::ReadDir> {
  let entries = fs::read_dir(dir);

  match entries {
    Ok(rd) => Some(rd),
    _ => None
  }
}

pub fn load_lofty_files() -> Option<Vec<Song>> {
  if let Some(entries) = read_directory(&PathBuf::from(dirs::audio_dir().unwrap_or(PathBuf::from("/")))) {
    let mut song_files: Vec<Song> = Vec::<Song>::new();
    entries.for_each(|entry| {
      let lofty_file = read_from_path(entry.as_ref().unwrap().path());
      match &lofty_file {
          Ok(tagged_file) => {
            let mut song = set_song_properties(tagged_file, entry.as_ref().expect("File is not valid"));
            set_song_metadata(&mut song, tagged_file);
            song_files.push(song);
          },
          _ => ()
        }
      }
    );
    println!("{}", dirs::audio_dir().unwrap().to_string_lossy());
    return Some(song_files);
  }
  None
}

pub fn set_song_properties(tagged_file: &TaggedFile, entry: &DirEntry) -> Song {
  let name = capitalize_words(entry.file_name().to_string_lossy().to_string().as_ref());
  let path = entry.path().to_str().unwrap().to_string();
  let duration = DurationFormat::new_from_duration(tagged_file.properties().duration()).as_string();

  let song = Song::new(name, path, duration);
  song
}

pub fn set_song_metadata(song: &mut Song, tagged_file: &TaggedFile) {
  let md: MetaData;
  let unknown = "Unknown";
  match tagged_file.primary_tag() {
    Some(tag) => {
      let title = tag.title().unwrap_or(Cow::from("Unknown")).to_string();
      let artist = tag.artist().unwrap_or(Cow::from("Unknown")).to_string();
      let album = tag.album().unwrap_or(Cow::from("Unknown")).to_string();
      md = MetaData::new(title, artist, album);
    }
    None => {
      md = MetaData::new(unknown.to_string(), unknown.to_string(), unknown.to_string());
    }
  }
  song.set_metadata(md);
}