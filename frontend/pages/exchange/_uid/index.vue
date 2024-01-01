<template>
<div class="exchange exchange-index" :class="{'loggedin':$store.state.user.authenticated}">

  <div class="exchange-actions" v-if="$store.state.user.authenticated">

    <button  v-if="$store.state.user.authenticated" class="toggle-menu mobile-only" title="Toggle menu" data-context="header-menu" @click="toggle_mobile_nav = !toggle_mobile_nav">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data">
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <div class="exchange-nav" :class="{'show':toggle_mobile_nav}">
      <template v-if="partner !== null">
        <nuxt-link :to="{name: 'exchange-uid', params: {uid: $route.params.uid}}" class="home" title="home"><home-icon /><span class="home-text">Home</span></nuxt-link>
        <nuxt-link :to="{name: 'exchange-uid-partner', params: {uid: partner.uid}}">Manage Organization</nuxt-link>
      </template>
      <nuxt-link v-if="$store.state.user.authenticated" :to="{name: 'exchange-uid-opps', params: {uid: exchange.uid}}">Manage Opportunities</nuxt-link>
      <nuxt-link v-if="partner !== null || ($store.state.user.authenticated && exchange.open_submission)" :to="{name: 'exchange-uid-submit', params: {uid: exchange.uid}}" class="button"><submit-opportunity-icon/> Add an Opportunity</nuxt-link>
    </div>

    <div class="exchange-logins">
      <div v-if="$store.state.user.authenticated">
        <a @click="$store.dispatch('logout')">Logout</a>
      </div>
    </div>


  </div><!-- .exchange-actions -->


  <!-- <div class="partner-logo"></div> -->

  <div class="exchange-wrapper">
    <div class="exchange-search  general-filters">
      <div class="ex-search basic-filter-backdrop">
        <form>
          <div class="gf-fields">
            <b-field label="Search" label-position="inside" data-context="find-keywords">
              <b-input ref="search_keywords" v-model="search_text" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
            </b-field>
            <lookup-place v-model="search_place" label-position="inside" data-context="find-lookup-place" />
            <!-- <div class="centered-row">
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
            </div> -->
            <action-button
              id="quick-search-btn"
              principal
              large
              type="button"
              @click="search({
                      text: search_text,
                      longitude: search_place.longitude,
                      latitude: search_place.latitude,
                      proximity: search_place.proximity,
                      beginning: beginning ? beginning : undefined,
                      ending: ending ? ending : undefined,
                      kids_only: kids_only,
                      adults_only: adults_only,
                      physical: physical,
                      min_age: min_age,
                      max_age: max_age,
                      near: search_place.near,
                      page: 0
                      })">
              <search-icon /> <span class="mobile-only">search</span>
            </action-button>
          </div>
        </form>

      </div>

    </div>

    <div class="exchange-filters">

      <transition name="slide">
        <div class="quickfilter">
          <!-- <div class="qf-button-group">
              <button type="button" :class="{'active':quickfilter_location=='In Person'||quickfilter_location=='in-person'}" @click="quickFilterLocation('In Person')">In Person</button>
              <button type="button" :class="{'active':quickfilter_location=='Online'||quickfilter_location=='online'}" @click="quickFilterLocation('Online')">Online</button>
          </div>
          <div class="qf-button-group">
              <button type="button" :class="{'active':quickfilter_time=='Scheduled'||quickfilter_time=='scheduled'}" @click="quickFilterTime('Scheduled')">Scheduled</button>
              <button type="button" :class="{'active':quickfilter_time=='On Demand'||quickfilter_time=='on-demand'}" @click="quickFilterTime('On Demand')">On Demand</button>
          </div> -->

          <button v-if="filter==false" type="button" @click="filter = true"><filter-icon /> More Filters</button>
          <button v-else type="button" @click="filter = false"><filter-icon /> Hide Filters</button>

        </div>
      </transition>


      <transition name="slide">
      <div v-if="filter==true">
        <div class="filters">
          <fieldset>

            <label class="b">Format</label>
            <b-field>
                <b-select placeholder="Format" v-model="selected_format">
                    <option
                        v-for="option in formats"
                        :value="option.format"
                        :key="option.format">
                        {{ option.title }}
                    </option>
                </b-select>
            </b-field>

            <label class="b">Date</label>
            <b-field>
                <b-select v-model="selected_date" @input="handleDate" placeholder="Date">
                    <option
                        v-for="option in periods"
                        :value="option.period"
                        :key="option.period">
                        {{ option.title }}
                    </option>
                </b-select>
            </b-field>

            <label class="b">Age</label>
            <p>
              <b-checkbox v-model="kids_only" :native-value="true" :disabled="loading">
                Kids Friendly Only
              </b-checkbox>
              <b-checkbox v-model="adults_only" :native-value="true" :disabled="loading">
                21+ Only
              </b-checkbox>
            </p>
            <b-field label="Participant Age Range Minimum" data-context="find-minimum-age">
              <b-checkbox v-model="min_age_active" :disabled="loading" />
              <b-slider v-model="min_age" :disabled="!min_age_active || loading" :min="0" :max="120" :step="1" size="is-medium" rounded>
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
              <input v-model="min_age" type="text" :disabled="!min_age_active || loading" class="slider-direct">
            </b-field>
            <b-field label="Participant Age Range Maximum" data-context="find-maximum-age">
              <b-checkbox v-model="max_age_active" :disabled="loading" />
              <b-slider v-model="max_age" :disabled="!max_age_active || loading" :min="0" :max="121" :step="1" size="is-medium" rounded>
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
          <!-- <fieldset data-context="find-physical">
            <label>
              <span  class="b">Format</span>
              <b-tooltip multilined>
                <b-icon icon="help-circle" />
                <template #content>
                  <p><Strong>In-Person:</strong> probably has face-to-face interactions, possibly some travel</p>
                  <p><strong>On-Demand:</strong> probably done independently at your leisure, possibly over the internet</p>
                </template>
              </b-tooltip>
            </label>
            <b-field id="filter-physical" >
              <b-radio-button v-model="physical" native-value="in-person-or-online" :disabled="loading" data-context="find-sort-in-person-or-online">
                <span class="radio-label">Everything</span>
              </b-radio-button>
              <b-radio-button v-model="physical" native-value="in-person" :disabled="loading" data-context="find-sort-in-person">
                <span class="radio-label">In-Person</span>
              </b-radio-button>
              <b-radio-button v-model="physical" native-value="online" :disabled="loading" data-context="find-sort-online">
                <span class="radio-label">On-Demand</span>
              </b-radio-button>
            </b-field>
          </fieldset> -->
        </div>


      </div>
      </transition>
    </div>



    <div class="exchange-results">
      <b-tabs v-if="calendar" v-model="opp_view">
        <b-tab-item label="Calendar">
          <opportunity-calendar :opportunities="month" :exchange="exchange" :has-previous="has_calendar_previous" @prev="month_add(-1);update_calendar()" @next="month_add(1);update_calendar()" />
        </b-tab-item>
        <b-tab-item label="Opportunities">
          <opportunity-list :opportunities="opportunities" :exchange="exchange" @switch="search({page: $event})" show-top-pagination />
        </b-tab-item>
      </b-tabs>
      <opportunity-list v-else :opportunities="opportunities" :exchange="exchange" @switch="search({page: $event})" show-top-pagination />
    </div>
  </div><!-- .exchange-wrapper -->

  <div class="exchange-logins exl-bottom">
    <div v-if="$store.state.user.authenticated">
      <a @click="$store.dispatch('logout')">Logout</a>
    </div>
    <div v-else class="e">
      <template v-if="exchange.open_submission">
        <nuxt-link :to="{name: 'exchange-uid-login', params: {uid: $route.params.uid}, query: {next: $route.path}}">Login</nuxt-link> or
        <nuxt-link :to="{name: 'exchange-uid-signup', params: {uid: $route.params.uid}, query: {next: $route.path}}">Signup</nuxt-link> to add an opportunity
      </template>
      <!-- <template v-else> -->
      <!--   <nuxt-link :to="{name: 'exchange-uid-login', params: {uid: $route.params.uid}, query: {next: $route.path}}">Login</nuxt-link> | -->
      <!--   <nuxt-link :to="{name: 'exchange-uid-signup', params: {uid: $route.params.uid}, query: {next: $route.path}}">Signup</nuxt-link> -->
      <!-- </template> -->
    </div>
  </div>

  <div class="exchange-power"><div>powered by <a href="http://sciencenearme.org" target="_blank">Science Near Me</a></div></div>

  <b-modal v-model="show_datepicker" aria-role="dialog" aria-label="Select a custom date range" aria-modal>
            <div class="card">
                <header class="modal-card-head">
                    <p class="modal-card-title">Select a date range</p>
                    <button
                        type="button"
                        class="delete"
                        @click="$emit('close')"/>
                </header>
                <section class="modal-card-body">
                    <b-field>
                        <b-datepicker
                            inline
                            placeholder="Click to select..."
                            :min-date="minDate"
                            :max-date="maxDate">
                        </b-datepicker>
                    </b-field>
                </section>
                 <footer class="modal-card-foot">
                    <b-button
                        label="Close"
                        @click="$emit('close')" />
                    <b-button
                        label="Apply Dates"
                        type="is-primary" />
                </footer>
            </div>
        </b-modal>



