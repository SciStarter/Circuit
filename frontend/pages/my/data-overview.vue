<template>
<div class="your-data-overview">
  <b-select v-if="available_orgs.length > 1" v-model="org" placeholder="Select Organization">
    <option v-for="org_name in available_orgs" :value="org_name" :key="org_name">
      {{org_name}}
    </option>
  </b-select>

  <div class="flex-header">
    <h1>Your Data Overview</h1>
  </div>

  <div id="current-proportion" class="labeled-gauge-ends">
    <progress-gauge class="gauge" :value="report[org].total_opportunities - report[org].current_opportunities" :max="report[org].total_opportunities" reverse/>
    <div class="labels">
      <div class="stack">
        <label>{{report[org].total_opportunities}}</label>
        <small>Opportunities<br>Total on SNM <b-tooltip label="Number of science opportunities that have ever appeared Science Near Me." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></small>
      </div>
      <div class="stack">
        <label>{{report[org].current_opportunities}}</label>
        <small>Opportunities<br>Current &amp; Future <b-tooltip label="Number of science opportunities that currently appear Science Near Me." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></small>
      </div>
    </div>
  </div>

  <div class="nav-mobile-wrapper">
  <div class="nav-tab-wrapper">
    <ul class="nav-tabs nav-tabs-alt">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'"><span>Engagement</span><small>How users interact with your opportunity pages</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'"><span>Audience</span><small>Demographics, locations and technology of your users</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'"><span>Traffic</span><small>How users get to your pages</small></a></li>
    </ul>
  </div>
