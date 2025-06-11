import { readFile } from "@tauri-apps/plugin-fs";
import { useEffect, useRef, useState } from "react";

interface PlayerProps extends SongPathType {
  // path: string; // path is a part of SongWithPathType, so no need to redefine it here
  // title: string; is a part of SongType, so no need to redefine it here
}

const Player: React.FC<PlayerProps> = ({ path: songPath, name: songTitle }) => {
  const [audioFile, setAudio] = useState("");
  const audioRef = useRef(null);

  useEffect(() => {
    const loadSong = async () => {
      if (songPath !== "") {
        // Read the audio file from the given path
        try {
          const audioData = await readFile(songPath);
          const audioUrl = URL.createObjectURL(new Blob([audioData]));
          setAudio(audioUrl);
        } catch (error) {
          console.error("Error loading song:", error);
        }
      }
    };

    loadSong();

    // Cleanup function to revoke the object URL when the component unmounts or songPath changes
    return () => {
      if (audioFile) {
        URL.revokeObjectURL(audioFile);
      }
    };
  }, [songPath]);

  useEffect(() => {
    if (audioRef.current) {
      // Reset the audio element when the songPath changes
      (audioRef.current as HTMLAudioElement).load();
      (audioRef.current as HTMLAudioElement).play().catch(error => {
        console.error("Error playing audio:", error);
      });
    }
  }, [audioFile]);

  return (
    <div>
      <h3>Now playing: {songTitle}</h3>
      {
        (audioFile !== "") && 
        (
          <audio ref={audioRef} src={audioFile} controls>
            Your browser does not support the audio element.
          </audio>
        )
      }
    </div>
  )
}

export default Player;