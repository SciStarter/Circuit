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
    <ul class="nav-tabs nav-tabs-alt">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'"><span>Engagement</span><small>How users interact with your opportunity pages</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'"><span>Audience</span><small>Demographics, locations and technology of your users</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'"><span>Traffic</span><small>How users get to your pages</small></a></li>
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

    <div class="data-wrapper">
      <div class="data-header">
        <div class="big-legend bl-blue">
          <div class="ll-icon"><eye-icon></eye-icon></div>
          <div>
            <h2>{{report[org].engagement.data.totals['Views']}}</h2>
            <h3>Page Views</h3>
            <div class="ll-legend">
              <div><span class="dark-blue"></span> Total</div>
              <div><span class="light-blue"></span> Unique</div> 
            </div>
          </div>
        </div>
        <div class="big-legend bl-yellow">
          <div class="ll-icon"><link-icon></link-icon></div>
          <div>
            <h2>{{report[org].engagement.data.totals['Clicks to Website']}}</h2>
            <h3>Clicks to Website</h3>
        </div>
        </div>
      </div>
      <line-chart
      :rows="report[org].engagement.data.chart"
      :xaxis="d => new Date(d.date)"
      :yaxes="['Views', 'Unique', 'Clicks to Website']"
      :colors="['#268699', '#BFDCE2', '#FABF40']"
      />
    </div>

    

    <table class="data-table">
      <thead>
        <tr>
          <th>Top Performing Opportunities</th>
          <th colspan="2">Total Views
            <a v-if="engagement_top_order == 'total_views_desc'" @click="engagement_top_order = 'total_views_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="engagement_top_order == 'total_views_asc'" @click="engagement_top_order = 'total_views_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="engagement_top_order = 'total_views_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Clicks to Website
            <a v-if="engagement_top_order == 'clicks_desc'" @click="engagement_top_order = 'clicks_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="engagement_top_order == 'clicks_asc'" @click="engagement_top_order = 'clicks_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="engagement_top_order = 'clicks_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in engagement_top_sorted">
          <td><a :href="row['slug']">{{row['name']}}</a></td>
          <td class="table-num">{{row['Views']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Views']" :max="report[org].engagement.data.max['Views']" color="#268699" /></td>
          <td class="table-num">{{row['Clicks to Website']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Clicks to Website']" :max="report[org].engagement.data.max['Clicks to Website']" color="#FABF40" /></td>
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

    <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          <th>Engagement By Location</th>
          <th colspan="2">Unique Users
            <a v-if="states_top_order == 'unique_users_desc'" @click="states_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'unique_users_asc'" @click="states_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">New Users
            <a v-if="states_top_order == 'new_users_desc'" @click="states_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'new_users_asc'" @click="states_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Returning Users
            <a v-if="states_top_order == 'returning_users_desc'" @click="states_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'returning_users_asc'" @click="states_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Total Pageviews
            <a v-if="states_top_order == 'total_pageviews_desc'" @click="states_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'total_pageviews_asc'" @click="states_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Unique Pageviews
            <a v-if="states_top_order == 'unique_pageviews_desc'" @click="states_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'unique_pageviews_asc'" @click="states_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Avg. Time
            <a v-if="states_top_order == 'average_time_desc'" @click="states_top_order = 'average_time_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'average_time_asc'" @click="states_top_order = 'average_time_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'average_time_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in states_top_sorted">
          <td v-if="selected_state === null"><a @click="select_state(row['name'])">{{row['name']}}</a></td>
          <td v-else>{{row['name']}}</td>
          <td class="table-num">{{row['Unique Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="states_max['Unique Users']" color="#268699" /></td>
          <td class="table-num">{{row['New Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['New Users']" :max="states_max['New Users']" color="#268699" /></td>
          <td class="table-num">{{row['Returning Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="states_max['Returning Users']" color="#268699" /></td>
          <td class="table-num">{{row['Total Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="states_max['Total Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Unique Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="states_max['Unique Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Avg. Time']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="states_max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>

    <div class="data-wrapper">

        <div class="data-head">
          <h3>Technology</h3>
        </div>
    

    <pie-chart :data="technology_pie" doughnut />

    <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          <th>Engagement By Device Type</th>
          <th colspan="2">Unique Users
            <a v-if="technology_top_order == 'unique_users_desc'" @click="technology_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'unique_users_asc'" @click="technology_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">New Users
            <a v-if="technology_top_order == 'new_users_desc'" @click="technology_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'new_users_asc'" @click="technology_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Returning Users
            <a v-if="technology_top_order == 'returning_users_desc'" @click="technology_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'returning_users_asc'" @click="technology_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Total Pageviews
            <a v-if="technology_top_order == 'total_pageviews_desc'" @click="technology_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'total_pageviews_asc'" @click="technology_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Unique Pageviews
            <a v-if="technology_top_order == 'unique_pageviews_desc'" @click="technology_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'unique_pageviews_asc'" @click="technology_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Avg. Time
            <a v-if="technology_top_order == 'average_time_desc'" @click="technology_top_order = 'average_time_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'average_time_asc'" @click="technology_top_order = 'average_time_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'average_time_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in technology_top_sorted">
          <td>{{row['name']}}</td>
          <td class="table-num">{{row['Unique Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report[org].technology.data.max['Unique Users']" color="#268699" /></td>
          <td class="table-num">{{row['New Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report[org].technology.data.max['New Users']" color="#268699" /></td>
          <td class="table-num">{{row['Returning Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report[org].technology.data.max['Returning Users']" color="#268699" /></td>
          <td class="table-num">{{row['Total Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report[org].technology.data.max['Total Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Unique Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report[org].technology.data.max['Unique Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Avg. Time']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report[org].technology.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>
  </div>
  </div>


  <div v-else-if="state=='traffic'">
    <h2>Traffic</h2>

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
        <a @click="save_traffic_csv">export csv</a>
      </div>
    </div>

    <div class="data-wrapper">
    <div class="data-header">
        <div class="big-legend bl-blue">
          <div class="ll-icon"><eye-icon></eye-icon></div>
          <div>
            <h2># (Total Unique Users during time period)</h2>
            <h3>Users</h3>
            
          </div>
        </div>
        <div class="ll-legend">
              <div><span class="dark-blue"></span> Unique</div>
              <div><span class="gray"></span> New</div> 
              <div><span class="light-blue"></span> Returning</div> 
            </div>
        </div>
    <line-chart
      :rows="report[org].traffic.data.chart"
      :xaxis="d => new Date(d.date)"
      :yaxes="['Unique', 'New', 'Returning']"
      :colors="['#268699', '#868686', '#BFDCE2']"
      />
    </div>


    <div class="data-wrapper">
      <div class="data-head">
          <h3>Referral Sources</h3>
        </div>
    <pie-chart :data="report[org].traffic.data.pie" />

    <table class="data-table">
      <thead>
        <tr>
          <th>Top Referrers</th>
          <th colspan="2">Unique Users
            <a v-if="traffic_top_order == 'unique_users_desc'" @click="traffic_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'unique_users_asc'" @click="traffic_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">New Users
            <a v-if="traffic_top_order == 'new_users_desc'" @click="traffic_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'new_users_asc'" @click="traffic_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Returning Users
            <a v-if="traffic_top_order == 'returning_users_desc'" @click="traffic_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'returning_users_asc'" @click="traffic_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Total Pageviews
            <a v-if="traffic_top_order == 'total_pageviews_desc'" @click="traffic_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'total_pageviews_asc'" @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Unique Pageviews
            <a v-if="traffic_top_order == 'unique_pageviews_desc'" @click="traffic_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'unique_pageviews_asc'" @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Avg. Time
            <a v-if="traffic_top_order == 'average_time_desc'" @click="traffic_top_order = 'average_time_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'average_time_asc'" @click="traffic_top_order = 'average_time_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'average_time_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in traffic_top_sorted">
          <td>{{row['name']}}</td>
          <td class="table-num">{{row['Unique Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report[org].traffic.data.max['Unique Users']" color="#268699" /></td>
          <td class="table-num">{{row['New Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report[org].traffic.data.max['New Users']" color="#268699" /></td>
          <td class="table-num">{{row['Returning Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report[org].traffic.data.max['Returning Users']" color="#268699" /></td>
          <td class="table-num">{{row['Total Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report[org].traffic.data.max['Total Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Unique Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report[org].traffic.data.max['Unique Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Avg. Time']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report[org].traffic.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>
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

import EyeIcon from '~/assets/img/eye.svg?inline'
import LinkIcon from '~/assets/img/link.svg?inline'
import SortIcon from '~/assets/img/sort.svg?inline'
import SortableIcon from '~/assets/img/sortable.svg?inline'

export default {
    name: "MyDataOverview",

    components: {
      EyeIcon,
      LinkIcon,
      SortIcon,
      SortableIcon
    },

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
                            "begin": "2022-07-27",
                            "end": "2022-07-29",
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

        traffic_top_sorted() {
            switch(this.traffic_top_order) {
                case "unique_users_asc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => cmp(a['Unique Users'], b['Unique Users']));
                case "unique_users_desc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => -cmp(a['Unique Users'], b['Unique Users']));
                case "new_users_asc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => cmp(a['New Users'], b['New Users']));
                case "new_users_desc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => -cmp(a['New Users'], b['New Users']));
                case "returning_users_asc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => cmp(a['Returning Users'], b['Returning Users']));
                case "returning_users_desc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => -cmp(a['Returning Users'], b['Returning Users']));
                case "total_pageviews_asc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "total_pageviews_desc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => -cmp(a['Total Pageviews'], b['Total Pageviews']));
                case "unique_pageviews_asc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "unique_pageviews_desc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => -cmp(a['Unique Pageviews'], b['Unique Pageviews']));
                case "average_time_asc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => cmp(a['Avg. Time'], b['Avg. Time']));
                case "average_time_desc":
                return this.report[this.org].traffic.data.table.slice().sort((a, b) => -cmp(a['Avg. Time'], b['Avg. Time']));
                default:
                return this.report[this.org].traffic.data.table;
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
                this.org + ' engagement ' + this.report[this.org].engagement.data.begin + ' - ' + this.report[this.org].engagement.data.end,
                ['date', ...this.report[this.org].engagement.data.columns],
                structured
            );
        },

        save_traffic_csv() {
            let structured = this.report[this.org].traffic.data.chart;

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
                this.org + ' traffic ' + this.report[this.org].traffic.data.begin + ' - ' + this.report[this.org].traffic.data.end,
                ['date', ...this.report[this.org].traffic.data.columns],
                structured
            );
        },
    },
}
</script>

<style lang="scss" scoped>

$linework : #dee2e6;
$lightblue: #BFDCE2;

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
    margin-bottom: 2rem;
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
    margin-bottom: 1rem;

    label {
        text-align: left;
        font: normal normal bold 16px/19px Roboto;
        letter-spacing: 0px;
        color: #2F2F2F;
        margin-bottom: .5rem;
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
    margin-bottom: 4rem;
}

.data-wrapper, .data-table {
  border: 1px solid $linework;
  margin-bottom: 2rem;
}

.data-wrapper .data-table {
  margin-bottom: 0;
  border-left: 0;
  border-right: 0;
  border-bottom: 0;
  tr:last-child {
    border-bottom: 0;
  }
}
.data-header {
  padding:1rem 1rem 0;
  display: flex;
  justify-content:space-between;
}

.big-legend {
  display:flex;
  h2 {
    margin:0;
    margin-bottom: 0;
    color: $snm-color-element-med;
  }
  h3 {
    margin:0;
    color: $snm-color-element-med;
  }
  &.bl-yellow {
    h2, h3 {
      color: $snm-color-action;
    }
  }
}

.ll-legend {
  display: flex;
  margin-top: 6px;
  > div {
    margin-right: 16px;
  }
  > :last-child {
    margin-right: 0;
  }
  span {
    display: inline-block;
    width: 16px;
    height:16px;
    border-radius: 100%;
    background-color: #165E6F;
    position: relative;
    bottom: -2px;
    &.light-blue {
      background-color: $lightblue;
    }
    &.gray {
      background-color: #868686;
    }
  }
}

.ll-icon {
  border-radius: 100%;
  height:45px;
  width: 45px;
  background-color: $lightblue;
  display: flex;
    justify-content: center;
    align-items: center;
    margin-right: 10px;
}
.ll-icon svg  * {
  fill: $snm-color-element-med;
}

.bl-yellow {
  .ll-icon {
    background-color: $snm-color-action;
  }
  .ll-icon svg * {
    fill:white;
  }
}

.data-table {
  width: 100%;

  thead {
    border-bottom: 1px solid $linework;
    box-shadow: 0 4px 4px rgba(0,0,0,.05);
  }

  td,th {
    padding: 3px 10px;
  }
  th {
    padding: 10px;
    font-weight: normal;
    &:first-child {
      font-weight: bold;
    }
  }

  tr {
    border-bottom: 1px solid $linework;
  }
  td,th {
    border-left: 1px solid $linework;
    vertical-align: middle;
    &:first-child {
      border-left: 0;
    }
  }
  .table-num {
    text-align: right;
    width: 50px;
    padding-right: 8px;
  }

  .table-bar {
    border-left: 0;
    padding-left: 0;
  }

  td > div {
    display: flex;
    align-items: center;
    :first-child {
      margin-right: 8px;
    }
  }

}

.data-head {
  padding:1rem;
  border-bottom: 1px solid $linework;
  box-shadow: 0 4px 4px rgba(0,0,0,.05);

  h3 {
    margin:0;
  }
}


.sort {
  position: relative;
  top:4px;
}

.sortable.sort {
  top:1px;
}
.sortable.sort svg {
  width: 12px;
  height: 12px;
  * {
    fill: #d5d5d5;
  }
}

.sort svg {
  width: 16px;
  height: 16px;
  * {
    fill : $snm-color-element-med;
  }
}

.sort-desc svg {
  transform:rotate(180deg);
}


</style>