</div>

  <aside class="data-update">Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">
    <h2>Engagement</h2>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report[org].engagement.data.opportunity_status" @input="load_data_into(report[org].organization, 0, report[org].engagement.data.time_period, $event, 'engagement')">
          <option v-for="status in report[org].engagement.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report[org].engagement.data.time_period" @input="load_data_into(report[org].organization, 0, $event, report[org].engagement.data.opportunity_status, 'engagement')">
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
            <h3>Page Views <b-tooltip label="Number of times a page has been viewed. Total contains repeating users. Unique does not contain a user's repeated views." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></h3>
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
            <h3>Clicks to Website <b-tooltip label="Number of times Science Near Me refers a user to the opportunity's website." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></h3>
        </div>
        </div>
      </div>
      <client-only>
        <line-chart
          :rows="report[org].engagement.data.chart"
          :xaxis="d => new Date(d.date)"
          :yaxes="['Views', 'Unique', 'Clicks to Website']"
          :colors="['#268699', '#BFDCE2', '#FABF40']"
          />
      </client-only>
    </div>


    <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
        <tr>
          <th>Top Performing Opportunities  <b-tooltip label="Opportunities with the most engagement on Science Near Me." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></th>
          <th colspan="2">Total Views <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
            <a v-if="engagement_top_order == 'total_views_desc'" @click="engagement_top_order = 'total_views_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="engagement_top_order == 'total_views_asc'" @click="engagement_top_order = 'total_views_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="engagement_top_order = 'total_views_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Clicks to Website <b-tooltip label="The number of times users clicked on the link to your website." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip>
            <a v-if="engagement_top_order == 'clicks_desc'" @click="engagement_top_order = 'clicks_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="engagement_top_order == 'clicks_asc'" @click="engagement_top_order = 'clicks_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="engagement_top_order = 'clicks_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
        </tr>
      </thead>
      <tbody v-if="engagement_top_sorted.length > 0">
        <tr v-for="row in engagement_top_sorted">
          <td><nuxt-link :to="'/my/opportunity-data-explorer?opp=' + row['slug']">{{row['name']}}</nuxt-link></td>
          <td class="table-num">{{row['Views']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Views']" :max="report[org].engagement.data.max['Views']" color="#268699" /></td>
          <td class="table-num">{{row['Clicks to Website']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Clicks to Website']" :max="report[org].engagement.data.max['Clicks to Website']" color="#FABF40" /></td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr>
          <td colspan="5">No Data to Display</td>
        </tr>
      </tbody>
    </table>
  </div>

   <h2 class="h2sep">User Searches</h2>
   <div class="filters">
      <div class="stack">
        <label>Location of User</label>
        <b-select v-model="location_us" @input="get_us_state($event)" style="margin-bottom: 10px;">
          <option key="all" value="all" selected>All States</option>
          <option v-for="s in metro_groups.all" :key="s" :value="s">{{ s }}</option>
        </b-select>
        <b-select v-model="location_metro" v-if="show_metro">
          <option :value="all">All Areas</option>
          <option v-for="metro in metro_groups[location_us]" :value="metro">{{ metro }}</option>
        </b-select>
      </div>
    </div>
   <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
      <tr>
        <th class="narrow-column">Top Searches by Keyword  <b-tooltip label="Most used keyword searches on Science Near Me, by location selected." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></th>
        <th colspan="2">Total Searches</th>
      </tr>
    </thead>
      <tbody v-if="local_searches && local_searches.length > 0">
        <tr v-for="row in local_searches">
          <td class="narrow-column">{{row[0]}}</td>
          <td class="table-num">{{row[1]}}</td>
          <td class="table-bar"><comparison-bar :value="row[1]" :max="local_searches_max" color="#165E6F" width="100%" height="1rem" /></td>
        </tr> 
      </tbody>
      <tbody v-else>
        <tr>
          <td colspan="3">No Data to Display</td>
        </tr>
      </tbody>
    </table>
  </div>

  </div>

  <div v-else-if="state=='states'">
    <h2>Audience</h2>

    <div class="notification">
      <label>Demographics Coming Soon!</label>
      We are working on getting demographic data at the opportunity level. Right now you can view <nuxt-link to="/my/snm-data-overview">site-wide demographic data</nuxt-link>.
    </div>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report[org].states.data.opportunity_status" @input="load_data_into(report[org].organization, 0, report[org].states.data.time_period, $event, 'states')">
          <option v-for="status in report[org].states.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report[org].states.data.time_period" @input="load_data_into(report[org].organization, 0, $event, report[org].states.data.opportunity_status, 'states')">
          <option v-for="period in report[org].states.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
    </div>

    <choropleth-states v-if="selected_state === null" :value="report[org].states.data.states" attr="Unique Users" @state="select_state($event)"/>
    <div v-else>
      <a @click="selected_state = null">← Back to US Map</a>
      <b-select v-model="selected_attr" placeholder="Select Data Type" style="margin-bottom:20px">
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
      <tbody v-if="states_top_sorted.length > 0">
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
          <td class="table-num">{{(row['Avg. Time'] / 1000).toFixed(2)}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="states_max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
      <tbody v-else>
        <tr>
          <td colspan="13">No Data to Display</td>
        </tr>
      </tbody>
    </table>
  </div>

    <div class="data-wrapper crush">

        <div class="data-head">
          <h3>Technology</h3>
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
          <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report[org].technology.data.max['Unique Users']" color="#268699" /></td>
          <td class="table-num">{{row['New Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report[org].technology.data.max['New Users']" color="#268699" /></td>
          <td class="table-num">{{row['Returning Users']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report[org].technology.data.max['Returning Users']" color="#268699" /></td>
          <td class="table-num">{{row['Total Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report[org].technology.data.max['Total Pageviews']" color="#268699" /></td>
          <td class="table-num">{{row['Unique Pageviews']}}</td>
          <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report[org].technology.data.max['Unique Pageviews']" color="#268699" /></td>
          <td class="table-num">{{(row['Avg. Time'] / 1000).toFixed(2)}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report[org].technology.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>

  </div>


  <div v-else-if="state=='traffic'">
    <h2>Traffic</h2>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report[org].traffic.data.opportunity_status" @input="load_data_into(report[org].organization, 0, report[org].traffic.data.time_period, $event, 'traffic')">
          <option v-for="status in report[org].engagement.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report[org].traffic.data.time_period" @input="load_data_into(report[org].organization, 0, $event, report[org].traffic.data.opportunity_status, 'traffic')">
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
    <client-only>
      <line-chart
        :rows="report[org].traffic.data.chart"
        :xaxis="d => new Date(d.date)"
        :yaxes="['Unique', 'New', 'Returning']"
        :colors="['#268699', '#868686', '#BFDCE2']"
        />
    </client-only>
    </div>


    <div class="data-wrapper crush">
      <div class="data-head">
          <h3>Referral Sources  <b-tooltip label="How users arrive to Science Near Me before finding your opportunity." position="is-top" append-to-body multilined><b-button label="?" /></b-tooltip></h3>
        </div>
    <pie-chart :data="report[org].traffic.data.pie" />
      </div>




  <!-- # Disabled due to the fact that almost all sites no longer provide us with referrer information # -->

  <!--   <div class="data-table-wrapper"> -->
  <!--   <table class="data-table"> -->
  <!--     <thead> -->
  <!--       <tr> -->
  <!--         <th>Top Referrers</th> -->
  <!--         <th colspan="2">Unique Users <b-tooltip label="Individual users only counted once." position="is-top" append-to-body multilined> -->
  <!--         <b-button label="?" /> -->
  <!--       </b-tooltip> -->
  <!--           <a v-if="traffic_top_order == 'unique_users_desc'" @click="traffic_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
  <!--           <a v-else-if="traffic_top_order == 'unique_users_asc'" @click="traffic_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
  <!--           <a v-else @click="traffic_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
  <!--         </th> -->
  <!--         <th colspan="2">New Users <b-tooltip label="First time visitors." position="is-top" append-to-body multilined> -->
  <!--         <b-button label="?" /> -->
  <!--       </b-tooltip> -->
  <!--           <a v-if="traffic_top_order == 'new_users_desc'" @click="traffic_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
  <!--           <a v-else-if="traffic_top_order == 'new_users_asc'" @click="traffic_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
  <!--           <a v-else @click="traffic_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
  <!--         </th> -->
  <!--         <th colspan="2">Returning Users <b-tooltip label="Visitors who have viewed more than once." position="is-top" append-to-body multilined> -->
  <!--         <b-button label="?" /> -->
  <!--       </b-tooltip> -->
  <!--           <a v-if="traffic_top_order == 'returning_users_desc'" @click="traffic_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
  <!--           <a v-else-if="traffic_top_order == 'returning_users_asc'" @click="traffic_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
  <!--           <a v-else @click="traffic_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
  <!--         </th> -->
  <!--         <th colspan="2">Total Pageviews <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined> -->
  <!--         <b-button label="?" /> -->
  <!--       </b-tooltip> -->
  <!--           <a v-if="traffic_top_order == 'total_pageviews_desc'" @click="traffic_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
  <!--           <a v-else-if="traffic_top_order == 'total_pageviews_asc'" @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
  <!--           <a v-else @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
  <!--         </th> -->
  <!--         <th colspan="2">Unique Pageviews <b-tooltip label="Times the page was viewed by a unique user." position="is-top" append-to-body multilined> -->
  <!--         <b-button label="?" /> -->
  <!--       </b-tooltip> -->
  <!--           <a v-if="traffic_top_order == 'unique_pageviews_desc'" @click="traffic_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
  <!--           <a v-else-if="traffic_top_order == 'unique_pageviews_asc'" @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
  <!--           <a v-else @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
  <!--         </th> -->
  <!--         <th colspan="2">Avg. Time <b-tooltip label="The average time spent on your page by users." position="is-top" append-to-body multilined> -->
  <!--         <b-button label="?" /> -->
  <!--       </b-tooltip> -->
  <!--           <a v-if="traffic_top_order == 'average_time_desc'" @click="traffic_top_order = 'average_time_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
  <!--           <a v-else-if="traffic_top_order == 'average_time_asc'" @click="traffic_top_order = 'average_time_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
  <!--           <a v-else @click="traffic_top_order = 'average_time_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
  <!--         </th> -->
  <!--       </tr> -->
  <!--     </thead> -->
  <!--     <tbody> -->
  <!--       <tr v-for="row in traffic_top_sorted"> -->
  <!--         <td>{{row['name'] || row['type_'] || '[unavailable]'}}</td> -->
  <!--         <td class="table-num">{{row['Unique Users']}}</td> -->
  <!--         <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report[org].traffic.data.max['Unique Users']" color="#268699" /></td> -->
  <!--         <td class="table-num">{{row['New Users']}}</td> -->
  <!--         <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report[org].traffic.data.max['New Users']" color="#268699" /></td> -->
  <!--         <td class="table-num">{{row['Returning Users']}}</td> -->
  <!--         <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report[org].traffic.data.max['Returning Users']" color="#268699" /></td> -->
  <!--         <td class="table-num">{{row['Total Pageviews']}}</td> -->
  <!--         <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report[org].traffic.data.max['Total Pageviews']" color="#268699" /></td> -->
  <!--         <td class="table-num">{{row['Unique Pageviews']}}</td> -->
  <!--         <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report[org].traffic.data.max['Unique Pageviews']" color="#268699" /></td> -->
  <!--         <td class="table-num">{{(row['Avg. Time'] / 1000).toFixed(2)}}</td> -->
  <!--         <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report[org].traffic.data.max['Avg. Time']" color="#268699" /></td> -->
  <!--       </tr> -->
  <!--     </tbody> -->
  <!--   </table> -->
  <!-- </div> -->
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
        
        const report = await context.$axios.$get("/api/ui/organization/analytics", context.store.state.auth);

        let metro_groups = {'all': []};
        let group = "";
        let metros = [];
        
        for(let item of await context.$axios.$get("/api/ui/finder/metros")) {
            if(item[0] != group) {
                if(metros.length > 0) {
                    metro_groups['all'].push(group);
                    metro_groups[group] = metros;
                }
                group = item[0];
                metros = [];
            }
            if(item[1]) {
                metros.push(item[1]);
            }
        }
        
        if(metros.length > 0) {
            metro_groups['all'].push(group);
            metro_groups[group] = metros;
        }

        let local_searches = await context.$axios.$get("/api/ui/finder/metro-searches", context.store.state.auth);
        let local_searches_max = 0;

        if(local_searches.length > 0) {
            local_searches_max = local_searches[0][1];
        }

        return {
            report,
            metro_groups,
            local_searches,
            local_searches_max
        };
    },

    data() {
        return {
            state: 'engagement',
            selected_org: '',
            engagement_top_order: 'total_views_desc',
            states_top_order: 'unique_users_desc',
            technology_top_order: 'unique_users_desc',
            traffic_top_order: 'unique_users_desc',
            selected_state: null,
            selected_attr: "Unique Users",
            location_us: "all",
            location_metro: "all",
            show_metro: false,
            local_searching: 0
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },

        available_orgs() {
            return Object.getOwnPropertyNames(this.report).filter(n => n != 'initial' && !n.startsWith('_'));
        },

        org: {
            async set(val) {
                if(this.report[val].updated === undefined) {
                    const info = await this.$axios.$get("/api/ui/organization/analytics", {params: {about: this.report[val].organization, kind: 0, period: "This Month", status: "Live and Closed"}, ...this.$store.state.auth});
                    this.report[val] = info;
                    this.selected_org = val;
                }
                else {
                    this.selected_org = val;
                }
            },

            get() {
                return this.selected_org ? this.selected_org : this.report.initial;
            }
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

    watch: {
        location_us() {
            clearTimeout(this.local_searching);
            this.local_searching = setTimeout(this.update_local, 500);
        },

        location_metro() {
            clearTimeout(this.local_searching);
            this.local_searching = setTimeout(this.update_local, 500);
        }
    },

    methods: {
        log(msg) {
            console.log(msg);
        },

        async update_local() {
            let url = "/api/ui/finder/metro-searches";

            if(this.location_us != 'all') {
                url = url + '?state=' + this.location_us;

                if(this.location_metro != 'all') {
                    url = url + '&metro=' + this.location_metro;
                }
            }

            this.local_searches = await this.$axios.$get(url, this.$store.state.auth);

            if(this.local_searches.length > 0) {
                this.local_searches_max = this.local_searches[0][1];
            }
            else {
                this.local_searches_max = 0;
            }
        },

        async load_data_into(about, kind, period, status, field) {
            const loading = this.$buefy.loading.open({container: null});
            const info = await this.$axios.$get("/api/ui/organization/analytics", {params: {about, kind, period, status, field}, ...this.$store.state.auth});
            this.report[this.org][field].data = info;
            loading.close();
        },

        select_state(state) {
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
        get_us_state(s){
          if (s == "all"){
            /* get all state data */
            this.show_metro = false;
          } else {
            /* get state data */
            this.show_metro = true;
          }
        }
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
    color: #bbb;
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
    min-width: 628px;
  }

  .data-update {
    margin-top: 2rem;
  }

  @media (max-width:1159px) {
  .snm-container {
    padding:1rem;
  }
}
.h2sep {
  border-top:1px solid #efefef;
  margin-bottom: 12px;
  padding-top:16px;
  font-size: 1.2rem;
}

</style>
