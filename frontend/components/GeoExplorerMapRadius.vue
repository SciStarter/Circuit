<template>
    <div id="radiusWrap">
        <div ref="display" id="map"></div>

        <div id="radiusRange">
            <h1>Radius</h1>
            <p>Click anywhere on the map to place the radius circle. Use the slider to control the circle's radius in miles.</p>
            <b-field>
                <span>{{ radius }}</span>
                <b-slider v-model="radius" :min="minRange" :max="maxRange"></b-slider>
            </b-field>
            <action-button principal @click="search()">Search Opportunities</action-button>
        </div>

        <div class="legend">
            <div class="total"><span class="points"><span>{{counts.points.total}}</span></span>
                <div>
                    <span>Specified Location</span>
                    <div><b-checkbox v-model="radius_show_points" :native-value="true"> Show</b-checkbox></div>
                </div>
            </div>
            <div class="total ptotal"><span class="polygons"><span>{{counts.polygons.total}}</span></span>
                <div>
                  <span>Regional</span>
                  <div><b-checkbox v-model="radius_show_polygons" :native-value="true"> Show</b-checkbox></div>
                </div>
            </div>
            <div class="total"><span class="anywhere"><span>{{counts.anywhere.total}}</span></span> Anywhere, Anytime</div>
        </div>
    </div>
</template>

<script>
import * as turf from "@turf/turf";
import 'mapbox-gl/dist/mapbox-gl.css'
import mapboxgl from 'mapbox-gl'


export default {
    name: "GeoExplorerMapRadius",

    props: {
        counts: {
            type: Object,
            required: true,
            default: () => ({}),
        },
        value: {
            type: Array,
            required: true,
            default: () => ({}),
        },
        centroid: {
            type: Array,
            required: true,
            default: [-95.7,37.1]
        }
    },

    data(){
        return {
            map: null,
            radius: 25,
            minRange: 1,
            maxRange: 50,
            point: this.centroid,
            radius_show_points: true,
            radius_show_polygons: false,
            projectsGeoJSON: null,
            polygonGeoJSON: null,
        };
    },


    watch: {
        radius(){
            this.updateRadius();
        },

        radius_show_polygons(){
            if (this.radius_show_polygons) {
                this.map.setLayoutProperty('projects-polygon', 'visibility', 'visible');
            } else {
                this.map.setLayoutProperty('projects-polygon', 'visibility', 'none');
            }
        }
    },

    mounted() {
        this.mapInit();
    },

    methods: {
        getProjectDates(startDate, stopDate) {
            var dateArray = new Array();
            var currentDate = startDate;
            while (currentDate <= stopDate) {
                dateArray.push(new Date (currentDate).toLocaleDateString());
                currentDate = currentDate.addDays(1);
            }
            // dateArray.push(stopDate.toLocaleDateString());
            return dateArray;
        },

        geo_obj(v,date){
            date = date || null;
            let obj = {
                "type":"Feature",
                "geometry": (v.location_type == 'at') ? v.location_point : v.location_polygon,
                "properties": {
                    "uid": v.uid,
                    "title": v.title,
                    "slug": v.slug,
                    "org": v.organization_name,
                    "start": v.start_datetimes,
                    "end": v.end_datetimes,
                    "date": date
                }
            }
            console.log(obj);
            return obj;
        },

        makeGeoJSON() {
            let geoJSON = {
                "type": "FeatureCollection",
                "name": "projects",
                "crs": { "type": "name", "properties": { "name": "urn:ogc:def:crs:OGC:1.3:CRS84" } },
                "features": []
            };

            // Remove anywhere anytime projects (location_type == any)
            let locationProjects = this.value.filter(v=>v.location_type != "any");

            // split apart projects into individual records by day or week
            locationProjects.map(v => {
                if((v.location_type == 'at' && (v.location_point == null || !v.location_point.coordinates || v.location_point.coordinates.length == 0)) || (v.location_type == 'near' && (!v.location_polygon || !v.location_polygon.coordinates || v.location_polygon.coordinates.length == 0))) {
                    return;
                }

                let obj;

                // must have at least one start date
                if (v.start_datetimes !== null && v.start_datetimes.length > 0){

                    // if only one start time
                    if (v.start_datetimes.length == 1) {

                        // if there are end days, have to test if go into more than one day
                        if (v.end_datetimes && v.end_datetimes.length == 1) {

                            // test if start and end are different days
                            if (new Date(v.start_datetimes[0]).toLocaleDateString() !== new Date(v.end_datetimes[0]).toLocaleDateString()) {

                                // build a new obj for each day
                                let dates = this.getProjectDates(new Date(v.start_datetimes[0]),new Date(v.end_datetimes[0]).addDays(1));
                                for (let i = 0; i < dates.length; i++){
                                    geoJSON.features.push(this.geo_obj(v,dates[i]));
                                }
                                return;

                            } else {
                                // else only need one object
                                geoJSON.features.push(this.geo_obj(v,new Date(v.start_datetimes[0]).toLocaleDateString()));

                            }
                        } else {
                            // only need one object
                            geoJSON.features.push(this.geo_obj(v,new Date(v.start_datetimes[0]).toLocaleDateString()));
                            return;
                        }

                    }

                }

            });

            this.projectsGeoJSON = geoJSON;

        },

        makeRadius(){
            let _center = turf.point(this.point);
            let _radius = this.radius;
            let _options = {
                steps: 80,
                units: "miles"
            };

            return turf.circle(_center, _radius, _options);
        },

        mapInit(){
            this.makeGeoJSON();

            let map = this.map = new mapboxgl.Map({
                accessToken: this.$config.mapboxToken,
                container: this.$refs.display,
                style: 'mapbox://styles/mapbox/light-v11',
                center: this.point,
                zoom: 5,
            });

            map.on('load', () => {
                map.addControl(new mapboxgl.NavigationControl(),'bottom-left');

                let geojson = {
                    "type": "FeatureCollection",
                    "features": []
                };

                geojson.features.push(this.makeRadius());

                map.addSource("circleData", {
                    type: "geojson",
                    data: geojson
                });


                map.addLayer({
                    id: "circle-fill",
                    type: "fill",
                    source: "circleData",
                    paint: {
                        "fill-color": "#333",
                        "fill-opacity": 0.4,
                    },
                });

                let ctx = this;
                map.on('click', function(e) {
                    let coordinates = e.lngLat;
                    ctx.point = [coordinates.lng,coordinates.lat];
                    ctx.updateRadius();
                    map.easeTo({center: coordinates, duration: 500});

                });

                // set up project points
                map.addSource('projects-data', {
                    type: 'geojson',
                    data: this.projectsGeoJSON,
                });

                map.addLayer(
                    {
                        id: 'projects-polygon',
                        type: 'fill',
                        source: 'projects-data',
                        filter: ['==', ['geometry-type'], 'Polygon'],
                        layout: {
                            'visibility': 'none',
                        },
                        paint: {
                            'fill-outline-color':"#2e618f",
                            'fill-color':"#397ab5",
                            'fill-opacity': 0.005,
                        }
                    }
                );

                map.addLayer(
                    {
                        id: 'projects-point',
                        type: 'circle',
                        source: 'projects-data',
                        filter: ['==', ['geometry-type'], 'Point'],
                        // minzoom: 8,
                        paint: {
                            'circle-color':"#397ab5",
                            'circle-stroke-color': 'white',
                            'circle-stroke-width': 1,
                            // 'circle-opacity': {
                            //     stops: [
                            //     [7, 0],
                            //     [8, 1]
                            //     ]
                            // }
                        }
                    }
                );

                map.on('click', 'projects-point', (event) => {
                    let props = event.features[0].properties;
                    new mapboxgl.Popup()
                        .setLngLat(event.features[0].geometry.coordinates)
                        .setHTML(`<p>${props.org}</p><a href="${props.slug}" target="_blank">${props.title}</a>`)
                        .addTo(map);
                });
            });
        },

        updateRadius() {
            const geojson = {
                "type": "FeatureCollection",
                "features": []
            };

            geojson.features.push(this.makeRadius());

            this.map.getSource('circleData').setData(geojson);
        },

        search() {
            //this.makeGeoJSON();
            //this.map.getSource('projects-data').setData(this.projectsGeoJSON);

            this.$router.push({ name: 'find', query: {
                beginning: new Date().toISOString(),
                sort: 'closest',
                latitude: this.point[1],
                longitude: this.point[0],
                proximity: Math.floor(this.radius * 1609.34),
            }});
        }
    },
 }
