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
interface ScoreLocation {
  x: number,
  y: number,
  height: number,
  width: number,
}

export const Video: React.FC = () => {

  const [imageBase64, setImageBase64] = useState("");
  const [isStreaming, setIsStreaming] = useState(false);
  const [slidervalue, setSliderValue] = useState([0]);
  // scoreLocation (x, y, width, height)
  const [scoreLocation, setScoreLocation] = useState<ScoreLocation>({ x: 0, y: 0, width: 100, height: 100 });



  async function start_stream() {
    await invoke("get_frame");
    setIsStreaming(true);
  }

  async function open_video() {
    // open dialog
    const file = await open({
      multiple: false,
      directory: false,
    });
    // emit open_video event for close current video
    emit("open_video");
    await invoke("open_video", { videoPath: file });
  }

  // call back for slider value was changed
  function seek_video(event: SliderValueChangeDetails) {
    // emit current slider index (0 ~ 100000)
    emit("video_seek", event.value.toString());
    // set slider value
    setSliderValue(event.value);

  }

  function seekbar_wheel(event: WheelEvent<HTMLDivElement>) {
    // if event.deltaY is plus, wheel is downing
    console.log(event.deltaY);

    const current_slider_value = slidervalue[0];
    console.log(current_slider_value)
    if (event.deltaY > 0) {
      setSliderValue([current_slider_value - 10]);


    }
    else {

      setSliderValue([current_slider_value + 10]);
    }

    emit("video_seek", current_slider_value);


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

  const [image] = useImage(imageBase64);

  const trRef = useRef();
  const rectRef = useRef();

  useEffect(() => {
    trRef.current.nodes([rectRef.current]);
    trRef.current.getLayer().batchDraw();
  })

  return (

    <Provider>

      <HStack>

      </HStack>

      <Stage width={1280} height={720}>
        <Layer>

          <Image image={image} />
          <Rect
            x={scoreLocation.x}
            y={scoreLocation.y}
            width={scoreLocation.width}
            height={scoreLocation.height}
            draggable
            stroke={'blue'}
            strokeWidth={5}
            strokeScaleEnabled={false}
            ref={rectRef}
            onDragEnd={(e) => {
              console.log(e.target.x());
            }}
            onTransformEnd={(e) => {
              console.log(e.target.x(), e.target.y(), e.target.scaleX(), e.target.scaleY());
            }}

          />
          <Transformer ref={trRef} />
        </Layer>
      </Stage>

      <Slider onValueChange={seek_video} max={100000} m={4} onWheel={seekbar_wheel} value={slidervalue} />




      <Button onClick={start_stream} disabled={isStreaming} />
      <Button onClick={stop_stream} disabled={!isStreaming} />
      <Button onClick={open_video} />









    </Provider>


  );


}

export default Video;

