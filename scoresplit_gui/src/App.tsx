import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [imageBase64, setImageBase64] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function start_stream() {
    await invoke("get_frame");


  }
  // listen event, then set to state
  listen("update_frame", (event) => {
    const image_string = event.payload;
    const fixed_string = "data:image/jpg;base64, " + image_string;
    setImageBase64(fixed_string);

  });
  return (
    <div>
      <img src={imageBase64} />

      <button onClick={start_stream}>
        start
      </button>
    </div>

  );
}

export default App;
