<template>
<div class="general-filters" :class="{'widget':widget,'evolveme':evolveme}">
  <template v-if="!widget">
  <div class="snm-container">
  <div class="basic-filter-backdrop">
    <form @submit.prevent="search">
    <div class="gf-fields">
      <b-field label="Search" label-position="inside" data-context="find-keywords">
        <b-input ref="search_keywords" v-model="text_proxy" :name="'new-' + Math.random()" placeholder="e.g. astronomy, festival" icon="magnify" />
      </b-field>
      <lookup-place v-model="place_proxy" @valid="set_valid" label-position="inside" data-context="find-lookup-place" />
      <div class="centered-row">
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="beginning_proxy"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="ending_proxy"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
      </div>
      <action-button v-if="searchButton" id="quick-search-btn" :disabled="!enableInvalidLocation && !location_valid" principal large arrow type="submit">
        <search-icon />
      </action-button>
    </div>
    <b-field v-if="searchButton">
      <!-- <b-checkbox v-model="include_online_proxy"> -->
      <!--   Include Online Opportunities -->
      <!-- </b-checkbox> -->
    </b-field>
  </form>
  </div>
</div>
</template>
<template v-if="widget">
  <div v-if="widgetLayout=='finder-wide'" class="widget-finder-wide">
    <form @submit.prevent="search">
    <div class="gf-fields">
      <b-field label="Search" label-position="inside" data-context="find-keywords">
        <b-input ref="search_keywords" v-model="text_proxy" :name="'new-' + Math.random()" placeholder="e.g. astronomy" icon="magnify" />
      </b-field>
      <lookup-place v-model="place_proxy" @valid="set_valid" label-position="inside" data-context="find-lookup-place" widget />
      <div class="centered-row">
        <div class="date-input">
          <input type="date" v-model="beginning_proxy">
          <label>From</label>
        </div>
        <div class="date-input">
          <input type="date" v-model="ending_proxy">
          <label>Until</label>
        </div>
      </div>
    </div>
    <!-- <b-field>
      <b-checkbox v-model="include_online_proxy">
        Include Online Opportunities
      </b-checkbox>
    </b-field> -->
    <div class="center-submit-btn">
      <action-button :loading="working" :disabled="!enableInvalidLocation && !location_valid" type="is-primary" principal>
        Search
      </action-button>
    </div>
  </form>
  </div>
  <div v-if="widgetLayout=='finder-thin'" class="widget-finder-thin">
    <form @submit.prevent="search">
    <div class="gf-fields">
      <b-field label="Search" label-position="inside" data-context="find-keywords">
        <b-input ref="search_keywords" v-model="text_proxy" :name="'new-' + Math.random()" placeholder="e.g. astronomy" icon="magnify" />
      </b-field>
      <lookup-place v-model="place_proxy" @valid="set_valid" label-position="inside" data-context="find-lookup-place" stacked />
        <div class="date-input">
          <input type="date" v-model="beginning_proxy">
          <label>From</label>
        </div>
        <div class="date-input">
          <input type="date" v-model="ending_proxy">
          <label>Until</label>
        </div>
    </div>
    <div class="center-submit-btn">
      <action-button :loading="working" :disabled="!enableInvalidLocation && !location_valid" type="is-primary" principal>
        Search
      </action-button>
    </div>
  </form>
  </div>
</template>
  <div v-if="quickLinks" class="quick-links snm-container">
    <nuxt-link to="/find?physical=online&descriptors[]=citizen_science">
      Online, Anytime Science
    </nuxt-link>
    <nuxt-link :to="near_url + '&topics[]=astronomy_and_space'">
      Astronomy Near Me
    </nuxt-link>
    <nuxt-link :to="near_url + '&descriptors[]=live_science'">
      Museums Near Me
    </nuxt-link>
    <nuxt-link :to="near_url + '&descriptors[]=festival'">
      Festivals Near Me
    </nuxt-link>
  </div>
</div>
</template>

<script>
import LookupPlace from '~/components/LookupPlace'

import SearchIcon from '~/assets/img/search.svg?inline'