</div>
</template>

<script>
import Vue from 'vue'
import HomeIcon from '~/assets/img/home.svg?inline'
import SubmitOpportunityIcon from '~/assets/img/submit-opportunity.svg?inline'
import LookupPlace from '~/components/LookupPlace'
import SearchIcon from '~/assets/img/search.svg?inline'
import FilterIcon from '~/assets/img/filter2.svg?inline'
export default {
    name: "ExchangeIndex",
    components: {
        SubmitOpportunityIcon,
        HomeIcon,
        SearchIcon,
        LookupPlace,
        FilterIcon
    },
    
    props: {
        partner: {
            type: Object,
            required: false,
            default: null,
        },
        
        exchange: {
            type: Object,
            required: true,
        },
    },
    
    async asyncData(context) {
        let partner = await context.$axios.$get(
            '/api/ui/organization/' + context.params.uid + '/public'
        );
        
        let default_query = !!partner.default_query ? Object.fromEntries(new URLSearchParams(partner.default_query).entries()) : {};
        
        let query = {...context.query};
        
        let calendar = (query.calendar !== undefined);
        
        if(!query.all) {
            query.partner = context.params.uid;
        }
        
        if(!query.impartial) {
            query.prefer_partner = context.params.uid;
        }
        
        if(query.page === undefined) {
            query.page = 0;
        }
        
        if(query.beginning === undefined) {
            query.beginning = new Date().toISOString();
        }
        
        let search_text = query.text || '';
        let search_place = {near: query.near || 0, longitude: query.longitude || 0, latitude: query.latitude || 0, proximity: query.proximity || 0};
        let beginning = query.beginning;
        let ending = query.ending;
        let min_age = parseInt(query.min_age) || 0;
        let max_age = parseInt(query.max_age) || 121;
        let adults_only = !!query.adults_only && query.adults_only != 'false';
        let kids_only = !!query.kids_only && query.kids_only != 'false';
        let physical = query.physical || 'in-person-or-online';
        let temporal = query.temporal || 'on-demand-or-scheduled';
        
        let opps = await context.$axios.$get('/api/ui/finder/search', { params: {...default_query, ...query} });
        
        let now = new Date();
        let search_year = now.getFullYear();
        let search_month = now.getMonth() + 1;
        let month = null;
        
        let base_year = search_year;
        let base_month = search_month;
        
        if(calendar) {
            month = await context.$axios.$get('/api/ui/finder/search', { params: {...default_query, ...query, year: search_year, month: search_month } });
        }
        
        return {
            quickfilter_location: physical,
            quickfilter_time: temporal,
            opportunities: opps,
            calendar,
            month,
            search_year,
            search_month,
            base_year,
            base_month,
            search_text,
            search_place,
            beginning,
            ending,
            min_age,
            max_age,
            adults_only,
            kids_only,
            physical,
            temporal
        };
    },
    
    data() {
      const today = new Date()
        return {
            toggle_mobile_nav: false,
            alert: false,
            opp_view: 0,
            filter: false,
            loading: false,
            num_reloads: 0,
            selected_date: null,
            selected_format: null,
            show_datepicker: false,
            minDate: new Date(today.getFullYear() - 80, today.getMonth(), today.getDate()),
            maxDate: new Date(today.getFullYear() + 18, today.getMonth(), today.getDate()),
            formats: [
                {
                    format: "event_in_person",
                    title: "Live, In-Person Event"
                },
                {
                    format: "virtual",
                    title: "Virtual Events"
                },
                {
                    format: "region_citsci",
                    title: "Science Activity in My Area"
                },
                {
                    format: "on_demand",
                    title: "On Demand Science Activities"
                },
                {
                    format: "alll",
                    title: "All Science Opportunities"
                },
            ],
            periods: [
                {
                    period: "today",
                    title: "Today"
                },
                {
                    period: "tomorrow",
                    title: "Tomorrow"
                },
                {
                    period: "this_week",
                    title: "This Week"
                },
                {
                    period: "next_week",
                    title: "Next Week"
                },
                {
                    period: "this_month",
                    title: "This Month"
                },
                {
                    period: "next_month",
                    title: "Next Month"
                },
                {
                    period: "custom",
                    title: "Custom"
                }
            ],
        };
    },
    
    computed: {
        default_query() {
            if(this.exchange && this.exchange.default_query) {
                let defaults = new URLSearchParams(this.exchange.default_query);
                return Object.fromEntries(defaults.entries());
            }
            else {
                return {};
            }
        },
        
        has_calendar_previous() {
            return this.search_year > this.base_year || (this.search_year == this.base_year && this.search_month > this.base_month);
        },
        
        beginning_proxy: {
            get() {
                return this.beginning ? new Date(this.beginning) : null;
            },
            
            set(val) {
                this.beginning = val.toISOString();
            }
        },
        
        ending_proxy: {
            get() {
                return this.ending ? new Date(this.ending) : null;
            },
            
            set(val) {
                this.ending = val.toISOString();
            }
        },
        
        min_age_active: {
            get() {
                return this.min_age !== undefined && this.min_age > 0;
            },
            
            set(value) {
                this.min_age = value ? 1 : 0;
            }
        },
        
        max_age_active: {
            get() {
                return this.max_age !== undefined && this.max_age < 121;
            },
            
            set(value) {
                this.max_age = value ? 120 : 121;
            }
        },
    },
    
    watchQuery: true,
    
    async mounted() {
        if(!this.search_place.near) {
            this.search_place = await this.$store.dispatch("get_here");
            this.search({
                longitude: this.search_place.longitude,
                latitude: this.search_place.latitude,
                near: this.search_place.near,
                beginning: this.beginning ? this.beginning : undefined,
                proximity: this.search_place.proximity || 80467,
                sort: "closest",
                page: 0
            });
        }
    },

    methods: {
        month_add(offset) {
            let sum = this.search_month + offset - 1;

            let zmonth = sum  % 12;

            // Compensate for JavaScript's incorrect modulo operation
            while(zmonth < 0) {
                zmonth += 12;
            }

            let yoffset = Math.floor(sum / 12);

            this.search_year += yoffset;
            this.search_month = zmonth + 1;
        },

        async update_calendar() {
            this.month = await this.$axios.$get('/api/ui/finder/search', { params: {...this.$route.query, year: this.search_year, month: this.search_month } });
        },

        search(assign) {
            let q = {...this.default_query, ...this.$route.query, ...assign};

            if(q.beginning === undefined) {
                q.beginning = new Date().toISOString();
            }

            if(q.min_age < 1 || q.min_age >= 121) {
                delete q.min_age;
            }

            if(q.max_age < 1 || q.max_age >= 121) {
                delete q.max_age;
            }

            q.r = this.num_reloads += 1;

            this.$router.push({name: 'exchange-uid', params: this.$route.params, query: q});
        },
        quickFilterLocation(value,event){
            if (value == this.quickfilter_location) {
                this.quickfilter_location = null;
            } else {
                this.quickfilter_location = value;
            }

            switch(this.quickfilter_location) {
            case "In Person":
                this.search({'physical': 'in-person'});
                break;
            case "Online":
                this.search({'physical': 'online'});
                break;
            default:
                this.search({'physical': 'in-person-or-online'});
            }
        },

        quickFilterTime(value,event){
            if (value == this.quickfilter_time) {
                this.quickfilter_time = null;
            } else {
                this.quickfilter_time = value;
            }

            switch(this.quickfilter_time) {
            case 'Scheduled':
                this.search({'temporal': 'scheduled', 'page': 0});
                break;
            case 'On Demand':
                this.search({'temporal': 'on-demand', 'page': 0});
                break;
            default:
                this.search({'temporal': 'on-demand-or-scheduled', 'page': 0});
            }
        },
        handleDate(v) {
            if (v == 'custom') {
                this.show_datepicker = true;
            }
        }
    },
}
</script>

