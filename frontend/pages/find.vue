<template>
<div id="find" :class="{filtering: filtering}">
  <general-filters
    id="filters-general"
    :text="query.text"
    :place="place"
    :beginning="beginning"
    :ending="ending"
    @text="query.text=$event"
    @place="place=$event"
    @beginning="beginning=$event"
    @ending="ending=$event"
    />
  <div class="snm-container">
  <div id="filters-ordering">
    <b-field id="filter-physical">
      <b-radio-button v-model="query.physical" native-value="in-person-or-online" data-context="find-sort-in-person-or-online">
        <span class="radio-label">In-Person<br> &amp;&nbsp;Online</span>
      </b-radio-button>
      <b-radio-button v-model="query.physical" native-value="in-person" data-context="find-sort-in-person">
        <span class="radio-label">In-Person<br> Only</span>
      </b-radio-button>
      <b-radio-button v-model="query.physical" native-value="online" data-context="find-sort-online">
        <span class="radio-label">Online Only</span>
      </b-radio-button>
    </b-field>
    <mini-select id="filter-sort" v-model="query.sort" label="Sort:" data-context="find-sort-order" @input="search">
      <option value="closest">
        Closest
      </option>
      <option value="soonest">
        Soonest
      </option>
    </mini-select>
    <action-button id="filter-trigger" contrast-fg @click="filtering = true">
      Refine results
    </action-button>
  </div>
  <div id="filters-refine">
    <div>
      <h2 class="no-mobile">
        Refine Results
      </h2>
      <fieldset>
        <label>Age</label>
        <b-field label="Minimum Age" data-context="find-minimum-age">
          <b-checkbox v-model="min_age_active" />
          <b-slider v-model="min_age" :disabled="!min_age_active" :min="0" :max="120" :step="1" size="is-medium" rounded>
            <!-- <b-slider-tick :value="12">
              12
            </b-slider-tick> -->
            <b-slider-tick :value="20">
              20
            </b-slider-tick>
            <b-slider-tick :value="40">
              40
            </b-slider-tick>
            <b-slider-tick :value="60">
              60
            </b-slider-tick>
            <b-slider-tick :value="80">
              80
            </b-slider-tick>
            <b-slider-tick :value="100">
              100
            </b-slider-tick>
          </b-slider>
          <input v-model="min_age" type="text" :disabled="!min_age_active" class="slider-direct">
        </b-field>
        <b-field label="Maximum Age" data-context="find-maximum-age">
          <b-checkbox v-model="max_age_active" />
          <b-slider v-model="max_age" :disabled="!max_age_active" :min="0" :max="120" :step="1" size="is-medium" rounded>
            <!-- <b-slider-tick :value="12">
              12
            </b-slider-tick> -->
            <b-slider-tick :value="20">
              20
            </b-slider-tick>
            <b-slider-tick :value="40">
              40
            </b-slider-tick>
            <b-slider-tick :value="60">
              60
            </b-slider-tick>
            <b-slider-tick :value="80">
              80
            </b-slider-tick>
            <b-slider-tick :value="100">
              100
            </b-slider-tick>
          </b-slider>
          <input v-model="max_age" type="text" class="slider-direct" :disabled="!max_age_active">
        </b-field>
      </fieldset>
      <fieldset>
        <label>Activity Type</label>
        <b-taginput v-model="selected_descriptors" :data="suggested_descriptors" field="1" open-on-focus autocomplete data-context="find-activty-type" @typing="descriptor_text = $event.toLowerCase()" />
      </fieldset>
      <fieldset>
        <label>Topic</label>
        <b-taginput v-model="selected_topics" :data="suggested_topics" field="1" open-on-focus autocomplete data-context="find-topic" @typing="topic_text = $event.toLowerCase()" />
      </fieldset>
      <fieldset data-context="find-cost">
        <label>Cost</label>
        <p>
          <b-radio v-model="cost" native-value="any">
            Any Cost
          </b-radio>
          <b-radio v-model="cost" native-value="free">
            Free Only
          </b-radio>
        </p>
      </fieldset>
      <fieldset>
        <label>Venue Type</label>
        <!-- The wireframes have this as a pair of checkboxes, but that
             implies four possible states: both checked, one box checked, the
             other box checked, or neither checked. We actually only have
             three meaningful states, so use a select instead. -->
        <b-select v-model="venue_type" data-context="find-venue-type">
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
      </fieldset>
      <fieldset data-context="find-organization">
        <label>Host Organization</label>
        <b-input :value="get_query('host', '')" type="text" @input="set_query('host', $event, '')" />
      </fieldset>
      <fieldset data-context="find-partner">
        <label>Partner Organization</label>
        <b-autocomplete
          v-model="partner_text"
          :data="suggested_partners"
          field="name"
          clearable
          keep-first
          select-on-click-outside
          @select="selected_partner = $event"
          />
      </fieldset>
      <div class="buttons no-mobile">
        <action-button tertiary @click="clear">
          Clear Filters
        </action-button>
        <action-button primary @click="search">
          Apply
        </action-button>
      </div>
      <div class="no-mobile">
        <h1>Share Your Results</h1>
        <p>Share the list by copying the link below</p>
        <a :href="query_link" @click.prevent="copy_query"><link-icon />Copy Link</a>
      </div>
    </div>
  </div>
  <section id="results">
    <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" />
  </section>
  <section id="pagination">
    <pagination :page-index="pagination.page_index" :last-page="pagination.last_page" @switch="set_query('page', $event) || search()" />
    <div class="mobile-only">
      <h1>Share Your Results</h1>
      <p>Share the list by copying the link below</p>
      <a :href="query_link" @click.prevent="copy_query"><link-icon />Copy Link</a>
    </div>
  </section>
  <div id="filters-submit">
    <button title="close filters" class="close" @click="filtering = false">
      &times;
    </button>
    <action-button @click="clear">
      Clear Filters
    </action-button>
    <action-button primary @click="search">
      Apply
    </action-button>
  </div>