</script>

<style lang="scss" scoped>
#radiusWrap,#map {
    height: 500px;
}

#radiusWrap {
    position: relative;
}

#radiusRange{
    display: flex;
    flex-direction: column;
    background-color: #fff;
    width: 250px;
    height: auto;
    position: absolute;
    top: 10px;
    left: 10px;
    background: #fff;
    box-shadow: 0 1px 4px rgba(0,0,0,.3);
    font-size: 0.8rem;
    border-radius: 6px;
    padding:10px;

    h1 {
        font-weight: bold;
        font-size: 1rem;
    }
    p {
        line-height: 1.2;
    }

    :deep(.field.has-addons) {
        display: flex;
        align-items: center;

        span {
            font-weight: bold;
            margin-right: 10px;
        }
    }
}

.legend {
    position: absolute;
    top:10px;
    right:10px;
    display: flex;
    flex-direction: column;
    .total {
        background-color: #fff;
        border: 1px solid $snm-color-border;
        font-weight: bold;
        font-size: 0.85rem;
        padding:5px 8px;
        border-radius: 6px;
        display: flex;
        margin-bottom: 10px;
        min-height: 50px;
        display: flex;
        align-items: center;
        box-shadow: 0 1px 4px rgba(0,0,0,.3);
        > span {
            margin-right: 10px;
            top:-5px;
            position:relative;
            > span {
                position: relative;
                z-index: 2;
                top: 5px;
                font-size: .75rem;
            }
        }
        :deep(.b-checkbox) {
            font-weight: normal;
            font-size: .75rem;
        }

    }
    .ptotal {


        small {
            font-weight: normal;
            font-size: 0.6rem;
            color: #7b7b7b;
            display: block;
        }

    }

    .polygons, .points {
        position: relative;
    }
    .polygons:before {
        content:'';
        width: 30px;
        display: block;
        aspect-ratio: 1;
        clip-path: polygon(79.39% 90.45%,20.61% 90.45%,2.45% 34.55%,50.00% 0.00%,97.55% 34.55%);
        background-color: #dfdfdf;
        position: absolute;
        left:50%;
        margin-left: -15px;
        top:0;
        z-index: 1;
    }
    .points > span {
        color: #fff;
    }
    .points:before {
        content:'';
        width: 30px;
        height: 30px;
        display: block;
       border-radius: 100%;
        background-color:  $snm-color-background-meddark;
        position: absolute;
        left:50%;
        margin-left: -15px;
        top:0;
        z-index: 1;
    }

}


</style>
