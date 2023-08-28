<template>
<div class="snm-wrapper">
  <b-loading :is-full-page="true" v-model="loading" :can-cancel="false"></b-loading>
      <div class="snm-container">

        <div class="inputs flex flex-justify-sb flex-align-bottom">
          <div class="flex flex-align-bottom">
            <!-- Arbitrary date ranges are never going to work within our resources. -->
            <!-- <div> -->
            <!-- <label>Time Period Available on SNM</label> -->
            <!-- <b-datepicker -->
            <!--     v-model="dates" -->
            <!--     editable -->
            <!--     icon="calendar-today" -->
            <!--     range -->
            <!--     :min-date="new Date('2/1/2022')" -->
            <!--     /> -->
            <!-- </div> -->
            <b-button @click="show_filters=true">
                <filter-icon class="filter iconn" />
                {{filterCount}} {{filterCount == 1 ? 'Filter' : 'Filters'}}
            </b-button>
           </div>
           <div>
                <action-button principal v-if="mapMode=='state'" @click="mapMode='radius'"><radius-icon class="radius iconn" /> Use Radius Mode</action-button>
                <action-button text v-if="mapMode=='radius'" @click="mapMode='state'">Exit Radius Mode</action-button>
            </div>
        </div>

    <div class="map">

        <geo-explorer-map-usa v-if="mapMode == 'usa'" :value="statesData" @state="select_state" />
        <geo-explorer-map-state v-else-if="mapMode == 'state'" @centroid="centroid = $event" :value="projectsData" :state="selected_state" :range="dates" />
        <geo-explorer-map-radius v-else-if="mapMode == 'radius'" :value="projectsData" :counts="counts" :centroid="centroid"/>

        <div v-if="mapMode == 'usa'" class="map-message"><span>Click a state to explore its opportunities</span></div>
        <div v-if="mapMode == 'state'" class="backtousa" @click="returnToUSA()">&laquo; back to USA</div>
        <div v-if="mapMode == 'state'" class="legend">
                <div class="total"><span class="points"><span>{{counts.points.total}}</span></span> Specific Location</div>
                <div class="total ptotal"><span class="polygons"><span>{{counts.polygons.total}}</span></span> <div>Regional<small>Viewable in Radius Mode Only</small></div></div>
                <div class="total"><span class="anywhere"><span>{{counts.anywhere.total}}</span></span> Anywhere, Anytime</div>
            </div>
        <div v-if="mapMode == 'state'" class="map-message"><span>Zoom in to further explore the opportunities</span></div>


    </div>



        <div id="counts" class="flex" v-if="mapMode != 'radius'">

            <div class="cdiv">
                <div class="sep cheader">
                    <h4>{{location}}</h4>
                    <h3>Specified Location</h3>
                    <b-tooltip label="Specified Location opportunities happen at a specific location with an address." multilined>
                        <b-button label="?" />
                    </b-tooltip>
                </div>
                <div class="sep ctotal">
                    {{counts.points.total}}
                </div>
                <div class="ctable">
                    <table>
                        <tbody>
                            <tr v-for="row in counts.points.domains">
                                <td class="table-label">{{row['name']}}</td>
                                <td class="table-num">{{row['value']}}</td>
                                <td class="table-bar"><comparison-bar :value="row['value']" :max="counts.max" color="#268699" /></td>
                            </tr>
                        </tbody>
                    </table>
                    <!-- <nuxt-link :to="'/find' + queries.points" class="view-button">View List</nuxt-link> -->
                </div>
            </div>
            <div class="cdiv">
                <div class="sep cheader">
                    <h4>{{location}}</h4>
                    <h3>Regional</h3>
                    <b-tooltip label="Regional opportunities can be done within a specified geographic boundary." multilined>
                        <b-button label="?" />
                    </b-tooltip>
                </div>
                <div class="sep ctotal">
                    {{counts.polygons.total}}
                </div>
                <div class="ctable">
                <table>
                    <tbody>
                        <tr v-for="row in counts.polygons.domains">
                            <td class="table-label">{{row['name']}}</td>
                            <td class="table-num">{{row['value']}}</td>
                            <td class="table-bar"><comparison-bar :value="row['value']" :max="counts.max" color="#268699" /></td>
                        </tr>
                    </tbody>
                </table>
                <!-- <nuxt-link :to="queries.polygons" class="view-button">View List</nuxt-link> -->
            </div>
            </div>
            <div class="cdiv">
                <div class="sep cheader">
                    <h4>{{location}}</h4>
                    <h3>Anywhere, Anytime</h3>
                    <b-tooltip label="Anywhere, Anytime opportunities are online activities people can do at their leisure" position="is-left" multilined>
                        <b-button label="?" />
                    </b-tooltip>
                </div>
                <div class="sep ctotal">
                    {{counts.anywhere.total}}
                </div>
                <div class="ctable">
                <table>
                    <tbody>
                        <tr v-for="row in counts.anywhere.domains">
                            <td class="table-label">{{row['name']}}</td>
                            <td class="table-num">{{row['value']}}</td>
                            <td class="table-bar"><comparison-bar :value="row['value']" :max="counts.max" color="#268699" /></td>
                        </tr>
                    </tbody>
                </table>
                <!-- <nuxt-link :to="queries.anywhere" class="view-button">View List</nuxt-link> -->
            </div>
            </div>

        </div>

        <b-modal
        v-model="show_filters"
        has-modal-card
        trap-focus
        :destroy-on-hide="false"
        aria-role="dialog"
        aria-label="Filter Opportunities"
        aria-modal
        >
        <div class="card flex-col">
            <div class="filter-header">
                <span><filter-icon /> Filter Opportunities</span>
                <action-button text @click="resetFilters">reset filters</action-button>
            </div>
            <div v-if="partner">
                <h2>Your Opportunities</h2>
                <b-checkbox v-model="filters.show_only_owned"> Show Only My Opportunities</b-checkbox>
            </div>
            <div>
                <div class="filter-head flex">
                    <h2>Public Engagment of Science Domains</h2>
                    <action-button text @click="selectAllDomains">Select All</action-button>
                    <action-button text @click="deselectAllDomains">Deselect All</action-button>
                </div>
                <div v-for="d in domains" class="filter-checks">
                    <b-checkbox v-model="filters.domains" :native-value="d.val"> {{ d.name }}</b-checkbox>
                </div>
            </div>
            <div>
                <fieldset>
                    <label class="h2">Activity Type</label>
                    <b-taginput v-model="filters.selected_descriptors" :disabled="loading" :data="suggested_descriptors" field="1" open-on-focus autocomplete data-context="find-activty-type" @typing="query.descriptor_text = $event.toLowerCase()" />
                </fieldset>
            </div>
            <div>
                <h2>Age</h2>
                <b-checkbox v-model="filters.kids_only" :native-value="true" :disabled="loading">
                    Kids Friendly Only
                </b-checkbox>
                <b-checkbox v-model="filters.adults_only" :native-value="true" :disabled="loading">
                    21+ Only
                </b-checkbox>
            </div>
            <div>
                <h2>Cost</h2>
                <b-radio v-model="filters.cost" native-value="any" :disabled="loading">
                    Any Cost
                </b-radio>
                <b-radio v-model="filters.cost" native-value="free" :disabled="loading">
                    Free Only
                </b-radio>
            </div>
            <div>
                <h2>Venue Type</h2>
                <b-select v-model="filters.venue_type" data-context="find-venue-type" :disabled="loading">
                    <option value="either">
                        Any
                    </option>
                    <option value="indoors">
                        Indoors
                    </option>
                    <option value="outdoors">
                        Outdoors
                    </option>
                </b-select>
            </div>

            <div class="filter-actions flex flex-justify-sb">
                <action-button principal @click="applyFilters">Apply Filters</action-button>
                <action-button tertiary @click="resetApplyFilters">Clear Filters</action-button>
            </div>
        </div>
      </b-modal>

      </div>
    </div>


