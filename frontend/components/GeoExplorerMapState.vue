<template>
    <div id="stateWrap">
        <div ref="display" id="map"></div>
        <div id="sliderbar">
            <h2 class="spad">Science Opportunities in<br /><span>{{ state }}</span></h2>
            <div class="checks spad">
                <b-radio v-model="display"
                    name="name"
                    native-value="day">
                    Show By Day
                </b-radio>
                <b-radio v-model="display"
                    name="name"
                    native-value="week"
                    v-if="sliderDates.length > 13">
                    Show By Week
                </b-radio>
                <b-radio v-model="display"
                    name="name"
                    native-value="month"
                    v-if="sliderDates.length > 45">
                    Show By Month
                </b-radio>
                <b-radio v-model="display"
                    name="name"
                    native-value="all">
                    Show All
                </b-radio>
            </div>
            <div class="spad" v-if="display != 'all'">
                <label v-if="display=='day'"><b>{{ sliderDates[currentSliderIndex] || 0 }}</b></label>
                <label v-else-if="display=='week'"><b>{{ formatWeek(sliderWeeks[currentSliderIndex]) || 0 }}</b></label>
                <label v-else-if="display=='month'"><b>{{ sliderMonths[currentSliderIndex] || 0 }}</b></label>
                <!-- <input ref="slider" type="range" min="0" :max="sliderDates.length - 1" step="1" value="0" @change="slide()" @keyleft="currentSliderIndex--" @keyright="currentSliderIndex++" /> -->
                <div class="flex slider-area">
                    <b-button v-if="!sliderPlaying" size="is-small" @click="playSlider()"><play-icon /></b-button>
                    <b-button v-else size="is-small" @click="clearPlayInterval()"><pause-icon /></b-button>
                    <b-field v-if="display == 'day'">
                        <b-slider type="is-warning" :min="0" :max="sliderDates.length - 1" v-model="currentSliderIndex" rounded :custom-formatter="()=>sliderDates[currentSliderIndex]" @dragstart="clearPlayInterval()"></b-slider>
                    </b-field>
                    <b-field v-if="display == 'week'">
                        <b-slider type="is-warning" :min="0" :max="sliderWeeks.length - 1" v-model="currentSliderIndex" rounded :custom-formatter="()=>formatWeek(sliderWeeks[currentSliderIndex])" @dragstart="clearPlayInterval()"></b-slider>
                    </b-field>
                    <b-field v-else-if="display == 'month'">
                        <b-slider type="is-warning" :min="0" :max="sliderMonths.length - 1" v-model="currentSliderIndex" rounded :custom-formatter="()=>sliderMonths[currentSliderIndex]" @dragstart="clearPlayInterval()"></b-slider>
                    </b-field>
                </div>
                
            </div>
        </div>
    </div>
     
</template>

<script>
Date.prototype.addDays = function(days) {
    let date = new Date(this.valueOf());
    date.setDate(date.getDate() + days);
    return date;
}

Date.prototype.getWeek = function() {
  const onejan = new Date(this.getFullYear(), 0, 1);
  return Math.ceil((((this - onejan) / 86400000) + onejan.getDay() + 1) / 7);
}

import 'mapbox-gl/dist/mapbox-gl.css'
import mapboxgl from 'mapbox-gl'
import centroid from '@turf/centroid'
import PlayIcon from '~/assets/img/play.svg?inline'
import PauseIcon from '~/assets/img/pause.svg?inline'

import STATES from "~/assets/geo/territories-usa.json"
import { COMPARISON_BINARY_OPERATORS } from '@babel/types';

