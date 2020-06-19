var basemap = L.tileLayer('https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}',
{
  attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
  maxZoom: 18,
  id: 'mapbox/streets-v11',
  tileSize: 512,
  zoomOffset: -1,
  accessToken: process.env.BASEMAP_KEY
});
var weathermap = L.tileLayer('https://tile.openweathermap.org/map/{layer}/{z}/{x}/{y}.png?appid={key}', 
{
  attribution: 'Weather data &copy; <a href="openweathermap.org">OpenWeatherMap</a>',
  layer: 'temp_new',
  key: process.env.WEATHER_KEY
})
var mymap = L.map('my_map',
{
  center: [41.9028, 12.4964],
  zoom: 6,
  layers: [basemap, weathermap]
});

var baseMaps = {
    "Terrain": basemap
};
var overlayMaps = {
    "Heat map": weathermap
};

L.control.layers(baseMaps, overlayMaps).addTo(mymap);

export var map=mymap;