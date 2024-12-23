import { useEffect, useRef, useState, WheelEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import { Provider } from "./components/ui/provider";
import { Button } from "./components/ui/button";
import { Slider } from "./components/ui/slider";
import { Box, SliderValueChangeDetails } from "@chakra-ui/react";
import { HStack, VStack } from "@chakra-ui/react";
import { open } from "@tauri-apps/plugin-dialog";
import { Stage, Layer, Rect, Text, Image, Transformer } from 'react-konva';
import useImage from "use-image";
import { Video } from "./video";

function App() {

  return (
    <Provider>
      <Video />
    </Provider>
  );
}

export default App;
