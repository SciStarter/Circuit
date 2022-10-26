<template>
<div class="your-data-overview snm-container">
  <b-select v-if="available_orgs.length > 1" v-model="org" placeholder="Select Organization">
    <option v-for="org_name in available_orgs" :value="org_name" :key="org_name">
      {{org_name}}
    </option>
  </b-select>

  <div class="flex-header">
    <h1>Data Overview</h1>
  </div>

  <div id="current-proportion" class="labeled-gauge-ends">
    <progress-gauge class="gauge" :value="report[org].total_opportunities - report[org].current_opportunities" :max="report[org].total_opportunities" reverse/>
    <div class="labels">
      <div class="stack">
        <label>{{report[org].total_opportunities}}</label>
        <small>Opportunities<br>Total on SNM</small>
      </div>
      <div class="stack">
        <label>{{report[org].current_opportunities}}</label>
        <small>Opportunities<br>Currently Live</small>
      </div>
    </div>
  </div>

  <div class="nav-tab-wrapper">
    <ul class="nav-tabs">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'">Engagement</a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'">Audience</a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'">Traffic</a></li>
    </ul>
  </div>

  <aside>Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">
    <h2>Engagement</h2>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report[org].engagement.data.opportunity_status" @input="log('TBD download from server')">
          <option v-for="status in report[org].engagement.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report[org].engagement.data.time_period" @input="log('TBD download from server')">
          <option v-for="period in report[org].engagement.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
      <div class="extra">
        <a @click="save_engagement_csv">export csv</a>
      </div>
    </div>

    <line-chart
      :rows="report[org].engagement.data.chart"
      :xaxis="d => new Date(d.date)"
      :yaxes="['Views', 'Unique', 'Clicks to Website']"
      :colors="['#268699', '#BFDCE2', '#FABF40']"
      />

    <table>
      <thead>
        <tr>
          <th>Top Performing Opportunities</th>
          <th>Total Views
            <a v-if="engagement_top_order == 'total_views_desc'" @click="engagement_top_order = 'total_views_asc'">&bigvee;</a>
            <a v-else-if="engagement_top_order == 'total_views_asc'" @click="engagement_top_order = 'total_views_desc'">&bigwedge;</a>
            <a v-else @click="engagement_top_order = 'total_views_desc'">&bigcirc;</a>
          </th>
          <th>Clicks to Website
            <a v-if="engagement_top_order == 'clicks_desc'" @click="engagement_top_order = 'clicks_asc'">&bigvee;</a>
            <a v-else-if="engagement_top_order == 'clicks_asc'" @click="engagement_top_order = 'clicks_desc'">&bigwedge;</a>
            <a v-else @click="engagement_top_order = 'clicks_desc'">&bigcirc;</a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in engagement_top_sorted">
          <td>{{row['name']}}</td>
          <td><comparison-bar :value="row['Views']" :max="report[org].engagement.data.max['Views']" color="#268699" /></td>
          <td><comparison-bar :value="row['Clicks to Website']" :max="report[org].engagement.data.max['Clicks to Website']" color="#FABF40" /></td>
        </tr>
      </tbody>
    </table>

  </div>

  <div v-else-if="state=='states'">
    <h2>Audience</h2>

    <div class="notification">
      <label>Demographics Coming Soon!</label>
      We are working on getting demographic data at the opportunity level. Right now you can view <nuxt-link to="/">site-wide demographic data</nuxt-link>.
    </div>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report[org].states.data.opportunity_status" @input="log('TBD download from server')">
          <option v-for="status in report[org].states.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report[org].states.data.time_period" @input="log('TBD download from server')">
          <option v-for="period in report[org].states.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
    </div>

    <choropleth-states v-if="selected_state === null" :value="report[org].states.data.states" attr="Unique Users" @state="select_state($event)"/>
    <div v-else>
      <a @click="selected_state = null">← Back to US Map</a>
      <b-select v-model="selected_attr" placeholder="Select Data Type">
        <option>Unique Users</option>
        <option>New Users</option>
        <option>Returning Users</option>
        <option>Total Pageviews</option>
        <option>Unique Pageviews</option>
        <option>Avg. Time</option>
      </b-select>
      <activity-regional :state="selected_state" :data="selected_state_data" :attr="selected_attr" />
    </div>

    <table>
      <thead>
        <tr>
          <th>Engagement By Location</th>
          <th>Unique Users
            <a v-if="states_top_order == 'unique_users_desc'" @click="states_top_order = 'unique_users_asc'">&bigvee;</a>
            <a v-else-if="states_top_order == 'unique_users_asc'" @click="states_top_order = 'unique_users_desc'">&bigwedge;</a>
            <a v-else @click="states_top_order = 'unique_users_desc'">&bigcirc;</a>
          </th>
          <th>New Users
            <a v-if="states_top_order == 'new_users_desc'" @click="states_top_order = 'new_users_asc'">&bigvee;</a>
            <a v-else-if="states_top_order == 'new_users_asc'" @click="states_top_order = 'new_users_desc'">&bigwedge;</a>
            <a v-else @click="states_top_order = 'new_users_desc'">&bigcirc;</a>
          </th>
          <th>Returning Users
            <a v-if="states_top_order == 'returning_users_desc'" @click="states_top_order = 'returning_users_asc'">&bigvee;</a>
            <a v-else-if="states_top_order == 'returning_users_asc'" @click="states_top_order = 'returning_users_desc'">&bigwedge;</a>
            <a v-else @click="states_top_order = 'returning_users_desc'">&bigcirc;</a>
          </th>
          <th>Total Pageviews
            <a v-if="states_top_order == 'total_pageviews_desc'" @click="states_top_order = 'total_pageviews_asc'">&bigvee;</a>
            <a v-else-if="states_top_order == 'total_pageviews_asc'" @click="states_top_order = 'total_pageviews_desc'">&bigwedge;</a>
            <a v-else @click="states_top_order = 'total_pageviews_desc'">&bigcirc;</a>
          </th>
          <th>Unique Pageviews
            <a v-if="states_top_order == 'unique_pageviews_desc'" @click="states_top_order = 'unique_pageviews_asc'">&bigvee;</a>
            <a v-else-if="states_top_order == 'unique_pageviews_asc'" @click="states_top_order = 'unique_pageviews_desc'">&bigwedge;</a>
            <a v-else @click="states_top_order = 'unique_pageviews_desc'">&bigcirc;</a>
          </th>
          <th>Avg. Time
            <a v-if="states_top_order == 'average_time_desc'" @click="states_top_order = 'average_time_asc'">&bigvee;</a>
            <a v-else-if="states_top_order == 'average_time_asc'" @click="states_top_order = 'average_time_desc'">&bigwedge;</a>
            <a v-else @click="states_top_order = 'average_time_desc'">&bigcirc;</a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in states_top_sorted">
          <td v-if="selected_state === null"><a @click="select_state(row['name'])">{{row['name']}}</a></td>
          <td v-else>{{row['name']}}</td>
          <td><comparison-bar :value="row['Unique Users']" :max="states_max['Unique Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['New Users']" :max="states_max['New Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Returning Users']" :max="states_max['Returning Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Total Pageviews']" :max="states_max['Total Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Unique Pageviews']" :max="states_max['Unique Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Avg. Time']" :max="states_max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>

    <h3>Technology</h3>

    <pie-chart :data="technology_pie" doughnut />

    <table>
      <thead>
        <tr>
          <th>Engagement By Device Type</th>
          <th>Unique Users
            <a v-if="technology_top_order == 'unique_users_desc'" @click="technology_top_order = 'unique_users_asc'">&bigvee;</a>
            <a v-else-if="technology_top_order == 'unique_users_asc'" @click="technology_top_order = 'unique_users_desc'">&bigwedge;</a>
            <a v-else @click="technology_top_order = 'unique_users_desc'">&bigcirc;</a>
          </th>
          <th>New Users
            <a v-if="technology_top_order == 'new_users_desc'" @click="technology_top_order = 'new_users_asc'">&bigvee;</a>
            <a v-else-if="technology_top_order == 'new_users_asc'" @click="technology_top_order = 'new_users_desc'">&bigwedge;</a>
            <a v-else @click="technology_top_order = 'new_users_desc'">&bigcirc;</a>
          </th>
          <th>Returning Users
            <a v-if="technology_top_order == 'returning_users_desc'" @click="technology_top_order = 'returning_users_asc'">&bigvee;</a>
            <a v-else-if="technology_top_order == 'returning_users_asc'" @click="technology_top_order = 'returning_users_desc'">&bigwedge;</a>
            <a v-else @click="technology_top_order = 'returning_users_desc'">&bigcirc;</a>
          </th>
          <th>Total Pageviews
            <a v-if="technology_top_order == 'total_pageviews_desc'" @click="technology_top_order = 'total_pageviews_asc'">&bigvee;</a>
            <a v-else-if="technology_top_order == 'total_pageviews_asc'" @click="technology_top_order = 'total_pageviews_desc'">&bigwedge;</a>
            <a v-else @click="technology_top_order = 'total_pageviews_desc'">&bigcirc;</a>
          </th>
          <th>Unique Pageviews
            <a v-if="technology_top_order == 'unique_pageviews_desc'" @click="technology_top_order = 'unique_pageviews_asc'">&bigvee;</a>
            <a v-else-if="technology_top_order == 'unique_pageviews_asc'" @click="technology_top_order = 'unique_pageviews_desc'">&bigwedge;</a>
            <a v-else @click="technology_top_order = 'unique_pageviews_desc'">&bigcirc;</a>
          </th>
          <th>Avg. Time
            <a v-if="technology_top_order == 'average_time_desc'" @click="technology_top_order = 'average_time_asc'">&bigvee;</a>
            <a v-else-if="technology_top_order == 'average_time_asc'" @click="technology_top_order = 'average_time_desc'">&bigwedge;</a>
            <a v-else @click="technology_top_order = 'average_time_desc'">&bigcirc;</a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in technology_top_sorted">
          <td>{{row['name']}}</td>
          <td><comparison-bar :value="row['Unique Users']" :max="report[org].technology.data.max['Unique Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['New Users']" :max="report[org].technology.data.max['New Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Returning Users']" :max="report[org].technology.data.max['Returning Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Total Pageviews']" :max="report[org].technology.data.max['Total Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Unique Pageviews']" :max="report[org].technology.data.max['Unique Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Avg. Time']" :max="report[org].technology.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>

  <div v-else-if="state=='traffic'">
    <h2>Traffic</h2>
    <!-- !!! -->
  </div>