</div>
</div>
</template>

<script>
import Vue from 'vue'
import copy from 'copy-to-clipboard'

import MiniSelect from '~/components/MiniSelect'
import OpportunityCard from '~/components/OpportunityCard'
import Pagination from '~/components/Pagination'
import GeneralFilters from '~/components/GeneralFilters'

import LinkIcon from '~/assets/img/link.svg?inline'

function from_qs (qs, names) {
    const dest = {}

    for (const name of names) {
        const val = qs[name];

        if (val !== undefined) {
            const is_array = name.endsWith('[]');
            dest[is_array ? name.slice(0, -2) : name] = (is_array && !Array.isArray(val)) ? [val] : val;
        }
    }

    return dest;
}

function empty_query() {
    return {
        physical: 'in-person-or-online',
        beginning: (new Date()).toISOString(),
        sort: 'closest'
    };
}

export default {
    name: 'Find',

    components: {
        MiniSelect,
        OpportunityCard,
        Pagination,

        LinkIcon
    },

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData (context) {
        const query = from_qs(context.query, [
            'longitude',
            'latitude',
            'proximity',
            'online',
            'text',
            'beginning',
            'ending',
            'physical',
            'min_age',
            'max_age',
            'topics[]',
            'descriptors[]',
            'cost',
            'venue_type',
            'host',
            'partner',
            'sort',
            'page',
            'per_page',
            'saved',
            'participated',
            'reviewing',
            'withdrawn',
            'over'
        ]);

        if(!Object.keys(query).length) {
            context.redirect({ name: 'find', query: empty_query() });
        }

        const results = await context.$axios.$get('/api/ui/finder/search', { params: query });

        const partners = await context.store.dispatch('get_partners');
        const descriptors = await context.store.dispatch('get_descriptors');
        const topics = await context.store.dispatch('get_topics');

        const local = {
            filtering: false,
            pagination: {
                page_index: 0,
                per_page: query.per_page ? parseInt(query.per_page) : 10,
                last_page: 0,
                total: 0
            },
            partners,
            descriptors,
            topics,
            opportunities: []
        };

        return Object.assign(local, results);
    },

    data() {
        return {
            query: Object.assign(empty_query(), this.$route.query),
            partner_text: "",
            descriptor_text: "",
            topic_text: ""
        };
    },

    head() {
        return {
            'title': 'Science Near Me Opportunities',
            'meta': [
                { hid: 'og:title', property: 'og:title', content: 'Science Near Me Opportunities' },
                { hid: 'og:image', property: 'og:image', content: require('~/assets/img/logo.jpg') },
            ]
        };
    },

    computed: {
        user() {
            return this.$store.state.user;
        },

        min_age_active: {
            get() {
                return this.get_query('min_age') !== undefined;
            },

            set(value) {
                this.set_query('min_age', value ? 0 : undefined);
            }
        },

        max_age_active: {
            get() {
                return this.get_query('max_age') !== undefined;
            },

            set(value) {
                this.set_query('max_age', value ? 120 : undefined);
            }
        },

        min_age: {
            get() {
                return this.get_query('min_age', 0);
            },

            set(value) {
                if(this.min_age_active) {
                    this.set_query('min_age', value);
                }
            }
        },

        max_age: {
            get() {
                return this.get_query('max_age', 120);
            },

            set(value) {
                if(this.max_age_active) {
                    this.set_query('max_age', value);
                }
            }
        },

        beginning: {
            get() {
                return this.query.beginning ? this.query.beginning.split('T')[0] : '';
            },

            set(val) {
                if(val) {
                    Vue.set(this.query, 'beginning', (new Date(val)).toISOString());
                } else {
                    Vue.delete(this.query, 'beginning');
                }
            }
        },

        ending: {
            get() {
                return this.query.ending ? this.query.ending.split('T')[0] : '';
            },

            set(val) {
                if(val) {
                    Vue.set(this.query, 'ending', (new Date(val)).toISOString());
                } else {
                    Vue.delete(this.query, 'ending');
                }
            }
        },

        place: {
            get() {
                return {
                    latitude: this.query.latitude,
                    longitude: this.query.longitude,
                    near: this.query.near,
                    proximity: this.query.proximity
                };
            },

            set(val) {
                if(val.near !== '' || (val.latitude !== 0 && val.longitude !== 0)) {
                    Vue.set(this.query, 'latitude', val.latitude);
                    Vue.set(this.query, 'longitude', val.longitude);
                    Vue.set(this.query, 'near', val.near);
                    Vue.set(this.query, 'proximity', val.proximity);
                } else {
                    Vue.delete(this.query, 'latitude');
                    Vue.delete(this.query, 'longitude');
                    Vue.delete(this.query, 'near');
                    Vue.delete(this.query, 'proximity');
                }
            }
        },

        selected_partner: {
            get() {
                const partner = this.get_query('partner', undefined);

                if(partner) {
                    return this.partners.filter(p => p.uid === partner)[0];
                }

                return undefined;
            },

            set(value) {
                console.log('P:', value);
                this.partner_text = value ? value.name : "";
                this.set_query('partner', value ? value.uid : undefined);
            }
        },

        suggested_partners() {
            const text = this.partner_text.toLowerCase();
            return this.partners.filter(p => p.name.toLowerCase().indexOf(text) >= 0);
        },

        selected_descriptors: {
            get() {
                const descriptors = this.get_query('descriptors[]', []);
                return this.descriptors.filter(opt => descriptors.indexOf(opt[0]) >= 0);
            },

            set(values) {
                this.set_query('descriptors[]', values.map(opt => opt[0]));
            }
        },

        suggested_descriptors() {
            return this.descriptors.filter(opt => opt[1].toLowerCase().indexOf(this.descriptor_text) >= 0);
        },

        selected_topics: {
            get() {
                const topics = this.get_query('topics[]', []);
                return this.topics.filter(opt => topics.indexOf(opt[0]) >= 0);
            },

            set(values) {
                this.set_query('topics[]', values.map(opt => opt[0]));
            }
        },

        suggested_topics() {
            return this.topics.filter(opt => opt[1].toLowerCase().indexOf(this.topic_text) >= 0);
        },

        cost: {
            get() {
                return this.get_query('cost', 'any');
            },

            set(value) {
                this.set_query('cost', value, 'any');
            }
        },

        venue_type: {
            get() {
                return this.get_query('venue_type', 'either');
            },

            set(value) {
                this.set_query('venue_type', value, 'either');
            }
        },

        query_link() {
            return 'https://sciencenearme.org' + this.$route.fullPath;
        }
    },

    watchQuery: true,

    methods: {
        copy_query() {
            if(navigator.clipboard !== undefined) {
                // Future: may need to request permission using the
                // navigator.permissions API. As of late 2021, no
                // browser supports requesting permissions via the
                // permissions API, much less requires it, but that's
                // what the spec says.
                navigator.clipboard.writeText(this.query_link).then(
                    () => this.$buefy.toast.open({
                        message: 'Copied to clipboard',
                        type: 'is-success'
                    }),
                    () => copy(this.query_link));
            }
            else {
                // This function uses the now-deprecated but currently
                // very well supported execCommand API to copy to the
                // clipboard, and falls back to using a prompt to
                // provide the text to copy if execCommand isn't
                // available.
                copy(this.query_link);
            }
        },

        get_query(name, fallback) {
            if(Object.getOwnPropertyDescriptor(this.query, name) !== undefined) {
                return this.query[name];
            }
            else {
                return fallback;
            }
        },

        set_query(name, value, marker) {
            if(value !== marker) {
                Vue.set(this.query, name, value);
            } else {
                Vue.delete(this.query, name);
            }
        },

        clear() {
            this.query = empty_query();
        },

        search() {
            this.filtering = false;
            this.$router.push({ name: 'find', query: this.query });
        },
    }
}
</script>

