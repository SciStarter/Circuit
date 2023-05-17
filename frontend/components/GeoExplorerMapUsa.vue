<template>
     <div ref="display"></div>
</template>

<script>
import 'mapbox-gl/dist/mapbox-gl.css'
import mapboxgl from 'mapbox-gl'
import centroid from '@turf/centroid'

import STATES from "~/assets/geo/simplified-territories-usa.json"

export default {
    name: "GeoExplorerMapUsa",

    props: {
        value: {
            type: Object,
            required: true,
            default: () => ({}),
        }
    },

    data() {
        return {
            map: null,
        };
    },

    computed: {
        states() {
            let states = structuredClone(STATES);

            states.features = states.features.map(f => {
                let val = this.value.states[f.properties.name];

                if(val !== undefined) {
                    val = val.point + val.polygon + this.value.anywhere;
                }
                else {
                    val = this.value.anywhere;
                }

                f.properties.projects = val;

                return f;
            });

            return states;
        },

        max_projects() {
            let max = 2;

            for(let f of this.states.features) {
                if(f.properties.projects > max) {
                    max = f.properties.projects;
                }
            }

            return max;
        },

    },

    mounted() {
        let map = this.map = new mapboxgl.Map({
            accessToken: this.$config.mapboxToken,
            container: this.$refs.display,
            style: 'mapbox://styles/mapbox/light-v11',
            center: [-98, 39],
            zoom: 3.5,
        });

        map.addControl(new mapboxgl.NavigationControl(),'bottom-left');

        map.on('style.load', () => {
            map.addSource('states-data', {type: 'geojson', data: this.states});
            

            map.addLayer({
                id: 'states-fill',
                type: 'fill',
                source: 'states-data',
                layout: {visibility: 'visible'},
                paint: {
                    'fill-color': {
                        property: 'projects',
                        stops: [
                            [this.value.anywhere, '#a9d5ec'],
                            [this.max_projects, '#065E6F'],
                        ],
                    },
                    'fill-opacity': 0.75,
                },
            });

            map.addLayer({
                id: 'states-stroke',
                type: 'line',
                source: 'states-data',
                layout: {visibility: 'visible'},
                paint: {
                    'line-color': "#fff",
                    'line-width':1
                },
            });

            map.on('click', 'states-fill', (evt) => {
                this.$emit('state', evt.features[0].properties.name);
            });

            const popup = new mapboxgl.Popup({
                closeButton: false,
                closeOnClick: false
            });

            map.on('mousemove', 'states-fill', (e) => {
                if (e.features.length > 0) {
                    const coordinates = centroid(e.features[0]).geometry.coordinates;
                    const name = e.features[0].properties.name;
                    const projects = e.features[0].properties.projects;

                    while (Math.abs(e.lngLat.lng - coordinates[0]) > 180) {
                        coordinates[0] += e.lngLat.lng > coordinates[0] ? 360 : -360;
                    }

                    popup.setLngLat(coordinates).setHTML(`<strong>${name}</strong><br>Projects: ${projects}`).addTo(map);
                }
            });

            map.on('mouseleave', 'states-fill', () => {
                popup.remove();
            });
        });
    },
}
</script>

<style lang="scss" scoped>
div {
    height: 500px;
}
</style>
