<template>
    <div class="snm-wrapper">
      <div class="snm-container">

        <div class="inputs flex flex-justify-sb flex-align-bottom">
           <div class="flex flex-align-bottom">
            <div>
            <label>Time Period Available on SNM</label>
            <b-datepicker
                v-model="dates"
                editable
                icon="calendar-today"
                range
                />
            </div>
            <b-button @click="show_filters=true">
                <filter-icon class="filter iconn" />
                {{filterCount}} {{filterCount == 1 ? 'Filter' : 'Filters'}}
            </b-button>
           </div>
           <div>
                <action-button principal><radius-icon class="radius iconn" /> Use Radius Mode</action-button>
            </div>
        </div>

        <div v-if="mapMode == 'usa'" id="usa_map" class="map">
            
        </div>

        <div v-else-if="mapMode == 'state'" id="state_map" class="map">
        
        </div>

        <div v-else-if="mapMode == 'radius'" id="radius_map" class="map">
        
        </div>

        <div id="counts" class="flex">

            <div class="cdiv">
                <div class="sep cheader">
                    <h4>{{location}}</h4>
                    <h3>Specified Location</h3>
                    <b-tooltip label="Specified Location opportunities hapen at a specific location with an address." multilined>
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
                    <nuxt-link :to="'/find' + queries.points" class="view-button">View List</nuxt-link>
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
                    {{counts.polygon.total}}
                </div>
                <div class="ctable">
                <table>
                    <tbody>
                        <tr v-for="row in counts.polygon.domains">
                            <td class="table-label">{{row['name']}}</td>
                            <td class="table-num">{{row['value']}}</td>
                            <td class="table-bar"><comparison-bar :value="row['value']" :max="counts.max" color="#268699" /></td>
                        </tr>
                    </tbody>
                </table>
                <nuxt-link :to="'/find' + queries.polygons" class="view-button">View List</nuxt-link>
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
                <nuxt-link :to="'/find' + queries.anywhere" class="view-button">View List</nuxt-link>
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
            asdf
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

    data(){
        return {
            dates: null,
            show_filters: false,
            filterCount: 0,
            location: 'USA',
            mapMode: 'usa',
            queries: {
                points: '',
                polygon: '',
                anywhere: '?search=xxx'
            },
            counts: {
                max: 2345,
                points: {
                    total: 8932,
                    domains: [
                        {
                            name: "Live Science",
                            value: 2345
                        },
                        {
                            name: "After School",
                            value: 1245
                        },
                        {
                            name: "Museum & Science",
                            value: 435
                        },
                    ]
                },
                polygon: {
                    total: 4256,
                    domains: [
                        {
                            name: "Live Science",
                            value: 2345
                        },
                        {
                            name: "After School",
                            value: 1245
                        },
                        {
                            name: "Museum & Science",
                            value: 435
                        },
                    ]
                },
                anywhere: {
                    total: 2342,
                    domains: [
                        {
                            name: "Live Science",
                            value: 345
                        },
                        {
                            name: "After School",
                            value: 245
                        },
                        {
                            name: "Museum & Science",
                            value: 35
                        },
                    ]
                },
            }
        }
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
</style>