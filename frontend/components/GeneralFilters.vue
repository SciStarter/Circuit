<template>
<div class="general-filters">
  <div class="snm-container">
  <div class="basic-filter-backdrop">
    <div class="gf-fields">
      <b-field label="Search" label-position="inside" data-context="find-keywords">
        <b-input ref="search_keywords" v-model="text_proxy" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
      </b-field>
      <lookup-place v-model="place_proxy" label-position="inside" data-context="find-lookup-place" />
      <div class="centered-row">
        <b-field label="From" label-position="inside" data-context="find-beginning">
          <b-datepicker v-model="beginning_proxy"
            editable
            icon="calendar-today" />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending">
          <b-datepicker v-model="ending_proxy"
            editable
            position="is-bottom-left"
            icon="calendar-today" />
        </b-field>
      </div>
      <action-button v-if="searchButton" id="quick-search-btn" principal large arrow @click="search">
        <search-icon />
      </action-button>
    </div>
    <b-field v-if="searchButton">
      <b-checkbox v-model="include_online_proxy">
        Include Online Opportunities
      </b-checkbox>
    </b-field>
  </div>
</div>
  <div v-if="quickLinks" class="quick-links snm-container">
    <nuxt-link to="/find?physical=online&descriptors[]=citizen_science">
      Online, Anytime Science
    </nuxt-link>
    <nuxt-link :to="search_url + '&topics[]=astronomy_and_space'">
      Astronomy Near Me
    </nuxt-link>
    <nuxt-link :to="search_url + '&topics[]=live_science'">
      Museums Near Me
    </nuxt-link>
    <nuxt-link :to="search_url + '&topics[]=festival'">
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
    },

    computed: {
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
            this.$router.push(this.search_url);
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

        .basic-filter-backdrop {
            display: inline-flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            background-color: $snm-color-element-med;
            border-radius: 10px;
            padding: 1rem 2rem 1rem 1rem;
            width: 100%;



            >div {
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
                color: $snm-color-element-dark;
            }
        }
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

</style>
