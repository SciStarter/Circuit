<template>
<div id="content" :class="{filtering: filtering}">
  <div id="filters-general">
    <div class="basic-filter-backdrop">
      <b-field>
        <b-input ref="search_keywords" v-model="query.text" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
      </b-field>
      <lookup-place v-model="place" label-position="inside" />
      <div class="centered-row">
        <b-field label="From" label-position="inside">
          <input v-model="beginning" class="control" type="date">
        </b-field>
        <b-field label="Until" label-position="inside">
          <input v-model="ending" class="control" type="date">
        </b-field>
      </div>
    </div>
  </div>
  <div id="filters-ordering">
    <b-field id="filter-physical">
      <b-radio-button v-model="query.physical" native-value="in-person-or-online">
        <span class="radio-label">In-Person<br> &amp;&nbsp;Online</span>
      </b-radio-button>
      <b-radio-button v-model="query.physical" native-value="in-person">
        <span class="radio-label">In-Person<br> Only</span>
      </b-radio-button>
      <b-radio-button v-model="query.physical" native-value="online">
        <span class="radio-label">Online Only</span>
      </b-radio-button>
    </b-field>
    <mini-select id="filter-sort" v-model="query.sort" label="Sort:">
      <option value="closest">
        Closest
      </option>
      <option value="soonest">
        Soonest
      </option>
    </mini-select>
  </div>
  <div id="filters-refine">
    <fieldset>
      <label>Age</label>
      <b-field label="Minimum Age">
        <b-checkbox v-model="min_age_active" />
        <b-slider v-model="min_age" :disabled="!min_age_active" :min="0" :max="120" :step="1" size="is-medium" rounded>
          <b-slider-tick :value="12">
            12
          </b-slider-tick>
          <b-slider-tick :value="21">
            21
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
      <b-field label="Maximum Age">
        <b-checkbox v-model="max_age_active" />
        <b-slider v-model="max_age" :disabled="!max_age_active" :min="0" :max="120" :step="1" size="is-medium" rounded>
          <b-slider-tick :value="12">
            12
          </b-slider-tick>
          <b-slider-tick :value="21">
            21
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
      <b-taginput v-model="selected_descriptors" :data="suggested_descriptors" field="1" open-on-focus autocomplete @typing="descriptor_text = $event.toLowerCase()" />
    </fieldset>
    <fieldset>
      <label>Topic</label>
      <b-taginput v-model="selected_topics" :data="suggested_topics" field="1" open-on-focus autocomplete @typing="topic_text = $event.toLowerCase()" />
    </fieldset>
    <fieldset>
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
      <b-select v-model="venue_type">
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
    <fieldset>
      <label>Host Organization</label>
      <b-input :value="get_query('host', '')" type="text" @input="set_query('host', $event, '')" />
    </fieldset>
    <fieldset>
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
  </div>
  <section id="results">
    <article>{{ matches }}</article>
  </section>
  <div id="filter-submit">
    <button title="close filters" @click="filtering = false">
      &times;
    </button>
    <b-button @click="clear">
      Clear Filters
    </b-button>
    <b-button @click="search">
      Apply
    </b-button>
  </div>
</div>
</template>

<script>
import Vue from 'vue'
import MiniSelect from '~/components/MiniSelect'

function from_qs (qs, names) {
    const dest = {}

    for (const name of names) {
        const val = qs[name];

        if (val !== undefined) {
            dest[name.endsWith('[]') ? name.slice(0, -2) : name] = val;
        }
    }

    return dest;
}

export default {
    name: 'Find',

    components: {
        MiniSelect
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

        const { payload: results } = await context.$axios.$get('/api/ui/finder/search', { params: query });

        const partners = await context.store.dispatch('get_partners');
        const descriptors = await context.store.dispatch('get_descriptors');
        const topics = await context.store.dispatch('get_topics');

        const local = {
            filtering: false,
            pagination: {
                page: 0,
                per_page: query.per_page ? parseInt(query.per_page) : 10,
                num_pages: 0
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
            query: Object.assign({sort: 'closest'}, this.$route.query),
            partner_text: "",
            descriptor_text: "",
            topic_text: ""
        };
    },

    computed: {
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
        }
    },

    watchQuery: true,

    methods: {
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
            this.query = JSON.parse('{"physical": "in-person-or-online"}');
        },

        search() {
            this.$router.push({ name: 'find', query: this.query });
        },
    }
}
</script>

<style lang="scss" scoped>
#filters-general {
    background-color: $snm-color-background-medium;
    padding: 0.75rem 0.75rem 0px;

    .centered-row {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    input[type="date"] {
        padding: 1rem;
        border-radius: 10px;
        border: 1px solid #B4B4B4;
    }
}

#filters-ordering {
    padding: 0.75rem;
    display: flex;
    justify-content: space-between;
}

#filter-physical {
    margin: 0px auto 0.75rem;
}

#filter-sort {
    display: none;
}

::v-deep label.b-radio.radio.button {
    border-color: $snm-color-action-border;
    border-radius: 10px;

    span {
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: 14px;
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

@media only screen and (min-width: $fullsize-screen) {
    #filters-ordering {
        span.radio-label {
            /* Since I couldn't find a good way to make the label text reflow automatically */
            br {
                display: none;
            }
        }
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
            font-size: 16px;
        }
    }
}
</style>