export default {
    name: "GeneralFilters",

    components: {
        LookupPlace,

        SearchIcon,
    },

    props: {
        text: {
            type: String,
            required: false,
            default: "",
        },

        disableFullSearch: {
            type: Boolean,
            required: false,
            default: false,
        },

        place: {
            type: Object,
            required: false,
            default: () => ({
                latitude: 0,
                longitude: 0,
                near: '',
                proximity: 0,
            }),
        },

        beginning: {
            type: String,
            required: false,
            default: () => (new Date()).toISOString().slice(0, 10),
        },

        ending: {
            type: String,
            required: false,
            default: null,
        },

        includeOnline: {
            type: Boolean,
            required: false,
            default: true,
        },

        searchButton: {
            type: Boolean,
            required: false,
            default: false,
        },

        quickLinks: {
            type: Boolean,
            required: false,
            default: false,
        },
        widget: {
            type: Boolean,
            required: false,
            default: false,
        },
        widgetLayout: {
            type: String,
            required: false,
            default: undefined,
        },

        evolveme: {
            type: Boolean,
            required: false,
            default: false,
        },

        enableInvalidLocation: {
            type: Boolean,
            required: false,
            default: false,
        },
    },

    data() {
        return {
            working: false,
            location_valid: false,
            default_location: {},
        };
    },

    computed: {
        near_url() {
            let chunks = ['/find?sort=closest'];

            if(this.$store.state.here) {
                chunks.push('&near=');
                chunks.push(encodeURIComponent(this.$store.state.here.near));

                chunks.push('&longitude=');
                chunks.push(encodeURIComponent(this.$store.state.here.longitude));

                chunks.push('&latitude=');
                chunks.push(encodeURIComponent(this.$store.state.here.latitude));

                chunks.push('&proximity=');
                chunks.push(encodeURIComponent(this.$store.state.here.proximity));
            }

            return chunks.join('');
        },

        search_url() {
            let chunks = ['/find?physical='];

            if(this.include_online_proxy) {
                chunks.push('in-person-or-online');
            }
            else {
                chunks.push('in-person');
            }

            if(this.text_proxy) {
                chunks.push('&text=');
                chunks.push(encodeURIComponent(this.text_proxy));
            }

            if(this.place_proxy.near) {
                chunks.push('&near=');
                chunks.push(encodeURIComponent(this.place_proxy.near));

                chunks.push('&longitude=');
                chunks.push(encodeURIComponent(this.place_proxy.longitude));

                chunks.push('&latitude=');
                chunks.push(encodeURIComponent(this.place_proxy.latitude));

                chunks.push('&proximity=');
                chunks.push(encodeURIComponent(this.place_proxy.proximity));
            }

            if(this.beginning_proxy) {
                chunks.push('&beginning=');
                chunks.push(new Date(this.beginning_proxy).toISOString());
            }

            if(this.ending_proxy) {
                chunks.push('&ending=');
                chunks.push(new Date(this.ending_proxy).toISOString());
            }

            chunks.push('&sort=closest');

            return chunks.join('');
        },

        include_online_proxy: {
            get() {
                return this.includeOnline;
            },

            set(val) {
                this.$emit('include-online', val);
            }
        },

        text_proxy: {
            get() {
                return this.text;
            },

            set(val) {
                this.$emit('text', val);
            }
        },

        place_proxy: {
            get() {
                return this.place;
            },

            set(val) {
                this.$emit('place', val);
            }
        },

        beginning_proxy: {
            get() {
                return this.beginning ? new Date(this.beginning) : null;
            },

            set(val) {
                this.$emit('beginning', val ? val.toISOString() : null);
            }
        },

        ending_proxy: {
            get() {
                return this.ending ? new Date(this.ending) : null;
            },

            set(val) {
                this.$emit('ending', val ? val.toISOString() : null);
            }
        },
    },

    methods: {
        search() {
            if(this.disableFullSearch) {
                this.$emit("searched");
            }
            else if(this.widget) {
                window.open('https://sciencenearme.org' + this.search_url, '_blank');
            }
            else {
                this.$router.push(this.search_url);
            }
        },

        set_valid(valid) {
            this.location_valid=valid;
            this.$emit('valid', valid);
        }
    },
}
</script>

<style lang="scss" scoped>