<style lang="scss" scoped>

.exchange {
  display:flex;
  flex-direction:column;
  min-height:100vh;
  .exchange-wrapper {
    flex-grow:1;
  }
}

.exchange-power {
  display:flex;
  justify-content:center;
  padding: 5px 20px;
}

.partner-logo {
    width: 300px;
    height: 200px;
    background: var(--logo-url);
    background-size: contain;
    background-repeat: no-repeat;
}
.exchange-search {
  display:flex;
  flex-direction:column;
  align-items:center;
  margin:20px 0;
}
.ex-search {
  display:flex;
  justify-content:center;
  width:100%;
  .control {
    max-width:800px;
    flex-grow:1;
  }
}
.search-snm {
  margin-top:10px;
  label {
    color: #999;
  }
  .b-checkbox.checkbox input[type=checkbox] + .check {
    border:1px solid #999;
  }
}

.exchange-results {
  display:flex;
  flex-direction:column;
  justify-content:center;
  align-items:center;
  :deep(.opportunity-list > article) {
    width:100%!important;
    max-width: 900px;
  }
  :deep(.opportunity-calendar > article) {
    width:100%!important;
    max-width: 900px;
  }
}

.exchange-filters {
  width:100%;
  max-width:900px;
  margin:0 auto;
  padding: 0 16px;
  margin-bottom: 10px;
  position: relative;

  .filter-btn {
    display:flex;
    justify-content:flex-end;
  }

  svg {
    width:14px;
    height:14px;
    position:relative;
    top:2px;
    margin-right:3px;
  }
  .filters {
    padding:24px;
    background-color:#efefef;
    border-radius:6px;
    

    p {
      margin-bottom:16px;
    }

    .b {
      font-weight:bold;
      line-height:30px;
    }

    fieldset:first-child {
      margin-bottom:20px;
    }
    .b-slider {
      max-width:300px;
      margin-right:10px;
    }
  }
}