</template>

<script>
import FilterIcon from '~/assets/img/filter.svg?inline'
import RadiusIcon from '~/assets/img/radius.svg?inline'
export default {
    name: "GeoExplorer",

    components: {
        FilterIcon,
        RadiusIcon
    },

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        let states = await context.$axios.$get("/api/ui/organization/opps-regional-overview");
        return {
            statesData: states.data,
            statesCounts: states.counts,
            counts: states.counts,
        }
    },

    data(){
        return {
            centroid: [-95.7,37.1],
            loading: false,
            suggested_descriptors: [],
            projectsData: [],
            partner: true,
            dates: [new Date()],
            show_filters: false,
            filterCount: 0,
            location: 'USA',
            mapMode: 'usa', /// usa, state, radius
            selected_state: null,
            radius_show_points: true,
            radius_show_polygons: false,
            queries: {
                points: '',
                polygon: '',
                anywhere: '?search=xxx'
            },
            filters: {
                show_only_owned: false,
                domains: ['citsci','livesci','maker','museum','school','policy','scicomm','formaled'],
                selected_descriptors: [],
                kids_only: false,
                adults_only: false,
                cost: 'any',
                venue_type: 'either'
            },
            domains: [
                {name:'Citizen Science',val:'citsci'},
                {name:'Formal Education',val:'formaled'},
                {name:'Live Science',val:'livesci'},
                {name:'Maker',val:'maker'},
                {name:'Museum & Science Center',val:'museum'},
                {name:'Out of School Time Program',val:'school'},
                {name:'Science Communication',val:'scicomm'},
                {name:'Science Policy',val:'policy'},
            ]
        }
    },

    computed: {

    },

    mounted() {
        let date = new Date();
        date.setDate(date.getDate() + 6);
        this.dates = [new Date(), date];
    },

    methods: {
        selectAllDomains(){
            this.filters.domains = ['citsci','livesci','maker','museum','school','policy','scicomm','formaled'];
        },
        deselectAllDomains(){
            this.filters.domains = [];
        },
        resetFilters(){
            this.filters = {
                show_only_owned: false,
                domains: ['citsci','livesci','maker','museum','school','policy','scicomm','formaled'],
                selected_descriptors: [],
                kids_only: false,
                adults_only: false,
                cost: 'any',
                venue_type: 'either'
            }
        },
        applyFilters(){
            this.filterCount = this.countFilters();
            this.show_filters = false;
        },
        countFilters(){
            let count = 0;
            let ctx = this.filters;
            if(ctx.show_only_owned){count++}
            if(ctx.domains.length!=8){count++}
            if(ctx.selected_descriptors>0){count++}
            if(ctx.kids_only){count++}
            if(ctx.adults_only){count++}
            if(ctx.cost!='any'){count++}
            if(ctx.venue_type!='either'){count++}
            return count;
        },
        resetApplyFilters(){
            this.resetFilters();
            this.filterCount = this.countFilters();
            this.show_filters = false;
        },
        select_state(state) {
            console.log("select", state);
            this.loading = true;
            this.$axios.$get("/api/ui/organization/opps-regional-detailed", {params: {name: state}}).then(info => {
                this.location = state;
                this.selected_state = state;
                this.mapMode = 'state';
                this.projectsData = info.data;
                this.counts = info.counts;
                this.loading = false;
            });
        },
        returnToUSA() {
            this.location = 'USA';
            this.selected_state = null;
            this.mapMode = 'usa';
            this.projectsData = [];
            this.counts = this.statesCounts;
        },
    }
}
</script>

