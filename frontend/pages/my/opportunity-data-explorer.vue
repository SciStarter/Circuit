<template>
<div class="your-data-overview">
  <div class="flex-header">
    <h1>Opportunity Data Explorer</h1>
  </div>

  <div class="filters">
    <div class="stack">
      <label>Opportunity</label>
      <b-select :value="current_opp" @input="load_opp($event)">
        <option v-for="opp in opps" :key="opp.id" :value="opp">
          {{opp.title.slice(0, 64)}}
        </option>
      </b-select>
    </div>
  </div>

  <div class="nav-mobile-wrapper">
  <div class="nav-tab-wrapper">
    <ul class="nav-tabs nav-tabs-alt">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'"><span>Engagement</span><small>How users interact with your opportunity</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'"><span>Audience</span><small>Demographics, locations and technology of your users</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'"><span>Traffic</span><small>How users get to your opportunity</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='overlap'}" @click="state='overlap'"><span>Engagement Overlap</span><small>See how your users interact with other opportunities</small></a></li>
    </ul>
  </div>
</div>

  <aside class="data-update">Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">


    <h2>{{current_opp.title}}</h2>
    <div class="filters">
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report.engagement.data.time_period" @input="load_data_into(current_opp.uid, 0, $event, 'Live and Closed', 'engagement')">
          <option v-for="period in report.engagement.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
    </div>

    <div class="data-wrapper">
      <div class="data-header">
      <div class="big-legend bl-blue">
          <div class="ll-icon"><eye-icon></eye-icon></div>
          <div>
            <h2>{{ report.engagement.data.bars.self["Views"] }}</h2>
            <h3>Page Views</h3>
          </div>

      </div>
      <div class="ll-legend">
              <div><span class="dark-blue"></span> Total</div>
              <div><span class="light-blue"></span> Unique</div>
            </div>
        </div>
      <client-only>
        <line-chart
          :rows="report.engagement.data.chart"
          :xaxis="d => new Date(d.date)"
          :yaxes="['Views', 'Unique']"
          :colors="['#268699', '#BFDCE2']"
          />
      </client-only>
    </div>

    <!-- Disabled until the computation catches up and we can display accurate data -->
  <!--   <div class="data-wrapper"> -->
  <!--     <div class="data-header"> -->
  <!--       <div class="big-legend bl-yellow"> -->
  <!--         <div class="ll-icon"><link-icon></link-icon></div> -->
  <!--         <div> -->
  <!--           <h2>{{ report.engagement.data.bars.self["Clicks to Website"] }}</h2> -->
  <!--           <h3>Clicks To Your Website</h3> -->
  <!--         </div> -->
  <!--       </div> -->
  <!--     </div> -->
      
  <!--     <div class="data-header2"> -->
  <!--       <h4>Conversion Rates</h4><small>based on total page views</small> -->
  <!--     </div> -->
  <!--     <div class="conversion-rate"> -->
  <!--       <h5>Your Conversion Rate</h5> -->
  <!--       <div class="flex align-center"> -->
  <!--         <strong>{{report.engagement.data.bars.self['Clicks to Website']}}</strong> clicks to the opportunity web site -->
  <!--         <comparison-bar :value="report.engagement.data.bars.self['Views'] > 0 ? report.engagement.data.bars.self['Clicks to Website'] / report.engagement.data.bars.self['Views'] : 0" :max="1.0" color="#165E6F" background="#DEDEDE" width="100%" height="2rem" /> -->
  <!--         <span class="con-num">{{report.engagement.data.bars.self['Views'] > 0 ? ((report.engagement.data.bars.self['Clicks to Website'] / report.engagement.data.bars.self['Views']) * 100).toFixed(2) : 0}}%</span> -->
  <!--       </div> -->
  <!--     </div> -->
  <!--     <div class="conversion-rate"> -->
  <!--       <h5>All SNM Opportunity Median</h5> -->
  <!--       <div class="flex align-center"> -->
  <!--         <strong>{{report.engagement.data.bars.median['Clicks to Website']}}</strong> median clicks to opportunity web sites, across all opportunities on Science Near Me -->
  <!--         <comparison-bar :value="report.engagement.data.bars.median['Views'] > 0 ? report.engagement.data.bars.median['Clicks to Website'] / report.engagement.data.bars.median['Views'] : 0" :max="1.0" color="#7CB4BF" background="#DEDEDE" width="100%" height="2rem" /> -->
  <!--         <span class="con-num">{{report.engagement.data.bars.median['Views'] > 0 ? ((report.engagement.data.bars.median['Clicks to Website'] / report.engagement.data.bars.median['Views'])*100).toFixed(2) : 0}}%</span> -->
  <!--       </div> -->
  <!--     </div> -->
  <!--     <div class="conversion-rate"> -->
  <!--       <h5>All SNM Opportunity Average</h5> -->
  <!--       <div class="flex align-center"> -->
  <!--         <strong>{{report.engagement.data.bars.mean['Clicks to Website']}}</strong> average clicks to opportunity web sites, across all opportunities on Science Near Me -->
  <!--         <comparison-bar :value="report.engagement.data.bars.mean['Views'] > 0 ? report.engagement.data.bars.mean['Clicks to Website'] / report.engagement.data.bars.mean['Views'] : 0" :max="1.0" color="#7CB4BF" background="#DEDEDE" width="100%" height="2rem" /> -->
  <!--         <span class="con-num">{{report.engagement.data.bars.mean['Views'] > 0 ? ((report.engagement.data.bars.mean['Clicks to Website'] / report.engagement.data.bars.mean['Views'])*100).toFixed(2) : 0}}%</span> -->
  <!--       </div> -->
  <!--     </div> -->
  <!--   </div> -->
  </div>

  <div v-else-if="state=='states'">
    <h2>Audience</h2>

    <div class="notification">
      <label>Demographics Coming Soon!</label>
      We are working on getting demographic data at the opportunity level. Right now you can view <nuxt-link to="/">site-wide demographic data</nuxt-link>.
    </div>

    <div class="filters">
      <!-- <div class="stack"> -->
      <!--   <label>Opportunity Status</label> -->
      <!--   <b-select :value="report.states.data.opportunity_status" @input="log('TBD download from server')"> -->
      <!--     <option v-for="status in report.states.opportunity_statuses" :key="status" :value="status"> -->
      <!--       {{status}} -->
      <!--     </option> -->
      <!--   </b-select> -->
      <!-- </div> -->
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report.states.data.time_period" @input="load_data_into(current_opp.uid, 0, $event, 'Live and Closed', 'states')">
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

    <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          <th>Engagement By Location</th>
          <th colspan="2">Unique Users <b-tooltip label="Individual users only counted once." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="states_top_order == 'unique_users_desc'" @click="states_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'unique_users_asc'" @click="states_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">New Users <b-tooltip label="First time visitors." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="states_top_order == 'new_users_desc'" @click="states_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'new_users_asc'" @click="states_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Returning Users <b-tooltip label="Visitors who have viewed more than once." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="states_top_order == 'returning_users_desc'" @click="states_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'returning_users_asc'" @click="states_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Total Pageviews <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="states_top_order == 'total_pageviews_desc'" @click="states_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'total_pageviews_asc'" @click="states_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Unique Pageviews <b-tooltip label="Times the page was viewed by a unique user." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="states_top_order == 'unique_pageviews_desc'" @click="states_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="states_top_order == 'unique_pageviews_asc'" @click="states_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="states_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Avg. Time <b-tooltip label="The average time spent on your page by users." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
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

    <div class="data-wrapper crush">
      <div class="data-head">
        <h3>Technology</h3>
        <b-select :value="report.technology.data.time_period" @input="load_data_into(current_opp.uid, 0, $event, 'Live and Closed', 'technology')">
          <option v-for="period in report.technology.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
      <pie-chart :data="technology_pie" doughnut />
    </div>
    <div class="data-table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>Engagement By Device Type</th>
            <th colspan="2">Unique Users <b-tooltip label="Individual users only counted once." position="is-top" append-to-body multilined>
                <b-button label="?" />
              </b-tooltip>
              <a v-if="technology_top_order == 'unique_users_desc'" @click="technology_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
              <a v-else-if="technology_top_order == 'unique_users_asc'" @click="technology_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
              <a v-else @click="technology_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
            </th>
            <th colspan="2">New Users <b-tooltip label="First time visitors." position="is-top" append-to-body multilined>
                <b-button label="?" />
              </b-tooltip>
              <a v-if="technology_top_order == 'new_users_desc'" @click="technology_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
              <a v-else-if="technology_top_order == 'new_users_asc'" @click="technology_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
              <a v-else @click="technology_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
            </th>
            <th colspan="2">Returning Users <b-tooltip label="Visitors who have viewed more than once." position="is-top" append-to-body multilined>
                <b-button label="?" />
              </b-tooltip>
              <a v-if="technology_top_order == 'returning_users_desc'" @click="technology_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
              <a v-else-if="technology_top_order == 'returning_users_asc'" @click="technology_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
              <a v-else @click="technology_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
            </th>
            <th colspan="2">Total Pageviews <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined>
                <b-button label="?" />
              </b-tooltip>
              <a v-if="technology_top_order == 'total_pageviews_desc'" @click="technology_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
              <a v-else-if="technology_top_order == 'total_pageviews_asc'" @click="technology_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
              <a v-else @click="technology_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
            </th>
            <th colspan="2">Unique Pageviews <b-tooltip label="Times the page was viewed by a unique user." position="is-top" append-to-body multilined>
                <b-button label="?" />
              </b-tooltip>
              <a v-if="technology_top_order == 'unique_pageviews_desc'" @click="technology_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
              <a v-else-if="technology_top_order == 'unique_pageviews_asc'" @click="technology_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
              <a v-else @click="technology_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
            </th>
            <th colspan="2">Avg. Time <b-tooltip label="The average time spent on your page by users." position="is-top" append-to-body multilined>
                <b-button label="?" />
              </b-tooltip>
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
            <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report.technology.data.max['Unique Users']" color="#268699" /></td>
            <td class="table-num">{{row['New Users']}}</td>
            <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report.technology.data.max['New Users']" color="#268699" /></td>
            <td class="table-num">{{row['Returning Users']}}</td>
            <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report.technology.data.max['Returning Users']" color="#268699" /></td>
            <td class="table-num">{{row['Total Pageviews']}}</td>
            <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report.technology.data.max['Total Pageviews']" color="#268699" /></td>
            <td class="table-num">{{row['Unique Pageviews']}}</td>
            <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report.technology.data.max['Unique Pageviews']" color="#268699" /></td>
            <td class="table-num">{{row['Avg. Time']}}</td>
            <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report.technology.data.max['Avg. Time']" color="#268699" /></td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>

  <div v-else-if="state=='traffic'">
    <h2>Traffic</h2>

    <div class="filters">
      <!-- <div class="stack"> -->
      <!--   <label>Opportunity Status</label> -->
      <!--   <b-select :value="report.engagement.data.opportunity_status" @input="log('TBD download from server')"> -->
      <!--     <option v-for="status in report.engagement.opportunity_statuses" :key="status" :value="status"> -->
      <!--       {{status}} -->
      <!--     </option> -->
      <!--   </b-select> -->
      <!-- </div> -->
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report.traffic.data.time_period" @input="load_data_into(current_opp.uid, 0, $event, 'Live and Closed', 'traffic')">
          <option v-for="period in report.traffic.time_periods" :key="period" :value="period">
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
          <div class="ll-icon ll-icon-user"><user-icon></user-icon></div>
          <div>
            <h2># (Total Unique Users during time period)</h2>
            <h3>Users</h3>

          </div>
        </div>
        <div class="ll-legend">
              <div><span class="dark-blue"></span> Unique</div>
              <div><span class="light-blue"></span> New</div>
              <div><span class="light-blue"></span> Returning</div>
            </div>
        </div>
      <client-only>
        <line-chart
          :rows="report.traffic.data.chart"
          :xaxis="d => new Date(d.date)"
          :yaxes="['Unique', 'New', 'Returning']"
          :colors="['#268699', '#868686', '#BFDCE2']"
          />
      </client-only>
    </div>

    <div class="data-wrapper crush">
      <div class="data-head">
          <h3>Referral Sources</h3>
        </div>
    <pie-chart :data="report.traffic.data.pie" />
      </div>
      <div class="data-table-wrapper">

    <table class="data-table">
      <thead>
        <tr>
          <th>Top Referrers</th>
          <th colspan="2">Unique Users <b-tooltip label="Individual users only counted once." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="traffic_top_order == 'unique_users_desc'" @click="traffic_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'unique_users_asc'" @click="traffic_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">New Users <b-tooltip label="First time visitors." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="traffic_top_order == 'new_users_desc'" @click="traffic_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'new_users_asc'" @click="traffic_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Returning Users <b-tooltip label="Visitors who have viewed more than once." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="traffic_top_order == 'returning_users_desc'" @click="traffic_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'returning_users_asc'" @click="traffic_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Total Pageviews <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="traffic_top_order == 'total_pageviews_desc'" @click="traffic_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'total_pageviews_asc'" @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Unique Pageviews <b-tooltip label="Times the page was viewed by a unique user." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="traffic_top_order == 'unique_pageviews_desc'" @click="traffic_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="traffic_top_order == 'unique_pageviews_asc'" @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Avg. Time <b-tooltip label="The average time spent on your page by users." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
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
          <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report.traffic.data.max['Unique Users']" color="#268699" /></td>
          <td class="table-num">{{row['New Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report.traffic.data.max['New Users']" color="#268699" /></td>
          <td class="table-num">{{row['Returning Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report.traffic.data.max['Returning Users']" color="#268699" /></td>
          <td class="table-num">{{row['Total Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report.traffic.data.max['Total Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Unique Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report.traffic.data.max['Unique Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Avg. Time']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report.traffic.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>
  </div>
  <div v-else-if="state=='overlap'">
    <div class="explain-header">
      <h2>Engagement Overlap</h2>
    <small>See what people who engage with your Science Near Me page tend to engage with. </small>

    </div>

    <div class="data-wrapper">

      <div class="data-head"><h3>Top 50</h3></div>

      <div class="bubble-header">
      <!-- It turns out that only 'views' is a practicable calculation, so disable the ability to select other criteria -->
      <!-- <div> -->
      <!--   <label>Engagement Type</label> -->
      <!--   <b-select :value="report.overlap.data.engagement_type" @input="log('TBD download from server')"> -->
      <!--     <option v-for="e_type in report.overlap.engagement_types" :key="e_type" :value="e_type"> -->
      <!--       {{e_type}} -->
      <!--     </option> -->
      <!--   </b-select> -->
      <!-- </div> -->

      <div class="ll-legend">
          <div><span class="gray"></span> {{current_opp.title}} (100% of users)</div>
          <div><span class="dark-blue"></span> Opportunity Owned By You</div>
          <div><span class="light-blue"></span> Opportunity Owned By Someone Else</div>
        </div>
      </div>

      <client-only>
        <bubble-chart :chart_data="report.overlap.data" :org="current_opp.title" :current_opp="current_opp" />
      </client-only>
    </div>


    <!-- Making this table sortable by different fields would be a
    huge can of worms, since it brings up logic conflicts and
    ambiguities. Seems like it should always be sorted by overlap
    percent anyhow, so that's what I've done here. -->

    <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          <th>Opportunity</th>
          <th colspan="2">Overlap Percent <b-tooltip label="The percent of users who veiwed both your opportunity and the listed opportunity." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
          <th>Hosted By <b-tooltip label="The host of the listed opportunity." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
          <th>Activity Types <b-tooltip label="The types of activities of the opportunity." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
          <th>Format <b-tooltip label="If the opportunity is event or On-Demand." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
          <th>Venue Types <b-tooltip label="If the opportunity is indoors or outdoors." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
          <th>Min. Age <b-tooltip label="Any minimum age set for participating in the opportunity." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
          <th>Max. Age <b-tooltip label="Any maximum age set for participating in the opportunity." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="row in report.overlap.data.table">
          <td>{{row.name}}</td>
          <td class="table-num">{{percent(row.overlap)}}</td>
          <td class="table-bar"><comparison-bar :value="row.overlap" :max="1.0" color="#268699" /></td>
          <td>{{row.host}}</td>
          <td>{{friendly(row.activity_types)}}</td>
          <td>{{row.format}}</td>
          <td>{{friendly(row.venue_types)}}</td>
          <td v-if="row.min_age > 0" >{{row.min_age}}</td>
          <td v-else><small>N/A</small></td>
          <td v-if="row.max_age < 999">{{row.max_age}}</td>
          <td v-else><small>N/A</small></td>
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
import UserIcon from '~/assets/img/user.svg?inline'
export default {
    name: "MyOpportunityDataExplorer",
    components:{
      EyeIcon,
      LinkIcon,
      SortIcon,
      SortableIcon,
      UserIcon
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

        const opps = (await context.$axios.$get('/api/ui/finder/search?mine=true&current=true&sort=alphabetical&per_page=4294967295&refs=true', context.store.state.auth))
              .matches
              .concat((await context.$axios.$get('/api/ui/finder/search?mine=true&current=false&sort=alphabetical&per_page=4294967295&refs=true', context.store.state.auth))
                      .matches);

        if(opps.length == 0) {
            context.error({
                statusCode: 404,
                message: "No opportunities"
            });
        }

        const current_opp = !!context.query.opp ? (opps.filter(opp => opp.slug == context.query.opp) || opps[0]) : opps[0];

        const report = await context.$axios.$get("/api/ui/organization/analytics", {
            params: {
                about: current_opp.uid,
                kind: 0,
                period: "This Month",
                status: "Live and Closed"
            },
            ...context.store.state.auth
        });

        return {
            opps,
            current_opp,
            report,
        };
    },

    data() {
        return {
            state: 'engagement',
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

        updated_local() {
            if(!this.report || !this.report.updated) {
                return "";
            }
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

        async load_opp(opp) {
            const loading = this.$buefy.loading.open({container: null});
            const info = await this.$axios.$get("/api/ui/organization/analytics", {
                params: {
                    about: opp.uid,
                    kind: 0,
                    period: "This Month",
                    status: "Live and Closed"
                },
                ...this.$store.state.auth
            });
            this.report = info;
            this.current_opp = opp;
            loading.close();
        },

        async load_data_into(about, kind, period, status, field) {
            const loading = this.$buefy.loading.open({container: null});
            const info = await this.$axios.$get("/api/ui/organization/analytics", {params: {about, kind, period, status, field}, ...this.$store.state.auth});
            this.report[field].data = info;
            loading.close();
        },

        friendly(list) {
            return list.map(x => x.split("_").map(w => w[0].toUpperCase() + w.slice(1)).join(" ")).join(", ");
        },

        percent(x) {
            return (100.0 * x).toFixed(1) + "%";
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
                this.cirrent_opp.title + ' engagement ' + this.report.engagement.data.begin + ' - ' + this.report.engagement.data.end,
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
                this.current_opp.title + ' traffic ' + this.report.traffic.data.begin + ' - ' + this.report.traffic.data.end,
                ['date', ...this.report.traffic.data.columns],
                structured
            );
        },
    },
}
</script>

<style lang="scss" scoped>
small {
    font-size: 10pt;
    color: #999;
}

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
    margin-bottom: 2rem;

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

.conversion-rate {
    margin: 1rem;
}

#current-proportion {
    margin: 1rem auto;
}

$linework : #dee2e6;
$lightblue: #BFDCE2;

.data-wrapper, .data-table {
  border: 1px solid $linework;
  margin-bottom: 2rem;
}

.data-wrapper {
  h5 {
    font-size: .8rem;
  }
}

.data-wrapper > .data-table {
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
    color: #5694A2;
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
    margin-right: 10px;
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
    &.ll-icon-user svg {
      height:32px;
      width: 32px;
    }
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
    white-space: nowrap;
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

.data-table-wrapper {
  width: 100%;
  overflow: auto;
  max-height: 500px;
  margin-bottom: 2rem;
  border-bottom: 1px solid $linework;
  border-top: 1px solid $linework;
  .data-table {
    border-top: 0;
    border-bottom: 0;
    margin-bottom: 0;

    :tr:last-child {
      border-bottom: 0;
    }
  }
  .data-table thead {
    position: sticky;
    top:0;
    background-color: #fff;
    z-index: 99;
  }
}

.data-wrapper.crush {
  margin-bottom:0;
  border-bottom: 0;
  padding-bottom: 1rem;
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

:deep(select){
  max-width: 200px;
}

.conversion-rate :deep(.outer), .conversion-rate :deep(.inner) {
  border-radius: 30px;
  height: 16px!important;
}

.conversion-rate :deep(.outer) {
  width: calc(100% - 50px);
}

.data-header2 {
  padding: 2rem 1rem 0 1rem;
  display: flex;
  align-items: baseline;
  h4 {
    font-weight: bold;
    margin-right: 6px;
  }
}

.con-num {
  font-weight: bold;
  font-size: 12px;
  margin-left: 6px;
}

.align-center {
  align-items: center;
}

.tooltip-trigger button {
    height: 1rem;
    width: 1rem;
    border-radius: 100%;
    padding: 0.5rem;
    font-size: 14px;
    margin-left: 0;
    background-color: $snm-color-action;
    border:0;
    position: relative;
    top:4px;
    margin-right: 8px;
    color: #fff;
    &:hover {
      color:#fff;
    }
  }

  .nav-mobile-wrapper {
    width: 100%;
    overflow-x: auto;
    .nav-tabs {
      margin-bottom: 0;
    }
  }
  .nav-tab-wrapper {
    min-width: 840px;
  }

  .data-update {
    margin-top: 2rem;
  }

  .bubble-header {
    padding:1rem;

    label {
      font-weight: bold;
    font-size: 14px;
    margin-bottom: 4px;
    display: block;
    }

    .ll-legend {
      margin-top: 1.2rem;
      > div {
        margin-right:2rem;
      }
      .gray {
        background-color: #dedede;
      }
    }
  }

  .explain-header {
    h2 {
      margin-bottom:4px;
    }
    small {
      display: block;
      margin-bottom: 2rem;
    }
  }

</style>