.general-filters {
    display: block;
    background-color: $snm-color-background-medium;
    overflow: visible;
    // padding: 0.75rem 0.75rem 0px;

    .centered-row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    input[type="date"] {
        padding: 1.2rem 1rem 0.5rem 1rem;
        border-radius: 5px;
        border: 1px solid #B4B4B4;
    }
}

#quick-search-btn {
  width: 40px;
  padding: .2rem;
  height: 48px;
  svg {
    left:0;
  }
}

@media (min-width:$tablet-screen){
    #homepage {
      .general-filters {
        display: none;
      }
    }
  }

@media (min-width: $fullsize-screen) {
  #homepage .general-filters{
    display: flex;
    padding-top: 3rem!important;
  }
    .general-filters {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        background-color: $snm-color-background-medlight;
        grid-row: 1;
        grid-column: 1 / -1;
        text-align: center;
        padding-top: 3rem;

        .datepicker {
          max-width: 140px;
          min-width: 140px;
        }

        .basic-filter-backdrop form {
            display: inline-flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            background-color: var(--secondary-color, $snm-color-element-med);
            border-radius: 10px;
            padding: 1rem 2rem 1rem 1rem;
            width: 100%;



             > div {
                display: flex;
                align-items: center;
                justify-content: center;
                width: 100%;

                div:first-child, div:nth-child(2) {
                  flex-grow: 1;
                }

                >* {
                    margin-top: 1rem;
                    color: $snm-color-element-light;
                }



                &:first-child>* {
                    position: relative;
                    top: -0.1rem;
                    margin: 0px 0.5rem;
                    height: 3rem;
                }

                .centered-row {
                    display: flex;
                    margin: 0px;

                    >* {
                        margin: 0px 0.5rem;
                    }
                }

                >.action-button {
                    position: relative;
                    top: 1px;
                }
            }
        }

        .quick-links {
            display: flex;
            justify-content: space-evenly;
            margin: 2rem 1rem 3rem 1rem;
            width: 100%;

            a {
                font-family: $snm-font-content;
                font-weight: bold;
                font-size: $snm-font-small;
                color: var(--primary-color, $snm-color-element-dark);
            }
        }
    }
    #find .general-filters .basic-filter-backdrop form {
      margin-top: 3rem;
    }
}

@media (max-width: 1099px) {
  .authenticated {
    #homepage {
      .gf-fields {
        flex-wrap: wrap;
      }
      .gf-fields > * {
        max-width: 50%;
        margin-bottom:1rem;
      }
    }
  }
}

@media (max-width:959px){
  #find {
    .gf-fields {
      padding:1rem;
    }
    .date {
      max-width: 48%;
    }
  }
}

/****** WIDGET *******/
.general-filters.widget {
    background-color: $snm-color-background-medium;
    display:block;
    padding:10px;
    padding-top:18px;

    .field {
      margin:0;
      margin-bottom:8px;
    }

    input[type='date'] {
      height:52px;
      padding-left:0.7rem;
      box-sizing: border-box;
      width: 100%;

    }
    .date-input {
      position:relative;
      width:48%;
      label {
        position: absolute;
        left: 1em;
        font-size: calc(1rem * 0.75);
        background-color: transparent;
        z-index: 5;
        white-space: nowrap;
        text-overflow: ellipsis;
        max-width: calc(100% - 2em);
        overflow: hidden;
        font-weight:bold;
        top:4px;
      }
    }
    .centered-row {
      justify-content:space-between;
    }

    .widget-finder-thin {
      .date-input {
        width:100%;
        margin-bottom:8px;
      }
    }
    .center-submit-btn {
      display: flex;
      justify-content:center;
      align-items:center;
    }

}

.evolveme.general-filters {
  background-color: transparent;
  padding-top:0;
  .basic-filter-backdrop form {
    background-color: #8E51F0;
  }
}

@media (max-width:959px) {
  .evolveme.general-filters {
    padding:1rem;
    background-color: $purple;
    border-radius: 8px;
    .centered-row {
      justify-content: flex-start;
      > div {
        width:100%;
      }
      > div:first-child {
        margin-right: 1rem;
      }
    }
  }
}

@media (max-width:488px) {
  .evolveme.general-filters{
    .centered-row {
      flex-direction: column;
    }
  }
}

</style>