<style lang="scss" scoped>
#filters-general {
    display: none;
}

.filtering #filters-general {
    display: block;
}

#filters-ordering {
    padding: 0.75rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

#filter-physical {
    display: none;
}

.filtering #filter-physical {
    display: block;
    margin: 0px auto 0.75rem;
}

#filter-sort {
    display: block;
    flex-grow: 0
}

.filtering #filter-sort {
    display: none;
}

#filter-trigger {
    display: block;
    flex-grow: 0;
}

.filtering #filter-trigger {
    display: none;
}

#filters-refine {
    display: none;
    background-color: $snm-color-background;

    fieldset {
        margin: 2rem 32px;

        > label {
            color: $snm-color-background-dark;
            font-weight: bold;
            font-family: $snm-font-heading;
            font-size: $snm-font-small;
        }

        ::v-deep label.label {
            font-family: $snm-font-content;
            font-weight: normal;
            font-size: $snm-font-smaller;
            color: $snm-color-element-sublabel;
        }
    }
}

.filtering #filters-refine {
    display: block;
}

#filters-submit {
    display: none;
    position: fixed;
    bottom: 0px;
    left: 0px;
    right: 0px;
    background-color: $snm-color-background-medium;
    flex-direction: row;
    z-index: 10;

    button.close {
        background-color: transparent;
        border: none;
        padding: 0px;
        margin: 0px 0.5rem;
        font-size: 40px;
        cursor: pointer;
        flex-shrink: 0;
        flex-grow: 0;
    }
}

