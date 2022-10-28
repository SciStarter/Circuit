<template>
<div class="your-data-overview snm-container">
  <div class="flex-header">
    <h1>Opportunity Data Explorer</h1>
  </div>

  <div class="filters">
    <div class="stack">
      <label>Opportunity</label>
      <b-select :value="current_opp" @input="log('TBD download from server')">
        <option v-for="opp in opps" :key="opp.id" :value="opp">
          {{opp.title}}
        </option>
      </b-select>
    </div>
    <div class="stack">
      <label>Time Period</label>
      <b-select :value="report.engagement.data.time_period" @input="log('TBD download from server')">
        <option v-for="period in report.engagement.time_periods" :key="period" :value="period">
          {{period}}
        </option>
      </b-select>
    </div>
  </div>

  <div class="nav-tab-wrapper">
    <ul class="nav-tabs">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'">Engagement</a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'">Audience</a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'">Traffic</a></li>
      <li><a class="tab-link" :class="{'active':state=='overlap'}" @click="state='overlap'">Engagement Overlap</a></li>
    </ul>
  </div>

  <aside>Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">
    <h2>{{current_opp.title}}</h2>

    <div>{{ report.engagement.data.bars.self["Views"] }} Page Views</div>

    <line-chart
      :rows="report.engagement.data.chart"
      :xaxis="d => new Date(d.date)"
      :yaxes="['Views', 'Unique']"
      :colors="['#268699', '#BFDCE2']"
      />

    <div>{{ report.engagement.data.bars.self["Clicks to Website"] }} Clicks To Your Website</div>

    <strong>Conversion Rates</strong>
    <div class="conversion-rate">
      <comparison-bar :value="report.engagement.data.bars.self['Views'] > 0 ? report.engagement.data.bars.self['Clicks to Website'] / report.engagement.data.bars.self['Views'] : 0" :max="1.0" color="#165E6F" background="#DEDEDE" width="100%" height="2rem" />
    </div>
    <div class="conversion-rate">
      <comparison-bar :value="report.engagement.data.bars.median['Views'] > 0 ? report.engagement.data.bars.median['Clicks to Website'] / report.engagement.data.bars.median['Views'] : 0" :max="1.0" color="#7CB4BF" background="#DEDEDE" width="100%" height="2rem" />
    </div>
    <div class="conversion-rate">
      <comparison-bar :value="report.engagement.data.bars.mean['Views'] > 0 ? report.engagement.data.bars.mean['Clicks to Website'] / report.engagement.data.bars.mean['Views'] : 0" :max="1.0" color="#7CB4BF" background="#DEDEDE" width="100%" height="2rem" />
    </div>
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
        <b-select :value="report.states.data.opportunity_status" @input="log('TBD download from server')">
          <option v-for="status in report.states.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report.states.data.time_period" @input="log('TBD download from server')">
          <option v-for="period in report.states.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
    </div>

    <choropleth-states v-if="selected_state === null" :value="report.states.data.states" attr="Unique Users" @state="select_state($event)"/>
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
          <td><comparison-bar :value="row['Unique Users']" :max="report.technology.data.max['Unique Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['New Users']" :max="report.technology.data.max['New Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Returning Users']" :max="report.technology.data.max['Returning Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Total Pageviews']" :max="report.technology.data.max['Total Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Unique Pageviews']" :max="report.technology.data.max['Unique Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Avg. Time']" :max="report.technology.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>

  <div v-else-if="state=='traffic'">
    <h2>Traffic</h2>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report.engagement.data.opportunity_status" @input="log('TBD download from server')">
          <option v-for="status in report.engagement.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report.engagement.data.time_period" @input="log('TBD download from server')">
          <option v-for="period in report.engagement.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
      <div class="extra">
        <a @click="save_traffic_csv">export csv</a>
      </div>
    </div>

    <line-chart
      :rows="report.traffic.data.chart"
      :xaxis="d => new Date(d.date)"
      :yaxes="['Unique', 'New', 'Returning']"
      :colors="['#268699', '#868686', '#BFDCE2']"
      />

    <pie-chart :data="report.traffic.data.pie" />

    <table>
      <thead>
        <tr>
          <th>Top Referrers</th>
          <th>Unique Users
            <a v-if="traffic_top_order == 'unique_users_desc'" @click="traffic_top_order = 'unique_users_asc'">&bigvee;</a>
            <a v-else-if="traffic_top_order == 'unique_users_asc'" @click="traffic_top_order = 'unique_users_desc'">&bigwedge;</a>
            <a v-else @click="traffic_top_order = 'unique_users_desc'">&bigcirc;</a>
          </th>
          <th>New Users
            <a v-if="traffic_top_order == 'new_users_desc'" @click="traffic_top_order = 'new_users_asc'">&bigvee;</a>
            <a v-else-if="traffic_top_order == 'new_users_asc'" @click="traffic_top_order = 'new_users_desc'">&bigwedge;</a>
            <a v-else @click="traffic_top_order = 'new_users_desc'">&bigcirc;</a>
          </th>
          <th>Returning Users
            <a v-if="traffic_top_order == 'returning_users_desc'" @click="traffic_top_order = 'returning_users_asc'">&bigvee;</a>
            <a v-else-if="traffic_top_order == 'returning_users_asc'" @click="traffic_top_order = 'returning_users_desc'">&bigwedge;</a>
            <a v-else @click="traffic_top_order = 'returning_users_desc'">&bigcirc;</a>
          </th>
          <th>Total Pageviews
            <a v-if="traffic_top_order == 'total_pageviews_desc'" @click="traffic_top_order = 'total_pageviews_asc'">&bigvee;</a>
            <a v-else-if="traffic_top_order == 'total_pageviews_asc'" @click="traffic_top_order = 'total_pageviews_desc'">&bigwedge;</a>
            <a v-else @click="traffic_top_order = 'total_pageviews_desc'">&bigcirc;</a>
          </th>
          <th>Unique Pageviews
            <a v-if="traffic_top_order == 'unique_pageviews_desc'" @click="traffic_top_order = 'unique_pageviews_asc'">&bigvee;</a>
            <a v-else-if="traffic_top_order == 'unique_pageviews_asc'" @click="traffic_top_order = 'unique_pageviews_desc'">&bigwedge;</a>
            <a v-else @click="traffic_top_order = 'unique_pageviews_desc'">&bigcirc;</a>
          </th>
          <th>Avg. Time
            <a v-if="traffic_top_order == 'average_time_desc'" @click="traffic_top_order = 'average_time_asc'">&bigvee;</a>
            <a v-else-if="traffic_top_order == 'average_time_asc'" @click="traffic_top_order = 'average_time_desc'">&bigwedge;</a>
            <a v-else @click="traffic_top_order = 'average_time_desc'">&bigcirc;</a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in traffic_top_sorted">
          <td>{{row['name']}}</td>
          <td><comparison-bar :value="row['Unique Users']" :max="report.traffic.data.max['Unique Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['New Users']" :max="report.traffic.data.max['New Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Returning Users']" :max="report.traffic.data.max['Returning Users']" color="#268699" /></td>
          <td><comparison-bar :value="row['Total Pageviews']" :max="report.traffic.data.max['Total Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Unique Pageviews']" :max="report.traffic.data.max['Unique Pageviews']" color="#268699" /></td>
          <td><comparison-bar :value="row['Avg. Time']" :max="report.traffic.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>
  <div v-else-if="state=='overlap'">
    <h2>Engagement Overlap</h2>
    <div>
      <b-select :value="report.overlap.data.engagement_type" @input="log('TBD download from server')">
        <option v-for="e_type in report.overlap.engagement_types" :key="e_type" :value="e_type">
          {{e_type}}
        </option>
      </b-select>
    </div>

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
    name: "MyOpportunityDataExplorer",

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
        const opps = await context.$axios.$get('/api/ui/finder/search?mine=true&sort=alphabetical&per_page=4294967295&refs=true', context.store.state.auth);

        if(!user.authenticated) {
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });
        }

        return {
            opps: opps.matches,
            current_opp: opps.matches.length ? opps.matches[0] : {},
            report: {
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
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "columns": ["Views" , "Unique", "Clicks to Website"],
                        "chart": [
                            {"date": "2022-07-29", "Views": 15, "Unique": 8, "Clicks to Website": 4},
                            {"date": "2022-07-28", "Views": 8, "Unique": 2, "Clicks to Website": 7},
                            {"date": "2022-07-27", "Views": 13, "Unique": 11, "Clicks to Website": 1},
                        ],
                        "bars": {
                            "self": {"Views": 432, "Unique": 234, "Clicks to Website": 119},
                            "mean": {"Views": 321, "Unique": 78, "Clicks to Website": 210},
                            "median": {"Views": 210, "Unique": 112, "Clicks to Website": 87},
                        },
                    },
                },

                "states": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
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
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        'mobile': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                        'tablet': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                        'desktop': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                    },
                },

                "traffic": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "columns": ["Unique", "New", "Returning"],
                        "max": {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        "chart": [
                            {"date": "2022-07-29", "Unique": 15, "New": 8, "Returning": 4},
                            {"date": "2022-07-28", "Unique": 8, "New": 2, "Returning": 7},
                            {"date": "2022-07-27", "Unique": 13, "New": 11, "Returning": 1},
                        ],
                        "pie": {
                            "labels": ["Direct", "Payed Search", "Display", "Affiliates", "Other"],
                            "datasets": [{
                                "label": "Referrers by Type",
                                "hoverOffset": 4,
                                "backgroundColor": ["#387ab5", "#5da136", "#cd4c24", "#e7e93c", "#5abdda"],
                                "data": [202, 15, 11, 0, 0],
                            }],
                        },
                        "table": [
                            {"name": "Test Ref 1", "type": "Direct", "Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                            {"name": "Test Ref 2", "type": "Direct", "Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                            {"name": "Test Ref 3", "type": "Direct", "Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                            {"name": "Test Ref 4", "type": "Paid Search", "Unique Users": 3, "New Users": 34, "Returning Users": 32, "Total Pageviews": 23, "Unique Pageviews": 22, "Avg. Time": 32},
                            {"name": "Test Ref 5", "type": "Paid Search", "Unique Users": 12, "New Users": 14, "Returning Users": 32, "Total Pageviews": 23, "Unique Pageviews": 32, "Avg. Time": 12},
                            {"name": "Test Ref 6", "type": "Display", "Unique Users": 11, "New Users": 13, "Returning Users": 33, "Total Pageviews": 22, "Unique Pageviews": 32, "Avg. Time": 13},
                        ],
                    },
                },

                "overlap": {
                    "engagement_types": ["Views", "Unique", "Clicks to Website"],
                    "data": {
                        "engagement_type": "Views",
                        "table": [
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Jellicle Dogs", "activity_types": ["science_slam", "service"], "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                        ]
                    },
                },
            },
        };
    },

    data() {
        return {
            state: 'engagement',
            org: "Demo Org",
            states_top_order: 'unique_users_desc',
            technology_top_order: 'unique_users_desc',
            traffic_top_order: 'unique_users_desc',
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
            return new Date(this.report.updated).toLocaleDateString();
        },

        states_max() {
            if(this.selected_state != null) {
                return this.report.states.data.states[this.selected_state].regional.max;
            }
            else {
                return this.report.states.data.max;
            }
        },

        states_data() {
            if(this.selected_state != null) {
                return this.report.states.data.states[this.selected_state].regional.regions;
            }
            else {
                return this.report.states.data.states;
            }
        },

        selected_state_data() {
            if(this.selected_state != null) {
                return this.report.states.data.states[this.selected_state];
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
                const src = this.report.technology.data[tech];

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
                    const src = this.report.technology.data[tech];
                    dataset.data.push(src[field]);
                }

                ret.datasets.push(dataset);
            }

            return ret;
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

        traffic_top_sorted() {
            switch(this.traffic_top_order) {
                case "unique_users_asc":
                return this.report.traffic.data.table.slice().sort((a, b) => cmp(a['Unique Users'], b['Unique Users']));
                case "unique_users_desc":
                return this.report.traffic.data.table.slice().sort((a, b) => -cmp(a['Unique Users'], b['Unique Users']));
                case "new_users_asc":
                return this.report.traffic.data.table.slice().sort((a, b) => cmp(a['New Users'], b['New Users']));
                case "new_users_desc":
                return this.report.traffic.data.table.slice().sort((a, b) => -cmp(a['New Users'], b['New Users']));
                case "returning_users_asc":
                return this.report.traffic.data.table.slice().sort((a, b) => cmp(a['Returning Users'], b['Returning Users']));
                case "returning_users_desc":
                return this.report.traffic.data.table.slice().sort((a, b) => -cmp(a['Returning Users'], b['Returning Users']));
                case "total_pageviews_asc":
                return this.report.traffic.data.table.slice().sort((a, b) => cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "total_pageviews_desc":
                return this.report.traffic.data.table.slice().sort((a, b) => -cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "unique_pageviews_asc":
                return this.report.traffic.data.table.slice().sort((a, b) => cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "unique_pageviews_desc":
                return this.report.traffic.data.table.slice().sort((a, b) => -cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "average_time_asc":
                return this.report.traffic.data.table.slice().sort((a, b) => cmp(a['Avg. Time'], b['Avg. Time']));
                case "average_time_desc":
                return this.report.traffic.data.table.slice().sort((a, b) => -cmp(a['Avg. Time'], b['Avg. Time']));
                default:
                return this.report.traffic.data.table;
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
            let structured = this.report.engagement.data.chart;

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
                this.org + ' engagement ' + this.report.engagement.data.begin + ' - ' + this.report.engagement.data.end,
                ['date', ...this.report.engagement.data.columns],
                structured
            );
        },

        save_traffic_csv() {
            let structured = this.report.traffic.data.chart;

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
                this.org + ' traffic ' + this.report.traffic.data.begin + ' - ' + this.report.traffic.data.end,
                ['date', ...this.report.traffic.data.columns],
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
    color: #165E6F;
    font-weight: bold;
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

.conversion-rate {
    margin: 1rem;
}

#current-proportion {
    margin: 1rem auto;
}
</style>
