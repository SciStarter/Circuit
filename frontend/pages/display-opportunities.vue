<template>
<div class="snm-wrapper">
  <div class="snm-container base-typography snm-container-first">
    <div class="head">
      <h1>Display Science Near Me Opportunities on Your Website</h1>
      <p>There are two ways to display STEM engagement opportunities on your website:</p>

      <div class="flex promo-wrap">
        <div class="promo">
          <h2>Science Near Me Exchange <span class="tag">recommended</span></h2>
          <p>
            Customize the SNM Opportunity Finder and Search Results to
            display opportunities in an iFrame, aligned with your
            community's location and interests! Include an optional
            "Add Opportunity" feature to add your opportunities and/or
            invite local organizers to add their programs and events
            to SNM and your site without leaving your site. Access
            on-demand analytics and download all opportunities added
            through your site anytime!
          </p>
          <p>
            To set up an Exchange on your site, <a href="mailto:info@sciencenearme.org?subject=Exchange Info Request">contact us</a> for more information.
          </p>
        </div>
        <div class="promo">
          <h2>Science Near Me Widget</h2>
          <p>Display up to 10 SNM Opportunities on your website through our simple plug and play tool. Follow the instructions below.</p>
          <p>For customizable and more comprehensive options, see <q>Science Near Me Exchange</q>.</p>
        </div>
      </div>
    </div>

    <div class="nav-tab-wrapper">
      <ul class="nav-tabs">
        <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Get the Exchange</a></li>
        <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Get the Widget</a></li>
      </ul>
    </div>

    <div v-if="state==1">
      <div class="flex">
        <div class="flex1">
          <section>
            <h2>Select Which Type of Widget You'd Like</h2>
            <div>
              <b-field>
                <b-radio v-model="widgetType" native-value="project">
                  Show one or more science opportunities
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="widgetType" native-value="finder">
                  Show the finder to allow people to search science opportunities
                </b-radio>
              </b-field>
            </div>
          </section>

          <section v-if="widgetType=='project'">
            <h2>Select Your Widget Style</h2>
            <p>Make selections below on how to display science opportunities.</p>
            <div class="radio-selects">
              <h3>Header</h3>
              <b-field>
                <b-radio v-model="header" native-value="header">
                  With Header
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="header" native-value="no-header">
                  Without a Header
                </b-radio>
              </b-field>
            </div>

            <div class="radio-selects">
              <h3>How many opportunities should be displayed?</h3>
              <p>You may display up to 10 science opportunities.</p>
              <b-field>
                <b-numberinput v-model="max" min="1" max="10"  controls-position="compact"></b-numberinput>
              </b-field>
            </div>

            <div class="radio-selects">
              <h3>Widget Size</h3>
              <b-field>
                <b-radio v-model="projectSize"
                         native-value="short-thin">
                  Short and Thin
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="projectSize"
                         native-value="tall-thin">
                  Tall and Thin
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="projectSize"
                         native-value="short-wide">
                  Short and Wide
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="projectSize"
                         native-value="tall-wide">
                  Tall and Wide
                </b-radio>
              </b-field>
            </div>

            <div class="radio-selects">
              <h3>Customization</h3>
              <b-field>
                <b-radio v-model="customize" native-value="no">
                  Select from all available opportunities on Science Near Me
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="customize" native-value="yes">
                  Filter and customize opportunities
                </b-radio>
              </b-field>

              <div v-if="customize=='yes'">
                <div class="nested">
                  <h4>Kid-Friendly Only</h4>
                  <b-field>
                    <b-checkbox v-model="filters.kid">Show only kid-friendly opportunities</b-checkbox>
                  </b-field>
                </div>

                <div class="nested">
                  <h4>Location</h4>
                  <b-field>
                    <b-radio v-model="filters.location" native-value="global">
                      Global
                    </b-radio>
                  </b-field>
                  <b-field>
                    <b-radio v-model="filters.location" native-value="near">
                      In the vicinity of a specific place
                    </b-radio>
                  </b-field>
                  <div v-if="filters.location=='near'" class="nested">
                    <form>
                      <b-field>
                        <b-autocomplete
                          :loading="place_loading"
                          :data="place_matches"
                          field="near"
                          :value="filters.near"
                          :name="'new-' + Math.random()"
                          :clearable="true"
                          placeholder="e.g. Iowa City, IA"
                          @typing="place_completions"
                          @select="place_select"
                          autocomplete="off"
                          label="Place"
                          />
                      </b-field>
                    </form>
                    <label style="margin-top:10px;display:block">Allowed distance, in miles</label>
                    <b-field>
                      <b-numberinput v-model="proximity_miles" min="1" max="100" controls-position="compact"></b-numberinput>
                    </b-field>
                  </div>
                </div>
                <!-- <div class="nested">
                     <h4>Include Online Only Opportunities</h4>
                     <b-field>
                       <b-radio v-model="filters.online" native-value="yes">
                         Include online only opportunities
                       </b-radio>
                     </b-field>
                     <b-field>
                       <b-radio v-model="filters.online" native-value="no">
                         Do not include online only opportunities
                       </b-radio>
                     </b-field>
                </div> -->
                <div class="nested check-grid" v-if="descriptors && descriptors.length">
                  <h4>Activity Type</h4>
                  <b-field v-for="desc in descriptors" :key="desc[0]">
                    <b-checkbox v-model="filters.activities" :native-value="desc[0]">
                      {{desc[1]}}
                    </b-checkbox>
                  </b-field>
                </div>
                <div class="nested">
                  <h4>Organization</h4>
                  <p>Limit your results to one partner organization. Begin typing the organization name and select when it displays in the dropdown menu.</p>
                  
                  <b-autocomplete
                    v-model="filters.partner_text"
                    :data="suggested_partners"
                    :name="'new-' + Math.random()"
                    field="name"
                    clearable
                    keep-first
                    select-on-click-outside
                    @select="filters.partner = $event ? $event.uid : ''"
                    />
                </div>
              </div>
            </div>
          </section>
          
          <section v-if="widgetType=='finder'">
            <h2>Select Your Widget Style</h2>
            <div class="radio-selects">
              <h3>Widget Size</h3>
              <b-field>
                <b-radio v-model="finderSize" native-value="finder-thin">
                  Thin
                </b-radio>
              </b-field>
              <b-field>
                <b-radio v-model="finderSize" native-value="finder-wide">
                  Wide
                </b-radio>
              </b-field>
            </div>
          </section>

        </div>
        <div class="flex2">
          <iframe :src="'/'+URLparams" referrerpolicy="origin" :width="width" :height="height" scrolling="no"></iframe>
          <textarea>&lt;iframe src="{{link}}" referrerpolicy="origin" width="{{width}}" height="{{height}}" scrolling="no"&gt;&lt;/iframe&gt;</textarea>
        </div>
      </div>
    </div>

    <div v-if="state==2" class="about-exchange">
      <p>
        An exchange is intended to be embedded on your site inside an
        iframe. It has no effect on the look of your site outside the
        iframe. The exchange is a high-level integration which
        provides a user interface which you can use and offer to your
        members. As a side-effect of offering the user interface
        directly, the exchange is also able to enforce data entry
        requirements which guarantee nice-looking records on Science
        Near Me.
      </p>
      <p>
        If you have many existing opportunities you'd like to display
        on SNM and your Exchange, please select one of the options to
        share your opportunities on
        our <a href="/add-opportunities">add opportuities page</a>.
        That way all those opportunities will appear on SNM and on
        your Exchange, in addition to any others available to your
        community from SNM! You'll still have the option to use the
        Add Opportunity Form on your site to add future opportunities.
      </p>
      <h2 class="h3" style="margin-top:20px">Request an Exchange</h2>
      <ol>
        <li>Create a Science Near Me account, if you haven't done so already</li>
        <li>Send an email request to <a href="mailto:info@sciencenearme.org">info@sciencenearme.org</a>
          <p>Please include:</p>
          <ul>
            <li>The email address associated with your Science Near Me account</li>
            <li>The name of your organization</li>
            <li>Your organization's contact info (address, phone)</li>
            <li>The URL or website on which you plan to host the exchange.</li>
          </ul>
        </li>
      </ol>
    </div>
  </div>
