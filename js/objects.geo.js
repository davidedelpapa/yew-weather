var base_features = {
    "type": "FeatureCollection",
    "features": [
        {
            "type": "Feature",
            "geometry": {
                "type": "Polygon",
                "coordinates": [[
                    [14.08937, 42.585112],
                    [14.08959, 42.585014],
                    [14.089478, 42.584895],
                    [14.089285, 42.584998],
                    [14.08937, 42.585112]
                ]]
            },
            "properties": { "name": "Torre di Cerrano" }
        },
        {
            "type": "Feature",
            "geometry": {
                "type": "Polygon",
                "coordinates": [[
                    [14.089907, 42.585606],
                    [14.090406, 42.585101],
                    [14.090148, 42.584915],
                    [14.089644, 42.585349],
                    [14.089907, 42.585606]
                ]]
            },
            "properties": { "name": "Beach di Cerrano" }
        },
        {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [14.089444, 42.585000]
            },
            "properties": {
                "popupContent": "<h3>Torre di Cerrano</h3><a href='https://it.wikipedia.org/wiki/Torre_di_Cerrano'>More</a>",
                "markerIcon": "viewtower"
            }
        },
        {
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [14.090035557746889,42.58525072399882]
            },
            "properties": {
                "popupContent": "<h3>Beach</h3>Hours: 8:00am - 8:00pm",
                "markerIcon": "beach",
                "markerColor": "#3d4575",
                "markerBGColor": "#5066f2"
            }
        }
    ]
};

export default base_features;