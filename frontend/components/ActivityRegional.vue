<template>
<div ref="display"></div>
</template>

<script>
import 'mapbox-gl/dist/mapbox-gl.css'
import mapboxgl from 'mapbox-gl'

import STATES from "~/assets/geo/simplified-territories-usa.json"

export default {
    name: "ActivityRegional",

    props: {
        state: {
            type: String,
            required: true,
            default: "Pennsylvania",
        },

        data: {
            type: Object,
            required: true,
            default: () => ({}),
        },

        attr: {
            type: String,
            required: true,
            default: "Unique Users",
        },
    },

    data() {
        return {
            map: null,
        };
    },

    computed: {
        state_feature() {
            return STATES.features.filter(f => f.properties.name === this.state)[0];
        },

        region_features() {
            return {
                "type": "FeatureCollection",
                "features": Object.entries(this.data.regional.regions).map(([name, data]) => ({
                    "type": "Feature",
                    "geometry": {
                        "type": "Point",
                        "coordinates": data.point,
                    },
                    "properties": {
                        name,
                        ...data
                    },
                })),
            };
        },
    },

    watch: {
        attr(new_val, old_val) {
            this.map.setLayoutProperty(old_val, 'visibility', 'none');
            this.map.setLayoutProperty(new_val, 'visibility', 'visible');
        }
    },

    mounted() {
        const coordinates = this.state_feature.geometry.coordinates[0];

        const bounds = new mapboxgl.LngLatBounds(
            coordinates[0],
            coordinates[0]
        );

        for(const coord of coordinates) {
            bounds.extend(coord);
        }

        let map = this.map = new mapboxgl.Map({
            accessToken: this.$config.mapboxToken,
            container: this.$refs.display,
            style: 'mapbox://styles/mapbox/streets-v11',
            bounds: bounds,
            fitBoundsOptions: {padding: 20},
        });

        map.on('style.load', () => {
            map.addSource('state-feature', {type: 'geojson', data: this.state_feature});
            map.addSource('region-features', {type: 'geojson', data: this.region_features});

            map.addLayer({
                id: 'state-shape',
                type: 'line',
                source: 'state-feature',
                layout: {visibility: 'visible'},
            });

            for(const layer_id of ["Unique Users", "New Users", "Returning Users", "Total Pageviews", "Unique Pageviews", "Avg. Time"]) {
                map.addLayer({
                    id: layer_id,
                    type: "heatmap",
                    source: "region-features",
                    layout: {
                        'visibility': (layer_id === this.attr) ? 'visible' : 'none',
                    },
                    paint: {
                        "heatmap-radius": 30,
                        "heatmap-weight": ["get", layer_id],
                        "heatmap-color": ["interpolate",["linear"],["heatmap-density"],0, "rgba(0, 0, 0, 0)", 0.1,"rgba(65,105,225, 0.5)",1,"rgba(0,255,255, 0.5)"]
                    }
                });
            }
        });
    },
}
</script>

<style lang="scss" scoped>
div {
    height: 300px;
}
</style>
