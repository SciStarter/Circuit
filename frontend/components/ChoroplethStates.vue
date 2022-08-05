<template>
<div ref="display"></div>
</template>

<script>
import 'mapbox-gl/dist/mapbox-gl.css'
import mapboxgl from 'mapbox-gl'
import centroid from '@turf/centroid'

import STATES from "~/assets/geo/simplified-territories-usa.json"

export default {
    name: "ChoroplethStates",
    
    props: {
        value: {
            type: Object,
            required: true,
            default: () => ({}),
        },

        attr: {
            type: String,
            required: false,
            default: "",
        },
    },
    
    data() {
        return {
            map: null,
        };
    },
    
    computed: {
        states() {
            const attr = this.attr;
            let states = structuredClone(STATES);
            
            states.features = states.features.map(f => {
                let val = this.value[f.properties.name];

                if(val !== undefined) {
                    val = val[attr] || val;
                }
                else {
                    val = 0;
                }

                f.properties.participants = val;

                return f;
            });
            
            return states;
        },
        
        max_participants() {
            let max = 2;
            
            for(let f of this.states.features) {
                if(f.properties.participants > max) {
                    max = f.properties.participants;
                }
            }
            
            return max;
        },
    },
    
    mounted() {
        let map = this.map = new mapboxgl.Map({
            accessToken: this.$config.mapboxToken,
            container: this.$refs.display,
            style: 'mapbox://styles/mapbox/streets-v11',
            center: [-98, 39],
            zoom: 2.5,
        });
        
        map.on('style.load', () => {
            map.addSource('states-data', {type: 'geojson', data: this.states});
            map.addLayer({
                id: 'states-fill',
                type: 'fill',
                source: 'states-data',
                layout: {visibility: 'visible'},
                paint: {
                    'fill-color': {
                        property: 'participants',
                        stops: [
                            [0, '#fff'],
                            [1, '#a9d5ec'],
                            [this.max_participants, '#397ab5'],
                        ],
                    },
                    'fill-opacity': 0.75,
                },
            });
        });
        
        const popup = new mapboxgl.Popup({
            closeButton: false,
            closeOnClick: false
        });
        
        map.on('mousemove', 'states-fill', (e) => {
            if (e.features.length > 0) {
                const coordinates = centroid(e.features[0]).geometry.coordinates;
                const name = e.features[0].properties.name;
                const participants = e.features[0].properties.participants;

                while (Math.abs(e.lngLat.lng - coordinates[0]) > 180) {
                    coordinates[0] += e.lngLat.lng > coordinates[0] ? 360 : -360;
                }

                popup.setLngLat(coordinates).setHTML(`<strong>${name}</strong><br>Participants: ${participants}`).addTo(map);
            }
        });
               
 
        map.on('mouseleave', 'states-fill', () => {
            popup.remove();
        });
    },
}
</script>

<style lang="scss" scoped>
div {
    height: 300px;
}
</style>