.filtering #filters-submit {
    display: flex;
}

#results {
    display: block;
}

.filtering #results {
    display: none;
}
#filters-refine .b-radio {
  margin-right: 1.5rem!important;
}

#pagination {
    display: block;

    div:last-child {
        margin: 32px;

        h1 {
            font-size: $snm-font-large;
            font-family: $snm-font-heading;
            line-height: 28px;
            letter-spacing: 0px;
            color: $snm-color-element-dark;
            font-weight: bold;
        }

        a {
            text-decoration: underline;
        }

        svg,img {
            display: inline-block;
            vertical-align: middle;
            margin-right: 0.75rem;
        }
    }
}

.filtering #pagination {
    display: none;
}

::v-deep label.b-radio.radio.button {
    border-color: $snm-color-action-border;
    border-radius: 10px;

    span {
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: $snm-font-smaller;
    }
}

.b-slider {
    width: 70%;
    width: calc(100% - 4em);
}

.slider-direct {
    width: 3em;
    margin: 0.5em 1em;
    padding: 0.25em;
    line-height: 1em;
    height: 2em;
    border-radius: 3px;
    border: 1px solid #ddd;
}

@media (min-width:$tablet-screen) {
  #results {
    padding: 0 1rem;
  }
  #filters-ordering {
      grid-row: 1;
      grid-column: 1;
  }
}