</div>
</template>

<script>
import debounce from 'lodash/debounce'

export default {
    name: "SelectYourWidget",

    async asyncData(context) {
        return {
            partners: (await context.store.dispatch('get_partners')).filter(p => p.name != 'Internal'),
            descriptors: await context.store.dispatch('get_descriptors'),
        };
    },

    data() {
        return {
            place_matches: [],
            place_loading: false,
            widgetType: 'project',
            header: 'header',
            state:2,
            max:1,
            projectSize: 'short-thin',
            customize: 'no',
            finderSize: 'finder-thin',
            filters: {
                kid: false,
                location: 'global',
                near: '',
                latitude: 0,
                longitude: 0,
                proximity: 25,
                activities:[],
                online: 'yes',
                partner_text: '',
                partner: '',
            },
            sizes:{
                'short-thin':{
                    header: {
                        width: 200,
                        height: 325
                    },
                    'no-header':
                    {
                        width: 200,
                        height: 275
                    }
                },
                'tall-thin':{
                    header: {
                        width: 200,
                        height: 575
                    },
                    'no-header':{
                        width: 200,
                        height: 525
                    }
                },
                'short-wide':{
                    header: {
                        width: 375,
                        height: 250
                    },
                    'no-header':{
                        width: 375,
                        height: 200
                    }
                },
                'tall-wide':{
                    header: {
                        width: 375,
                        height: 475
                    },
                    'no-header':{
                        width: 375,
                        height: 425
                    },
                },
                'finder-thin': {
                    width: 200,
                    height: 445
                },
                'finder-wide':{
                    width: 375,
                    height: 310
                }
            }
        }
    },

    computed: {
        suggested_partners() {
            if(!this.filters.partner_text) {
                return [];
            }
            const text = this.filters.partner_text.toLowerCase();
            return this.partners.filter(p => p.name.toLowerCase().indexOf(text) >= 0);
        },

        proximity_miles: {
            get() {
                return Math.ceil(this.filters.proximity / 1609.34);
            },
            set(value) {
                this.filters.proximity = Math.floor(value * 1609.34);
            }
        },

        URLparams(){
            if (this.widgetType == 'project') {
                let url = `widget?layout=${this.projectSize}&style=${this.header}&max=${this.max}`;

                if(this.filters.kid) {
                    url += '&max_age=13';
                }

                if(this.filters.location == 'near' && (this.filters.longitude || this.filters.latitude)) {
                    url += '&longitude=' + this.filters.longitude;
                    url += '&latitude=' + this.filters.latitude;
                    url += '&proximity=' + this.filters.proximity;
                }

                for(let slug of this.filters.activities) {
                    url += '&descriptors[]=' + slug;
                }

                if(this.filters.online === 'yes') {
                    url += '&physical=in-person-or-online'
                }
                else if(this.filters.online === 'no') {
                    url += '&physical=in-person';
                }

                if(this.filters.partner) {
                    url += '&partner=' + this.filters.partner;
                }

                return url;
            } else {
                return `widget?layout=${this.finderSize}`
            }
        },

        link(){
            return `https://sciencenearme.org/${this.URLparams}`
        },

        width(){
            if (this.widgetType == 'project') {
                return this.sizes[this.projectSize][this.header].width
            } else {
                return this.sizes[this.finderSize].width
            }
        },

        height(){
            if (this.widgetType == 'project') {
                return this.sizes[this.projectSize][this.header].height
            } else {
                return this.sizes[this.finderSize].height
            }
        }
    },

    methods: {
        place_completions: debounce(function (near) {
            this.place_matches = []

            if (near.length < 3) {
                return
            }

            this.place_loading = true;

            this.$axios.$get('https://geocode.arcgis.com/arcgis/rest/services/World/GeocodeServer/suggest?f=json&text=' + encodeURIComponent(near))
                .then(({ suggestions }) => { this.place_matches = suggestions.map(x => x.text); })
                .catch((error) => { this.place_matches = []; console.error(error) })
                .finally(() => { this.place_loading = false })
        }, 500),

        place_select(evt) {
            if(evt === undefined) {
                return;
            }

            if(evt === null || evt === '') {
                this.filters.near = '';
                this.filters.longitude = 0;
                this.filters.latitude = 0
                this.filters.proximity = 0;
            }

            this.place_loading = true

            this.$axios.$post('/api/ui/finder/geo', { lookup: 'coords', place: { near: evt, longitude: 0, latitude: 0, proximity: this.filters.proximity }})
                .then(({ places }) => {
                    if (places.length > 0) {
                        this.filters.near = evt;
                        this.filters.longitude = places[0].longitude;
                        this.filters.latitude = places[0].latitude;
                        this.filters.proximity = places[0].proximity;
                    }
                })
                .catch((error) => { console.error(error) })
                .finally(() => { this.place_loading = false })
        },
    },
}
</script>

