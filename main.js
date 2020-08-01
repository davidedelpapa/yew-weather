import init, { run_app } from "./pkg/yew_weather.js";
import "./js/wasm_bridge.js";
import mymap from "./js/map.js";

async function main() {
  await init("/pkg/yew_weather_bg.wasm");
  run_app();
}

// Export the Leaflet map
window.mymap = mymap;

main();

