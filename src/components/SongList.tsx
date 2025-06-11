import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Song from "./Song";
import Player from "./Player";

const SongList = () => {
  const [songList, setSongList] = useState([] as SongType[]);
  const [songPath, setSongPath] = useState("");
  const [songTitle, setSongTitle] = useState("");

  const loadSong = (id: number) => {
    setSongPath(songList[id].path);
    setSongTitle(songList[id].name);
    console.log(songList[id].path);
  }

  useEffect(() => {
    const getSongList = async () => {
      setSongList(await invoke("get_songs"));
    }
    getSongList();
  }, []);

  useEffect(() => {
    if (songList.length === 0) return;
    loadSong(0);
  }, [songList]);

  return (
    songList.length > 0 ? (
      <div>
        <h2>Song List</h2>
        {songList.map((song, index) => (
          <Song name={song.name} key={index} id={index} duration={song.duration} play={loadSong} />
        ))}
        <Player path={songPath} name={songTitle} />
      </div>     
    ) : (
      <div>Song list is empty</div>
  ));  
}

export default SongList;