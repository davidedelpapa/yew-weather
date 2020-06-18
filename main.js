import init, { run_app } from "./pkg/yew_weather.js";
import { __compiled } from "./src/js/version.js";
import {map} from "./src/js/map.js";

// Add compilation time
console.log("WASM program compiled: " + __compiled());

async function main() {
  await init("/pkg/yew_weather_bg.wasm");

  run_app();
}
main();