<style lang="scss" scoped>

.flex {
    flex-direction: column;
}
.flex1, .flex2 {
    flex:1 1 auto;
}

.flex2 {
    display: flex;
    flex-direction:column;
    align-items:center;

    textarea {
        width:100%;
        font-size:16px;
        height:150px;
        margin-top:2rem;
    }
}

.radio-selects {
    border-radius:6px;
    border:1px solid $snm-color-border;
    margin:10px 0;
    padding:10px;
}

section {
    margin-bottom:2rem;
}

.check-grid {
    display: flex;
    flex-wrap: wrap;

    > * {
        width: 12rem;
    }
    > h4 {
        flex: 0 0 100%;
    }
}

.nested {
    padding: 10px 20px;
    margin-bottom:10px;
    border-bottom:1px solid $snm-color-border;
    &:last-child {
        border-bottom:0;
    }
    .nested {
        padding:0 30px;
        border-bottom:0;
        margin-bottom:30px;
        margin-top:-10px;
    }
}

.promo-wrap {
  flex-direction: column;
  .promo {
    border:1px solid $snm-color-border;
    padding:16px;
    margin-bottom:2rem;
    border-radius: 6px;
    h2 {
      background-color: $snm-color-background-medlight;
      padding:8px 16px;
      margin: -16px -16px 16px -16px;
    }
    p {
      margin-bottom: 0;
    }
  }
}

.head {
  h1 {
    margin-bottom: .2rem;
  }
  p {
    margin-bottom: 2rem;
  }
}

@media (min-width:768px) {

    .flex {
        flex-direction: row;
    }
    .flex1 {
        flex: 1 1 auto;
        padding-right:20px;
    }

    .flex2 {
        flex:0 0 375px;
        align-self: flex-start;
        position:sticky;
        top:20px;
    }
    .authenticated .flex2 {
      top:90px;
    }
    .promo-wrap {
      flex-direction: row;
      justify-content: space-between;
      .promo {
        width:48%;
      }
    }
}

.base-typography .head {
  border:0;
  padding-bottom: 0;;
}

.about-exchange {
  ol {
    list-style-position: inside;

    li {
      margin-bottom: 16px;
      p {
        margin:16px 0;
      }
    }
  }
  ul {
    list-style-type: disc;
    margin-left: 2rem;;
  }
}

.promo .tag {
  font-size:12px;
  background-color:$snm-color-action;
  color: #fff;
  vertical-align: middle;
}


</style>
