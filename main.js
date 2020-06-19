import init, { run_app } from "./pkg/yew_weather.js";
import "./js/map.js";

async function main() {
  await init("/pkg/yew_weather_bg.wasm");
  run_app();
}
main();
