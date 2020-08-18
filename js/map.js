//import base_features from "./objects.geo";

var lat=process.env.LATITUDE;
var lng=process.env.LONGITUDE;
var mapbox_token = process.env.BASEMAP_KEY;

var position = [lat, lng];
var mapboxurl = 'https://api.mapbox.com/styles/v1/{id}/tiles/{z}/{x}/{y}?access_token={accessToken}';

// Mapbox streetmap
var basemap = L.tileLayer(mapboxurl,
{
  attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
  maxNativeZoom: 18,
  maxZoom: 20,
  id: 'mapbox/streets-v11',
  tileSize: 512,
  zoomOffset: -1,
  accessToken: mapbox_token
});

// Mapbox satellite
var satmap = L.tileLayer(mapboxurl,
{
  attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/">OpenStreetMap</a> contributors, <a href="https://creativecommons.org/licenses/by-sa/2.0/">CC-BY-SA</a>, Imagery © <a href="https://www.mapbox.com/">Mapbox</a>',
  maxNativeZoom: 18,
  maxZoom: 20,
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

function pointToLayer(feature, latlng) {
  if (feature.properties && feature.properties.markerIcon) {
    if (feature.properties.markerColor) {
      var color = feature.properties.markerColor;
    }
    else {
      var color = '#725139';
    }
    if (feature.properties.markerBGColor) {
      var BGcolor = feature.properties.markerBGColor;
    }
    else {
      var BGcolor = '#f2c357';
    }
    if (feature.properties.markerSize) {
      var size = feature.properties.markerSize;
    }
    else {
      var size = 30;
    }
    var mki = L.icon.mapkey({
      icon: feature.properties.markerIcon,
      color: color,
      background: BGcolor,
      size:size
    });
    return L.marker(latlng, {icon:mki});
  }
  return L.marker(latlng);
}

function styleProps(feature) {
  if (feature.properties) {
    if (feature.properties.color) {
      return {color: color};
    }
  }
}

// First overlay
function get_basefeatures_overlay() {
  var base_features = [];
  var rawbasefeatures = sessionStorage.basefeatures;
  if(rawbasefeatures || rawbasefeatures === "") {
    var base_features = JSON.parse(rawbasefeatures);
  }
  return base_features
}
// Create a first_overlay for base_features data
function get_overlay_layer () {
  var geojsonData = get_basefeatures_overlay();
  //console.log(geojsonData)
  return L.geoJSON(geojsonData, {
    onEachFeature: onEachFeature,
    pointToLayer: pointToLayer,
  })
}
var first_overlay = get_overlay_layer();



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
    onEachFeature: onEachFeature,
    pointToLayer: pointToLayer,
  })
}
var infolayer = get_datalayer();

// Declare bounds
var southWest = L.latLng(42.583631, 14.092246),
    northEast = L.latLng(42.586724, 14.087289),
    mybounds = L.latLngBounds(southWest, northEast);

// The map
var mymap = L.map('my_map',
  {
    center: position,
    zoom: 18,
    minZoom:15,
    maxZoom: 20,
    maxBounds: mybounds,
    maxBoundsViscosity: 1.0,
    layers: [basemap, satmap, first_overlay, infolayer]
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

// Marker
/*
var mki = L.icon.mapkey({icon:"castle",color:'#725139',background:'#f2c357',size:30});
// Append to marker:
L.marker(position,{icon:mki}).addTo(mymap);
*/

// Function to redraw the geoJSON layer, and its control
// connected then to an event
function redrawLayer(e){
  controls.removeLayer(infolayer);
  
  mymap.removeLayer(infolayer);
  mymap.removeLayer(first_overlay);

  infolayer = get_datalayer();
  infolayer.addTo(mymap);

  first_overlay = get_overlay_layer();
  first_overlay.addTo(mymap);

  controls.addOverlay(infolayer, "Info");
}
mymap.on('submit', redrawLayer);

// Popup with position
var popup = L.popup();
function onMapClick(e) {
    var rev_coord = [e.latlng.lng, e.latlng.lat]
    popup
        .setLatLng(e.latlng)
        .setContent("Position " + e.latlng.toString() + "<br/>GeoJSON: [" + rev_coord + "]")
        .openOn(mymap);
}
mymap.on('click', onMapClick);

export default mymap;