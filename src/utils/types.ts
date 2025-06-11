interface SongTitleType {
  name: string;
}

// This type is used for song objects that only have a title and duration
interface SongDurationType extends SongTitleType {
  duration: string;
}

// This type is used for song objects that only have a title and path
interface SongPathType extends SongTitleType {
  path: string;
}

// This type is used for song objects that have a title, duration, and path
interface SongType extends SongDurationType, SongPathType {
  metadata: {
    title: string,
    album: string,
    artist: string
  }
}

