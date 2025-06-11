import { Play } from "lucide-react";
import React from "react";

interface SongProps extends SongDurationType {
  id: number;
  play: (id: number) => void;
}

const Song: React.FC<SongProps> = ({ name, id, duration, play }) => {
  return (
    <div>
      <button className="border-black border rounded-xl" onClick={() => play(id)}><Play className="p-1" /></button>
      <span>{name}</span>
      <span>{duration}</span>
    </div>
  )
}

export default Song