<style scoped lang="scss">
.not-authenticated .snm-container {
    padding-top:2rem;
}
.flex-align-bottom {
    align-items: flex-end;
}

.inputs {
    > div:first-child > div:first-child {
        margin-right: 1rem;
    }
    margin-bottom: 1rem;
    :deep(.action-button){
        margin-bottom: 0;
    }
    :deep(.button){
        margin-right: 1rem;
    }
    label {
        font-size: .85rem;
        font-weight: bold;
    }
}

.iconn {
    width: 20px;
    height: 20px;
    position: relative;
    top: 4px;
}
.iconn.radius {
    top:0;
}
.filter path {
    fill: #444;
}
button.action-button svg.radius circle{
    fill:#fff;
}

.map {
    height: 500px;
    width: 100%;
    border:1px solid $snm-color-border;
    position: relative;
    margin-bottom: 1rem;
}

.map-message {
    display:flex;
    align-items: center;
    justify-content: center;
    position:absolute;
    top: 10px;
    width:100%;
    left:0;

    span {
        background: #fff;
        box-shadow: 0 1px 4px rgba(0,0,0,.3);
        padding:2px 6px;
        font-size: 0.8rem;
        border-radius: 6px;
    }

}

.backtousa {
    background-color: #fff;
    border: 1px solid $snm-color-border;
    font-weight: bold;
    font-size: 0.85rem;
    padding:5px 8px;
    border-radius: 6px;
    position: absolute;
    top:20px;
    left:20px;
    z-index: 95;
    cursor:pointer;
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


.cdiv {
    flex-basis: 0;
    flex-grow: 1;
    margin-right: 1rem;
    border: 1px solid $snm-color-border;
    font-weight: bold;

    &:last-child {
        margin-right: 0;
    }

    > div {
        padding: 1rem;
    }
}
.sep {
    border-bottom: 1px solid $snm-color-border;
}
.cheader {
    position: relative;
    h3,h4 {
        font-weight: bold;
    }
    h3 {
        font-size: 1.2rem;
        line-height: 1;
    }
    h4 {
        font-size: .85rem;
    }
    .b-tooltip {
        position:absolute;
        top:1rem;
        right:1rem;
    }
}

.ctotal {
    font-size: 1.8rem;
    color: $snm-color-background-meddark;
    padding: .5rem 1rem!important;
}

.cdiv table {
    width:100%;
    td {
        font-size: .85rem;
        padding-bottom: .35rem;
    }
}
.ctable {
    text-align: center;
}
.table-label {
   max-width: 100px;
   line-height: 1.1;
   color: $snm-color-background-meddark;
   padding-right: 10px;
   text-align: left!important;
}
.table-num {
    padding-right: 10px;
    font-weight: normal;
    text-align: right!important;
}
.table-bar {
    padding-top: 4px;
}

.tooltip-trigger button {
    height: 1rem;
    width: 1rem;
    border-radius: 100%;
    padding: 0.5rem;
    font-size: 14px;
    margin-left: 6px;
    background-color: $snm-color-action;
  }
  .view-button {
    display: inline-block;
    font-size: .85rem;
    background-color: $snm-color-action;
    padding:.4rem 1rem;
    border:1px solid $snm-color-action-border;
    border-radius: 6px;
    margin-top: 1rem;
    color: #4a4a4a;
    &:hover {
        color: #4a4a4a;
        background-color: darken($snm-color-action,10%);
    }
  }

@media(max-width:564px){
    .inputs {
        flex-direction: column;
         align-items: flex-start;
         button {
            margin-left:0!important;
         }
    }
}

@media(max-width:767px){
    #counts {
        flex-direction: column;
        .cdiv {
            margin-bottom: 1rem;
            margin-right: 0;
        }
    }
}

.card {
    overflow: auto;
    max-height: calc(100vh - 20px);
    padding: 1rem;
    h2,.h2 {
        font-weight: bold;
        margin-bottom: 1rem;
    }

    > div {
        border-bottom: 1px solid $snm-color-border;
        padding-bottom: 1rem;
        margin-bottom: 1rem;
        &:last-child {
            border:0;
            padding:0;
            margin:0;
        }
    }
}

.filter-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 2rem;
    > span {
        font-weight: bold;
        display: flex;
        font-size: .8rem;
    }

    svg {
        width:20px;
        height: 20px;
        margin-right: 10px;
    }
}

.b-checkbox, .b-radio {
    margin-right: 1.6rem!important;
}

.filter-head {
    margin-bottom: 1rem;
    h2 {
        margin-bottom: 0;
        margin-right: 1rem;
    }
    button.action-button {
        margin-right: 1rem;
    }
}

:deep(.datepicker.control){
    z-index: 99;
}
</style>
