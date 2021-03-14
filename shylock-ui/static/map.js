
export function show_map(apikey, lat, lon) {
  
  var mymap = L.map('mapid').setView([lat, lon], 16);

  L.tileLayer('https://tile.thunderforest.com/atlas/{z}/{x}/{y}.png?apikey={accessToken}', {
    attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors, Imagery © <a href="https://www.thunderforest.com/">Thunderforest</a>',
    maxZoom: 19,
    id: 'mapbox/streets-v11',
    tileSize: 512,
    zoomOffset: -1,
    accessToken: apikey
  }).addTo(mymap);

  //var tileUrl = 'https://tile.thunderforest.com/atlas/{z}/{x}/{y}.png?apikey=' + apikey,
  //layer = new L.TileLayer(tileUrl, {maxZoom: 19});

  // add the layer to the map
  // mymap.addLayer(layer);
  
  var marker = L.marker([lat, lon]).addTo(mymap);
}