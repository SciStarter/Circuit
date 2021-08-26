<template>
<div id="content">
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
    <a href="https://slashdot.org/">slashdot</a>
  </div>
  <section id="results">
    <article>{{ matches }}</article>
  </section>
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

    return dest
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

        const { payload } = await context.$axios.$get('/api/ui/finder/search', { params: query });

        return Object.assign(
            {
                pagination: {
                    page: 0,
                    per_page: query.per_page ? parseInt(query.per_page) : 10,
                    num_pages: 0
                },
                opportunities: []
            },
            payload
        )
    },

    data() {
        return {
            query: Object.assign(
                {
                    sort: 'closest'
                },
                this.$route.query
            )
        };
    },

    computed: {
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
        }
    },

    methods: {
        search() {
            this.$router.push({ name: 'find', query: this.query });
        }
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