export default {
    name: "GeoExplorerMap",

    components: {
        PlayIcon,
        PauseIcon
    },

    props: {
        value: {
            type: Object,
            required: true,
            default: () => ({}),
        },
        state: {
            type: String,
            required: true,
            default: null
        },
        range: {
            type: Array,
            required: true,
            default: null
        }

    },

    data() {
        return {
            map: null,
            projectsGeoJSON: null,
            sliderDates: [],
            currentSliderIndex: 0,
            display: "day",
            sss: 1,
            sliderPlaying: false,
            playInterval: null,
            months: ["January","February","March","April","May","June","July","August","September","October","November","December"],
            sliderMonths:[],
            sliderWeeks:[]
        };
    },

    computed: {
        statePoly() {
            let states = structuredClone(STATES);
            states.features = states.features.filter(s => {
                return s.properties.name == this.state;
            });
            return states;
        }
    },

    methods: {
        getTotalDates(a,b){
            let date1 = new Date(a);
            let date2 = new Date(b);
            let Difference_In_Time = date2.getTime() - date1.getTime();
            return (Difference_In_Time / (1000 * 3600 * 24)) + 1;
        },
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
        getProjectMonths() {
            let startMonth = new Date(this.range[0]).getMonth();
            let endMonth = new Date(this.range[1]).getMonth();
            let startYear = new Date(this.range[0]).getFullYear();
            let endYear = new Date(this.range[1]).getFullYear();
            let diff = (endYear-startYear)*12+(endMonth-startMonth)+1

            let dates = [];
            let m = startMonth;
            let y = startYear;
            for (let i = 0; i< diff;i++){
                dates.push(`${this.months[m]} ${y}`);
                if (m == 11) { m = 0; y++} else {m++}
            }

            this.sliderMonths = dates;

        },
        getDatesOfWeek(weekNumber, year) {
            // get the date of the first day of the year
            const startDate = new Date(year, 0, 1);

            // get the number of days to add to the first day of the year to get to the start of the desired week
            const daysToAdd = (weekNumber - 1) * 7 - startDate.getDay() + 1;

            // create a new date object with the start date plus the days to add
            const result = new Date(startDate);
            result.setDate(result.getDate() + daysToAdd);

            // create an array of seven date objects for the seven days of the week
            const weekDates = [];
            for (let i = 0; i < 7; i++) {
                weekDates.push(new Date(result.getFullYear(), result.getMonth(), result.getDate() + i));
                if (i==0){i=5}
            }

            return weekDates;
        },
        formatWeek(val){
            let arr = val.split(" ");
            let weekArr = this.getDatesOfWeek(arr[0],arr[1]);
            let dates = [ weekArr[0].toLocaleDateString(), weekArr[1].toLocaleDateString() ];
            return  dates.join("–");
        },
        getProjectWeeks(){
            const startDate = new Date(this.range[0]);
            const endDate = new Date(this.range[1]);
            const result = [];

            let currentDate = new Date(startDate);

            while (currentDate <= endDate) {
                let week = currentDate.getWeek() + " " + currentDate.getFullYear()
                

                // console.log(weekArr);

            // add week number to result array if not already present
            if (!result.includes(week)) {
                result.push(week);
            }

            // move to next week
            currentDate.setDate(currentDate.getDate() + 7);
            }

            this.sliderWeeks = result;
        },
        geo_obj(v,date){
            date = date || null;
            let obj = {
                "type":"Feature",
                "geometry": {
                    "type": (v.location_point) ? "Point" : "Polygon",
                    "coordinates": (v.location_point) ? v.location_point.coordinates : v.location_polygon.coordinates
                },
                "properties": {
                    "uid": v.uid,
                    "title": v.title,
                    "slug": v.slug,
                    "org": v.organization_name,
                    "start": v.start_datetimes,
                    "end": v.end_datetimes,
                    "date": date,
                    "month": this.months[new Date(date).getMonth()] + " " + new Date(date).getFullYear(),
                    "week": new Date(date).getWeek() + " " + new Date(date).getFullYear()
                }
            }
            return obj;
        },
        makeGeoJSON(){
            let geoJSON = {
                "type": "FeatureCollection",
                "name": "projects",
                "crs": { "type": "name", "properties": { "name": "urn:ogc:def:crs:OGC:1.3:CRS84" } },
                "features": []
            };

            // Remove anywhere anytime projects (location_type == any)
            let locationProjects = this.value.matches.filter(v=>v.location_type != "any");
            

            // split apart projects into individual records by day or week
            locationProjects.map(v => {

                // don't push if there is no coordinates to display
                if ( (!v.location_point && !v.location_point.coordinates) && (!v.location_polygon && !v.location_polygon.coordinates) ) {
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
                    
                    // TODO
                    // if there are multiple start times 
                    // is this a case???
                    else if (v.start_datetimes.length > 1) {
                        console.log(v);
                    }
          
                

                }
  
            });

            this.projectsGeoJSON = geoJSON;

        },
        filterDate(){

            if (this.display == "day"){
                let date = this.sliderDates[this.currentSliderIndex];
            // filter map to only the date of slider
                this.map.setFilter('projects-point', ['match', ['get', 'date'], [date], true, false]);
            } else if (this.display == "week"){
                let date = this.sliderWeeks[this.currentSliderIndex];
                // filter map to only the date of slider
                this.map.setFilter('projects-point', ['match', ['get', 'week'], [date], true, false]);
            } else if (this.display == "month"){
                let date = this.sliderMonths[this.currentSliderIndex];
                // filter map to only the date of slider
                this.map.setFilter('projects-point', ['match', ['get', 'month'], [date], true, false]);
            }

            
        },
        playSlider(){
            let ctx = this;
            ctx.sliderPlaying = true;
            ctx.playInterval = setInterval(function(){
                let length;
                if (ctx.display=="day") {
                    length = ctx.sliderDates.length;
                } else if (ctx.display == 'week'){
                    length = ctx.sliderWeeks.length
                } else if (ctx.display == 'month') {
                    length = ctx.sliderMonths.length
                }
                if (ctx.currentSliderIndex == length - 1) {
                    ctx.currentSliderIndex = 0;
                } else {
                    ctx.currentSliderIndex++;
                }
                
            },500);
        },
        clearPlayInterval(){
            this.sliderPlaying = false;
            if (this.playInterval){
                clearInterval(this.playInterval);
            } 
        },
        mapInit(){
            let map = this.map = new mapboxgl.Map({
                accessToken: this.$config.mapboxToken,
                container: this.$refs.display,
                style: 'mapbox://styles/mapbox/light-v11',
                center: centroid(this.statePoly.features[0]).geometry.coordinates,
                zoom: 5,
            });
            map.on('load', () => {

            map.addControl(new mapboxgl.NavigationControl(),'bottom-left');

            map.addSource('states-data', {type: 'geojson', data: this.statePoly});

            map.addLayer({
                id: 'states-stroke',
                type: 'line',
                source: 'states-data',
                layout: {visibility: 'visible'},
                paint: {
                    'line-color': "#397ab5",
                    'line-width':3
                },
            });

            map.addSource('projects-data', {
                type: 'geojson', 
                data: this.projectsGeoJSON,
            });

            // map.addLayer({
            //     id: 'project-heat',
            //     type: 'heatmap',
            //     source: 'projects-data',
            //     maxzoom: 9,
            //     paint: {
            //     // increase weight as diameter breast height increases
            //     // 'heatmap-weight': {
            //     //     property: 'dbh',
            //     //     type: 'exponential',
            //     //     stops: [
            //     //     [1, 0],
            //     //     [62, 1]
            //     //     ]
            //     // },
            //     // increase intensity as zoom level increases
            //     'heatmap-intensity': {
            //         stops: [
            //         [5, 1],
            //         [9, 3]
            //         ]
            //     },
            //     // assign color values be applied to points depending on their density
            //     'heatmap-color': [
            //         'interpolate',
            //         ['linear'],
            //         ['heatmap-density'],
            //         0,
            //         'rgba(236,222,239,0)',
            //         0.2,
            //         'rgb(208,209,230)',
            //         0.4,
            //         'rgb(166,189,219)',
            //         0.6,
            //         'rgb(103,169,207)',
            //         0.8,
            //         'rgb(28,144,153)'
            //     ],
            //     // increase radius as zoom increases
            //     'heatmap-radius': {
            //         stops: [
            //         [5, 15],
            //         [9, 20]
            //         ]
            //     },
            //     // decrease opacity to transition into the circle layer
            //     'heatmap-opacity': {
            //         default: 1,
            //         stops: [
            //         [8, 1],
            //         [9, 0]
            //         ]
            //     }
            //     }
            // });

            // map.addLayer(
            //     {
            //         id: 'projects-point',
            //         type: 'circle',
            //         source: 'projects-data',
            //         minzoom: 8,
            //         paint: {
            //         'circle-color':"#397ab5",
            //         'circle-stroke-color': 'white',
            //         'circle-stroke-width': 1,
            //         'circle-opacity': {
            //             stops: [
            //             [7, 0],
            //             [8, 1]
            //             ]
            //         }
            //         }
            //     }
            // );

            map.addLayer(
                {
                    id: 'projects-point',
                    type: 'circle',
                    source: 'projects-data',
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


            // this.slide();
            this.filterDate();


            // mapbox changes over time tutorial
            // https://docs.mapbox.com/help/tutorials/show-changes-over-time/



            // map.setFilter('projects-point',    
            //      [">=", ['get', 'timestamp'], startDate.getTime()],
            //      ["<=", ['get', 'timestamp'], oneWeekLater.getTime()]);


            });
        },
        mapUpdate(){
            this.makeGeoJSON();
            this.sliderDates = this.getProjectDates(this.range[0],this.range[1]);
            this.currentSliderIndex = 0;
            // this.$refs.slider.value = 0;
            this.getProjectMonths();
            this.getProjectWeeks();
        },
    },

    mounted() {
        this.makeGeoJSON();
        this.sliderDates = this.getProjectDates(this.range[0],this.range[1]);
        this.mapInit();
        this.getProjectMonths();
        this.getProjectWeeks();
    },
    watch: {
        // I think this has to change to value instead of range, but I'm not getting live data
        // but we also need range watched and updated when someeone selects new dates
        range(){
            this.mapUpdate();
        },
        currentSliderIndex(v){
            this.filterDate();
        },
        display(d){
            let ctx = this;
            if (d == "all") {
                if (ctx.playInterval){
                    clearInterval(ctx.playInterval);
                }
                this.map.setFilter('projects-point',null);
            } else {
                this.filterDate();
                this.currentSliderIndex = 0;
            }
            
        }
    }
}
</script>

<style lang="scss" scoped>
#stateWrap,#map {
    height: 500px;
}

#stateWrap {
    position: relative;
}
#sliderbar {
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
    padding-top:50px;

    .spad {
        padding:10px;
        border-top:1px solid #efefef;
    }

    input[type="range"]{
        width:100%;
    }

    h2 {
        font-weight: bold;
        font-size: 0.9rem;
        line-height: 1.1;
        span {
            font-size: 1.1rem;
        }
    }
}

.checks {
    display: flex;
    flex-direction: column;
    > label {
        margin-bottom: 8px;
        &:last-child{
            margin-bottom: 0;
        }
    }
}

.slider-area {
    button {
        margin-right: 10px;

        svg {
            width:10px;
            height: 10px;
        }
    }
    > div {
        flex-grow: 1;
    }
}

:deep(.mapboxgl-popup-content) {
    font-weight: bold;

    a {
        font-size: 18px;
    }
}

</style>
