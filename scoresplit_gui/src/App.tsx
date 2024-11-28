import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { emit, listen } from "@tauri-apps/api/event";
import { Provider } from "./components/ui/provider";
import { Button } from "./components/ui/button";
import { Slider } from "./components/ui/slider";
import { Box } from "@chakra-ui/react";
import { HStack, VStack } from "@chakra-ui/react";



function App() {
  const [imageBase64, setImageBase64] = useState("");
  const [isStreaming, setIsStreaming] = useState(false);



  async function start_stream() {
    await invoke("get_frame");
    setIsStreaming(true);
  }

  function stop_stream() {
    emit("stop_stream");
    setIsStreaming(false);
  }

  // listen event, then set to state
  listen("update_frame", (event) => {
    const image_string = event.payload;
    const fixed_string = "data:image/jpg;base64, " + image_string;
    setImageBase64(fixed_string);

  });
  return (
    <Provider>
      <Box w={720} h={480} m={4}>
        <HStack>

        </HStack>
        <img src={imageBase64} />
      </Box>


      <Slider />

      <Button onClick={start_stream} disabled={isStreaming} />
      <Button onClick={stop_stream} disabled={!isStreaming} />









    </Provider>


  );
}

export default App;