/*********** NAVIGATION *****/
.exchange-actions {
  display:flex;
  justify-content:space-between;
  background-color: #efefef;
  padding:8px 20px;
  position:sticky;
  top:0;
  z-index:999;

  .button {
    color: #087a91;
    svg {
      vertical-align: middle;
      position: relative;
      top: -2px;
      margin-right:10px;
      path {
        fill: #087a91;
      }
    }
  }
  a:not(.button):hover {
    text-decoration:underline;
  }
  .home {
    width:20px;
    svg {
      width:20px;
      height:20px;
      path {
        fill: #087a91!important;
      }
    }
  }
}


.exchange-logins {
  margin-left: auto;
  display: flex;
  align-items: center;
  &.exl-bottom {
    margin-right: auto;
  }
}
.exchange-nav {
  display: flex;
  align-items: center;
}
.exchange-nav a {
  margin-right:10px;
  margin-left:10px;
  &:first-child {
    margin-left:0;
  }
}

@media (min-width:701px){
  .toggle-menu,.home-text {
    display:none!important;
  }
}
@media (max-width:700px){
  .exchange-nav {
    flex-direction:column;
    position:absolute;
    top:45px;
    left:0;
    width:100%;
    z-index:100;
    background-color:#efefef;
    display:none;
    a {
      width:100%;
      margin:0;

      &:not(.button){
        padding:16px;
        border-top:1px solid #fff;
      }
      &.button {
        width: calc(100% - 32px);
        margin: 10px auto;
      }
    }
    .home {
      width:100%;
      svg {
        display:none;
      }
    }
  }

  .exchange-nav {
    align-items: flex-start;
    &.show {
      display:flex;
    }
  }
  .toggle-menu {
    border:0;
  }
}

