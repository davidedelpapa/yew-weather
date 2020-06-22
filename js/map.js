var lat=process.env.LATITUDE;
var lng=process.env.LONGITUDE;
var mapbox_token = process.env.BASEMAP_KEY;

var position = [lat, lng];
var mapboxurl = 'https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}';

// Mapbox streetmap
var basemap = L.tileLayer(mapboxurl,
{
  attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
  maxZoom: 18,
  id: 'mapbox/streets-v11',
  tileSize: 512,
  zoomOffset: -1,
  accessToken: mapbox_token
});

// Mapbox satellite
var satmap = L.tileLayer(mapboxurl,
{
  attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
  maxZoom: 18,
  id: 'mapbox/satellite-v9',
  tileSize: 512,
  zoomOffset: -1,
  accessToken: mapbox_token
});

// Display popup if popupContent property
// is present in the geoJSON feature
function onEachFeature(feature, layer) {
  if (feature.properties && feature.properties.popupContent) {
      layer.bindPopup(feature.properties.popupContent);
  }
}

// Get geoJSON data from the sessionStorage
function get_data() {
  var geojsonData = [];
  var rawGeojsonData = sessionStorage.geojsonData;
  if(rawGeojsonData || rawGeojsonData === "") {
    var geojsonData = JSON.parse(rawGeojsonData);
  }
  return geojsonData
}

// Create a layer for geoJSON data
function get_datalayer () {
  var geojsonData = get_data();
  return L.geoJSON(geojsonData, {
    onEachFeature: onEachFeature
  })
}
var infolayer = get_datalayer();

// The map
var mymap = L.map('my_map',
  {
    center: position,
    zoom: 18,
    layers: [basemap, satmap, infolayer]
  });

// Basemaps in Layer Control
var baseMaps = {
    "Satellite": satmap,
    "Streets": basemap
};
// Overlay maps in Layer Control
var overlayMap = {
  "Info": infolayer
}; 
// Layer Control
var controls = L.control.layers(baseMaps, overlayMap).addTo(mymap);

// Function to redraw the geoJSON layer, and its control
// connected then to an event
function redrawLayer(e){
  controls.removeLayer(infolayer);
  mymap.removeLayer( infolayer);
  infolayer = get_datalayer();
  infolayer.addTo(mymap);
  controls.addOverlay(infolayer, "Info");
}
mymap.on('submit', redrawLayer);

export default mymap;