</div>
</template>

<script>

function cmp(a, b) {
    if(a > b) {
        return 1;
    }
    else if(a < b) {
        return -1;
    }
    else {
        return 0;
    }
}

export default {
    name: "MyDataOverview",

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });
        }

        return {
            report: {
                "Demo Org": {
                    "uid": 'c36bd22f-f530-4469-8c9e-b919951e3486',
                    "updated": "2022-07-28T14:33:27.12343242-07:00",
                    "total_opportunities": 23,
                    "current_opportunities": 18,
                    "engagement": {
                        "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                        "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                        "data": {
                            "opportunity_status": "Live and Closed",
                            "time_period": "This Month",
                            "columns": ["Views" , "Unique", "Clicks to Website"],
                            "totals": {"Views": 36, "Unique": 21, "Clicks to Website": 12},
                            "max": {"Views": 432, "Unique": 234, "Clicks to Website": 210},
                            "chart": [
                                {"date": "2022-07-29", "Views": 15, "Unique": 8, "Clicks to Website": 4},
                                {"date": "2022-07-28", "Views": 8, "Unique": 2, "Clicks to Website": 7},
                                {"date": "2022-07-27", "Views": 13, "Unique": 11, "Clicks to Website": 1},
                            ],
                            "table": [
                                {"name": "Test Opp 1", "slug": "test-opp-1", "Views": 432, "Unique": 234, "Clicks to Website": 119},
                                {"name": "Test Opp 2", "slug": "test-opp-2", "Views": 321, "Unique": 78, "Clicks to Website": 210},
                                {"name": "Test Opp 3", "slug": "test-opp-3", "Views": 210, "Unique": 112, "Clicks to Website": 87},
                                {"name": "Test Opp 4", "slug": "test-opp-4", "Views": 122, "Unique": 34, "Clicks to Website": 12},
                                {"name": "Test Opp 5", "slug": "test-opp-5", "Views": 97, "Unique": 12, "Clicks to Website": 4},
                                {"name": "Test Opp 6", "slug": "test-opp-6", "Views": 15, "Unique": 2, "Clicks to Website": 1},
                            ],
                        },
                    },

                    "states": {
                        "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                        "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                        "data": {
                            "opportunity_status": "Live and Closed",
                            "time_period": "This Month",
                            "max": {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                            "states": {
                                'Texas': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "regional": {
                                    'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                    "regions": {
                                        'Agua Dulce': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-97.910833, 27.7825]},
                                        'Bear Creek': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-97.932778, 30.181944]},
                                        'Blackwell': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-100.319722, 32.085556]},
                                        'Buffalo Springs': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-101.709167, 33.532222]},
                                    },
                                }},
                                'California': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "regional": {
                                    'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                    "regions": {
                                        'Arcata': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-124.090556, 40.868056]},
                                        'Buellton': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-120.193889, 34.614167]},
                                        'Cotati': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-122.709167, 38.327778]},
                                        'Eastvale': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-117.564167, 33.963611]},
                                    },
                                }},
                                'Oregon': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "regional": {
                                    'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                    "regions": {
                                        'Keizer': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-123.021944, 45.000556]},
                                        'Monmouth': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-123.23, 44.849167]},
                                        'Winston': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-123.4175, 43.121667]},
                                        'Nyssa': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-116.996944, 43.879167]},
                                    },
                                }},
                            },
                        },
                    },

                    "technology": {
                        "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                        "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                        "data": {
                            "opportunity_status": "Live and Closed",
                            "time_period": "This Month",
                            'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                            'mobile': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                            'tablet': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                            'desktop': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        },
                    },
                },
            },
        };
    },

    data() {
        return {
            state: 'engagement',
            org: "Demo Org",
            engagement_top_order: 'total_views_desc',
            states_top_order: 'unique_users_desc',
            technology_top_order: 'unique_users_desc',
            selected_state: null,
            selected_attr: "Unique Users",
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },

        available_orgs() {
            return Object.getOwnPropertyNames(this.report).filter(n => !n.startsWith('_'));
        },

        updated_local() {
            return new Date(this.report[this.org].updated).toLocaleDateString();
        },

        states_max() {
            if(this.selected_state != null) {
                return this.report[this.org].states.data.states[this.selected_state].regional.max;
            }
            else {
                return this.report[this.org].states.data.max;
            }
        },

        states_data() {
            if(this.selected_state != null) {
                return this.report[this.org].states.data.states[this.selected_state].regional.regions;
            }
            else {
                return this.report[this.org].states.data.states;
            }
        },

        selected_state_data() {
            if(this.selected_state != null) {
                return this.report[this.org].states.data.states[this.selected_state];
            }
            else {
                return {};
            }            
        },

        states_tabular() {
            let ret = [];

            const states = Object.getOwnPropertyNames(this.states_data);


            for(let state of states) {
                if(state.startsWith("_")) {
                    continue;
                }

                const src = this.states_data[state];

                const val = {
                    "name": state,
                    "Unique Users": src["Unique Users"],
                    "New Users": src["New Users"],
                    "Returning Users": src["Returning Users"],
                    "Total Pageviews": src["Total Pageviews"],
                    "Unique Pageviews": src["Unique Pageviews"],
                    "Avg. Time": src["Avg. Time"],
                };

                ret.push(val);
            }

            return ret;
        },

        technology_tabular() {
            const techs = ["mobile", "tablet", "desktop"]
            let ret = [];

            for(let tech of techs) {
                const src = this.report[this.org].technology.data[tech];

                const val = {
                    "name": tech,
                    "Unique Users": src["Unique Users"],
                    "New Users": src["New Users"],
                    "Returning Users": src["Returning Users"],
                    "Total Pageviews": src["Total Pageviews"],
                    "Unique Pageviews": src["Unique Pageviews"],
                    "Avg. Time": src["Avg. Time"],
                };

                ret.push(val);
            }

            return ret;
        },

        technology_pie() {
            const techs = ["mobile", "tablet", "desktop"];
            const colors = ["#165E6F", "#7CB4BF", "#D6D6D6"];
            const fields = ["Unique Users", "New Users", "Returning Users", "Total Pageviews", "Unique Pageviews", "Avg. Time"];

            let ret = {
                labels: techs,
                datasets: [],
            };

            for(let field of fields) {
                if(field !== "Unique Users") {
                    continue;
                }

                let dataset = {
                    label: field,
                    hoverOffset: 4,
                    backgroundColor: colors,
                    data: [],
                };

                for(let tech of techs) {
                    const src = this.report[this.org].technology.data[tech];
                    dataset.data.push(src[field]);
                }

                ret.datasets.push(dataset);
            }

            return ret;
        },

        engagement_top_sorted() {
            switch(this.engagement_top_order) {
                case "total_views_desc":
                return this.report[this.org].engagement.data.table.slice().sort((a, b) => -cmp(a['Views'], b['Views']));
                case "total_views_asc":
                return this.report[this.org].engagement.data.table.slice().sort((a, b) => cmp(a['Views'], b['Views']));
                case "unique_views_desc":
                return this.report[this.org].engagement.data.table.slice().sort((a, b) => -cmp(a['Unique'], b['Unique']));
                case "unique_views_asc":
                return this.report[this.org].engagement.data.table.slice().sort((a, b) => cmp(a['Unique'], b['Unique']));
                case "clicks_desc":
                return this.report[this.org].engagement.data.table.slice().sort((a, b) => -cmp(a['Clicks to Website'], b['Clicks to Website']));
                case "clicks_asc":
                return this.report[this.org].engagement.data.table.slice().sort((a, b) => cmp(a['Clicks to Website'], b['Clicks to Website']));
                default:
                return this.report[this.org].engagement.data.table;
            }
        },

        states_top_sorted() {
            switch(this.states_top_order) {
                case "unique_users_asc":
                return this.states_tabular.slice().sort((a, b) => cmp(a['Unique Users'], b['Unique Users']));
                case "unique_users_desc":
                return this.states_tabular.slice().sort((a, b) => -cmp(a['Unique Users'], b['Unique Users']));
                case "new_users_asc":
                return this.states_tabular.slice().sort((a, b) => cmp(a['New Users'], b['New Users']));
                case "new_users_desc":
                return this.states_tabular.slice().sort((a, b) => -cmp(a['New Users'], b['New Users']));
                case "returning_users_asc":
                return this.states_tabular.slice().sort((a, b) => cmp(a['Returning Users'], b['Returning Users']));
                case "returning_users_desc":
                return this.states_tabular.slice().sort((a, b) => -cmp(a['Returning Users'], b['Returning Users']));
                case "total_pageviews_asc":
                return this.states_tabular.slice().sort((a, b) => cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "total_pageviews_desc":
                return this.states_tabular.slice().sort((a, b) => -cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "unique_pageviews_asc":
                return this.states_tabular.slice().sort((a, b) => cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "unique_pageviews_desc":
                return this.states_tabular.slice().sort((a, b) => -cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "average_time_asc":
                return this.states_tabular.slice().sort((a, b) => cmp(a['Avg. Time'], b['Avg. Time']));
                case "average_time_desc":
                return this.states_tabular.slice().sort((a, b) => -cmp(a['Avg. Time'], b['Avg. Time']));
                default:
                return this.states_tabular;
            }
        },

        technology_top_sorted() {
            switch(this.technology_top_order) {
                case "unique_users_asc":
                return this.technology_tabular.slice().sort((a, b) => cmp(a['Unique Users'], b['Unique Users']));
                case "unique_users_desc":
                return this.technology_tabular.slice().sort((a, b) => -cmp(a['Unique Users'], b['Unique Users']));
                case "new_users_asc":
                return this.technology_tabular.slice().sort((a, b) => cmp(a['New Users'], b['New Users']));
                case "new_users_desc":
                return this.technology_tabular.slice().sort((a, b) => -cmp(a['New Users'], b['New Users']));
                case "returning_users_asc":
                return this.technology_tabular.slice().sort((a, b) => cmp(a['Returning Users'], b['Returning Users']));
                case "returning_users_desc":
                return this.technology_tabular.slice().sort((a, b) => -cmp(a['Returning Users'], b['Returning Users']));
                case "total_pageviews_asc":
                return this.technology_tabular.slice().sort((a, b) => cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "total_pageviews_desc":
                return this.technology_tabular.slice().sort((a, b) => -cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "unique_pageviews_asc":
                return this.technology_tabular.slice().sort((a, b) => cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "unique_pageviews_desc":
                return this.technology_tabular.slice().sort((a, b) => -cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "average_time_asc":
                return this.technology_tabular.slice().sort((a, b) => cmp(a['Avg. Time'], b['Avg. Time']));
                case "average_time_desc":
                return this.technology_tabular.slice().sort((a, b) => -cmp(a['Avg. Time'], b['Avg. Time']));
                default:
                return this.technology_tabular;
            }
        },
    },

    methods: {
        log(msg) {
            console.log(msg);
        },

        select_state(state) {
            console.log("selected", state);
            this.selected_state = state;
        },

        save_engagement_csv() {
            let structured = this.report[this.org].engagement.data.chart;

            if(!structured || !structured.length || structured.length <= 0) {
                this.$buefy.dialog.alert({
                    title: 'Error',
                    message: "The selected data set is empty, there's nothing to download.",
                    type: 'is-danger',
                    hasIcon: true,
                    icon: 'times-circle',
                    iconPack: 'fa',
                    ariaRole: 'alertdialog',
                    ariaModal: true
                });

                return;
            }

            this.$save_table_csv(
                this.org,
                ['date', ...this.report[this.org].engagement.data.columns],
                structured
            );
        },
    },
}
</script>

<style lang="scss" scoped>
h1 {
    margin: 1rem 0px;
    text-align: left;
    font: normal normal bold 24px/28px Fira Sans;
    letter-spacing: 0px;
    color: #1A1A1A;
    text-transform: uppercase;
}

h2 {
    margin: 0.75rem 0px;
    text-align: left;
    font: normal normal bold 24px/28px Fira Sans;
    letter-spacing: 0px;
    color: #1A1A1A;
}

h3 {
    margin: 05rem 0px;
    text-align: left;
    font: normal normal bold 16px/19px Roboto;
    letter-spacing: 0px;
    color: #2F2F2F;
}

th,td {
    padding-right: 1rem;
}

aside {
    display: block;
    text-align: right;
    color: #ccc;
}

.notification {
    background-color: #FFF2D6;
    border: 1px solid #FABF40;
    box-shadow: 0px 3px 6px #FAFAFA;
    border-radius: 6px;
    padding: 1rem;

    >label {
        display: block;
        text-align: left;
        font: normal normal bold 16px/19px Roboto;
        letter-spacing: 0px;
        color: #1D1D1D;
    }
}

.stack {
    display: flex;
    flex-direction: column;
}

.labeled-gauge-ends {
    display: flex;
    flex-direction: column;
    width: 350px;
    height: 275px;

    >.labels {
        display: flex;
        text-align: center;
        justify-content: space-between;

        label {
            font: normal normal bold 72px/86px Fira Sans;
            letter-spacing: 0px;
        }

        small {
            font: normal normal bold 17px/19px Roboto;
            letter-spacing: 0px;
        }

        >:first-child {
            color: #165E6F;
        }

        >:last-child {
            color: #5694A2;
        }
    }
}

.filters {
    display: flex;
    align-items: center;
    justify-content: flex-start;

    label {
        text-align: left;
        font: normal normal bold 16px/19px Roboto;
        letter-spacing: 0px;
        color: #2F2F2F;
    }

    >* {
        margin-right: 2rem;
    }

    >.extra {
        margin-left: auto;
    }
}

#current-proportion {
    margin: 1rem auto;
}
</style>