/*** search ***/


.general-filters {
    display: block;
    overflow: visible;
    // padding: 0.75rem 0.75rem 0px;

    // .centered-row {
    //     display: flex;
    //     flex-direction: row;
    //     justify-content: space-between;
    // }

    input[type="date"] {
        padding: 1.2rem 1rem 0.5rem 1rem;
        border-radius: 5px;
        border: 1px solid #B4B4B4;
    }
}

#quick-search-btn {
  width: auto;
  padding: .5rem;
  height: 36px;
  svg {
    left:0;
    width: 25px;
    height: 20px;
    margin-right: 0.5rem;
  }
  span {
    margin-right: 1rem;
  }
}



@media (min-width: 768px) {


    .general-filters {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        // grid-row: 1;
        // grid-column: 1 / -1;
        // text-align: center;
        max-width:900px;
        margin:20px auto;

        .datepicker {
          max-width: 140px;
          min-width: 140px;
        }

        #quick-search-btn {
          width: 50px;
          padding: .2rem;
          height: 48px;
          svg {
            left:0;
            height: 25px;
            margin-right: 0;
          }
          span {
            display: none;
          }
        }

        .basic-filter-backdrop form {
            // display: inline-flex;
            // flex-direction: column;
            // align-items: center;
            // justify-content: center;
            // border-radius: 10px;
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
    .opportunity-list {
      width:100%;
      max-width:900px;
      padding: 0 20px;
    }

    .quickfilter {
      position: relative!important;
      top: auto!important;
      right: auto!important;
      margin: 0 0 1rem;
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

// @media (max-width:959px){
//   #find {
//     .gf-fields {
//       padding:1rem;
//     }
//     .date {
//       max-width: 48%;
//     }
//   }
// }

@media (max-width:767px) {
  .ex-search {
    padding:0 20px;
    form {
      width:100%;
    }
  }
  .gf-fields {
    display: flex;
    flex-direction:column;

    .date {
          width: 48%;
        }
         .centered-row {
            display: flex;
            flex-direction: row;
            justify-content: space-between;
        }
  }

  #quick-search-btn {
    margin-right:auto;
    margin-left: 0;
    margin-top: 0;
  }
  .field.lookup-place {
    margin-top:0;
  }
  .opportunity-list {
    width:100%;
  }
}

@media (max-width:400px) {
  :deep(.filters .field.has-addons) {
    flex-wrap: wrap;
    .b-slider {
      max-width: calc(100% - 55px);
    }
    .slider-direct {
      width: 100px;
      margin-top:6px;
      margin-left:55px;
    }
  }
}

:deep(#filter-physical .field.has-addons) {
  flex-wrap:nowrap;
  flex-direction: column;
}

.loggedin  {
:deep(div.pagination-selector.small){
    top: 56px;
}
}

@media (max-width:767px){
  .exchange-results {
    padding:0 1rem;
  }

}

@media (max-width:700px){
  .loggedin  {
    :deep(div.pagination-selector.small){
        top: 45px;
    }
  }
}

.quickfilter {
  display: flex;
  // margin:.75rem .3rem;
  align-items: center;
  position: absolute;
    top: -60px;
    right: 10px;

  button, :deep(.mini-select) {
    font-size: .75rem;
    padding: .25rem .5rem;
    border:1px solid #d4d4d4;
    border-radius: 6px;
    margin-right: 8px;
    cursor: pointer;

    svg {
      width: 16px;
    vertical-align: middle;
    height: 16px;
    position: relative;
    top: -1px;
    margin:-2px 0;
    }
  }
}
.qf-button-group {
  margin-right: 8px;
  button {
    font-size: .75rem;
    padding: .25rem .5rem;
    border:1px solid #d4d4d4;
    cursor: pointer;
    margin-right: 0;
    &:first-child {
      border-radius: 6px 0 0 6px;
    }
    &:last-child {
      border-radius: 0 6px 6px 0;
    }
    &.active {
      background-color: $snm-color-action;
      border-color: $snm-color-action;
    }
  }
}

// @media (max-width:959px){
//   .quickfilter {
//     margin: 1rem 0 0;
//   }
// }

:deep(.modal .modal-content){
    width: auto;
    overflow: auto;
}

@media (max-width: 768px){
    :deep(.modal .modal-content){
        max-width: 960px;
        padding: 0;
        max-height: 100vh;
        margin: 0;
    }
}
</style>