@media (min-width: 960px) {
  #find > .snm-container {
      display: grid;
      grid-template-columns: 60% 40%;
      grid-template-rows: 4rem 1fr minmax(2rem, auto);
      row-gap: 0.5rem;
      margin-top: 1rem;
  }
  #filters-ordering {
    grid-row: 1;
    grid-column: 1/3;
    justify-content: flex-start;
    #filter-physical {
      margin-right: 1rem;
    }
  }
  #filters-refine {
      grid-column: 2;
      grid-row: 2/-1;
      display: block;
  }
  #results {
      grid-row: 2;
      grid-column: 1;
  }
  #pagination {
      grid-row: 3;
      grid-column: 1;
  }
}

@media (min-width: $fullsize-screen) {

    #find .general-filters {
      padding-bottom: 2rem;
      padding-left:1rem;
      padding-right: 1rem;
    }
    #find .general-filters .basic-filter-backdrop {
      margin-bottom: 2rem;
    }

    #filters-general {
        display: block;
        padding:0;
    }

    #filters-ordering {

        span.radio-label {
            /* Since I couldn't find a good way to make the label text reflow automatically */
            br {
                display: none;
            }
        }
    }

    #filter-physical {
        display: block;
        margin: 0px;
    }

    #filter-sort {
        display: inline-block;
    }

    ::v-deep label.b-radio.radio.button {
        border-color: $snm-color-action-border;
        border-radius: 10px;

        span {
            font-family: $snm-font-content;
            font-weight: normal;
            font-size: $snm-font-small;
        }
    }

    #filter-trigger {
        display: none;
    }

    #filters-refine {

        fieldset {
            margin: 1rem 32px;
        }

        >div {
            position: sticky;
            top: 0px;
            display: flex;
            flex-direction: column;

            >h2 {
                font-family: $snm-font-heading;
                font-weight: bold;
                font-size: $snm-font-large;
                color: $snm-color-element-dark;
                margin: 1.5rem 32px 0px;
            }

            >div.buttons {
                margin: 0px 32px;
            }

            >div:last-child {
                margin: 32px;

                h1 {
                    font-size: $snm-font-large;
                    font-family: $snm-font-heading;
                    line-height: 28px;
                    letter-spacing: 0px;
                    color: $snm-color-element-dark;
                    font-weight: bold;
                }

                a {
                    text-decoration: underline;
                }

                svg,img {
                    display: inline-block;
                    vertical-align: middle;
                    margin-right: 0.75rem;
                }
            }
        }
    }
    .authenticated {
      #filters-refine > div {
        top: 70px;
      }
    }

    #results {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-evenly;
        padding: 0 1rem;
    }

}

@media (min-width: 1200px) {
  #find > .snm-container {
      display: grid;
      grid-template-columns: 1fr rem(340px);
      // grid-template-columns: 1fr minmax(calc(#{$fullsize-screen} - 50rem), 60rem) 25rem 1fr;
      grid-template-rows: 4rem 1fr minmax(2rem, auto);
      padding: 0 1rem;
  }
  #filters-refine {
      grid-column: 2;
      grid-row: 1/-1;
      display: block;
  }
  #results {
      grid-row: 3;
      grid-column: 1;
  }
  #pagination {
      grid-row: 4;
      grid-column: 1;
  }
  #find .general-filters {
    padding-left:2rem;
    padding-right: 2rem;
  }
}
</style>
