<template>
<div class="your-data-overview">
  <div class="flex-header">
    <h1>SNM Data Overview</h1>
  </div>

  <div class="nav-mobile-wrapper">
  <div class="nav-tab-wrapper">
    <ul class="nav-tabs nav-tabs-alt">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'"><span>Engagement</span><small>How users interact with our site</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'"><span>Audience</span><small>Learn about SNM users</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'"><span>Traffic</span><small>How users get to our opportunities</small></a></li>
      <li><a class="tab-link" :class="{'active':state=='domain'}" @click="state='domain'"><span>Domain Insights</span><small>See how our users interact across science domains</small></a></li>
    </ul>
  </div>
</div>

  <aside class="data-update">Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">


    <div class="flex flex2">
      <div id="snm-unique-visitors" class="stack center big-legend">
        <telescope-icon></telescope-icon>
        <h2>{{report.engagement.data.stats.unique_visitors}}</h2>
        <h3>Unique SNM Visitors</h3>
      </div>
      <div id="snm-accounts" class="stack center big-legend">
        <atom-icon></atom-icon>
        <h2>{{report.engagement.data.stats.accounts}}</h2>
        <h3>SNM Accounts</h3>
      </div>
    </div>

    <h3>Total Engagement</h3>
    <div class="flex flex2 engagement-divs">
      <div id="snm-total-opp-page-views" class="big-legend bl-blue">
          <div class="ll-icon"><eye-icon></eye-icon><strong>T</strong></div>
          <div>
            <h2>{{Math.max(report.engagement.data.stats.opportunity_views, 6316)}}</h2> <!-- min from Google Analytics -->
            <h3>Total Opportunity Page Views</h3>
          </div>
      </div>
      <div id="snm-total-opp-unique" class="big-legend bl-blue">
        <div class="ll-icon"><eye-icon></eye-icon><strong>U</strong></div>
          <div>
            <h2>{{Math.max(report.engagement.data.stats.opportunity_unique, 5212)}}</h2> <!-- min from Google Analytics -->
            <h3>Unique Opportunity Page Views</h3>
          </div>
      </div>
    </div>

    <div class="flex flex3  engagement-divs">
      <div id="snm-website-clicks" class="big-legend bl-blue">
        <div class="ll-icon"><link-icon></link-icon></div>
          <div>
            <h2>{{report.engagement.data.stats.opportunity_exits}}</h2>
            <h3>Clicks to Websites</h3>
          </div>
      </div>
      <div id="snm-self-reports" class="big-legend bl-blue">
        <div class="ll-icon"><plus-icon></plus-icon></div>
          <div>
            <h2>{{report.engagement.data.stats.didits}}</h2>
            <h3>Self-Reports</h3>
          </div>
      </div>
      <div id="snm-saves" class="big-legend bl-blue">
        <div class="ll-icon"><save-icon></save-icon></div>
          <div>
            <h2>{{report.engagement.data.stats.saves}}</h2>
            <h3>Saves</h3>
          </div>
      </div>
    </div>

    <div class="flex flex3 engagement-divs">
      <div id="snm-likes" class="big-legend bl-blue">
        <div class="ll-icon"><like-icon></like-icon></div>
          <div>
            <h2>{{report.engagement.data.stats.likes}}</h2>
            <h3>Likes</h3>
          </div>
      </div>
      <div id="snm-shares" class="big-legend bl-blue">
        <div class="ll-icon"><share-icon></share-icon></div>
          <div>
            <h2>{{report.engagement.data.stats.shares}}</h2>
            <h3>Shares</h3>
          </div>
      </div>
      <div id="snm-calendar-adds" class="big-legend bl-blue">
        <div class="ll-icon calendar"><calendar-icon></calendar-icon></div>
          <div>
            <h2>{{report.engagement.data.stats.calendar_adds}}</h2>
            <h3>Calendar Adds</h3>
          </div>
      </div>
    </div>

    <div class="data-table-wrapper">
    <table class="data-table">
      <thead>
      <tr>
        <th class="narrow-column">Top 30 Searches by Keyword</th>
        <th colspan="2">Total Searches</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="row in report.engagement.data.searches">
        <td class="narrow-column">{{row.phrase}}</td>
        <td class="table-num">{{row.searches}}</td>
        <td class="table-bar"><comparison-bar :value="row.searches" :max="report.engagement.data.search_max" color="#165E6F" width="100%" height="1rem" /></td>
      </tr>
    </tbody>
    </table>
  </div>
  </div>

  <div v-else-if="state=='states'">
    <div class="data-wrapper">
      <div class="data-head">
        <h3>Sex &amp; Age</h3>
      </div>
      <div class="flex flex2 sex audience">
      <div class="donuts">
          <div class="donut donut-first">
            <label>Female</label>
            <div class="donut-wrap">
              <pie-chart :data="female_pie" doughnut simplify />
              <span class="female">{{percent(report.demographics.sex.female.proportion)}}</span>
            </div>
          </div>
          <div class="donut">
            <label>Male</label>
            <div class="donut-wrap">
            <pie-chart :data="male_pie" doughnut simplify />
            <span class="male">{{percent(report.demographics.sex.male.proportion)}}</span>
            </div>
          </div>
      </div>
      <div>
      <div class="bar-viz" v-for="entry in sorted_kv(report.demographics.age)">
        <label>{{entry[0]}}</label>
          <comparison-bar :value="entry[1].male.proportion" :max="entry[1].proportion" color="#165E6F" background="#7CB4BF" :width="(entry[1].proportion * 100)*5 + '%'" height="1rem" />
        <span>{{percent(entry[1].proportion)}}</span>
      </div>
    </div>
     </div>
    </div>

    <div class="flex flex2 fill audience">
        <div class="push">
          <div class="data-head">
            <h3>Ethnicity</h3>
          </div>
         
          <div class="bar-viz">
            <label>Caucasian</label>
              <comparison-bar :value="report.demographics.ethnicity['Cauc.'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.ethnicity['Cauc.'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>Hispanic</label>
            <comparison-bar :value="report.demographics.ethnicity['Hisp'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.ethnicity['Hisp'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>African American</label>
            <comparison-bar :value="report.demographics.ethnicity['Afr. Am.'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.ethnicity['Afr. Am.'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>Asian</label>
            <comparison-bar :value="report.demographics.ethnicity['Asian'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.ethnicity['Asian'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>Other</label>
            <comparison-bar :value="report.demographics.ethnicity['Other'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.ethnicity['Other'].proportion * 100).toFixed(2)}}%</span>
          </div>
        </div>

        <div class="push">
          <div class="data-head">
            <h3>Education</h3>
          </div>
          <div class="bar-viz">
            <label>No College</label>
            <comparison-bar :value="report.demographics.education['No College'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.education['No College'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>College</label>
            <comparison-bar :value="report.demographics.education['College'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.education['College'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>Grad School</label>
            <comparison-bar :value="report.demographics.education['Grad. Sch.'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.education['Grad. Sch.'].proportion * 100).toFixed(2)}}%</span>
          </div>
        </div>
      </div>

      <div class="flex flex2 fill audience">
        <div class="push">
          <div class="data-head">
            <h3>Household Income</h3>
          </div>
          <div class="bar-viz">
            <label>$0-50k</label>
            <comparison-bar :value="report.demographics.income['$0-50k'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.income['$0-50k'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>$50-100k</label>
            <comparison-bar :value="report.demographics.income['$50-100k'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.income['$50-100k'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>$100-150k</label>
            <comparison-bar :value="report.demographics.income['$100-150k'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.income['$100-150k'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>$150k+</label>
            <comparison-bar :value="report.demographics.income['$150k+'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.income['$150k+'].proportion * 100).toFixed(2)}}%</span>
          </div>
        </div>

        <div class="push">
          <div class="data-head">
            <h3>Children</h3>
          </div>
          <div class="bar-viz">
            <label>Has Kids</label>
            <comparison-bar :value="report.demographics.children['Some Children under 17'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.children['Some Children under 17'].proportion * 100).toFixed(2)}}%</span>
          </div>
          <div class="bar-viz">
            <label>No Kids</label>
            <comparison-bar :value="report.demographics.children['No Children under 17'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
            <span>{{(report.demographics.children['No Children under 17'].proportion * 100).toFixed(2)}}%</span>
          </div>
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
          <td class="table-num">{{(row['Avg. Time'] / 1000).toFixed(2)}}</td>
          <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="states_max['Avg. Time']" color="#268699" /></td>
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
          <th colspan="2">Unique Users
            <b-tooltip label="Individual users only counted once." position="is-top" append-to-body multilined>
              <b-button label="?" />
            </b-tooltip>
            <a v-if="technology_top_order == 'unique_users_desc'" @click="technology_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'unique_users_asc'" @click="technology_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">New Users
            <b-tooltip label="First time visitors." position="is-top" append-to-body multilined>
              <b-button label="?" />
            </b-tooltip>
            <a v-if="technology_top_order == 'new_users_desc'" @click="technology_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'new_users_asc'" @click="technology_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Returning Users
            <b-tooltip label="Visitors who have viewed more than once." position="is-top" append-to-body multilined>
              <b-button label="?" />
            </b-tooltip>
            <a v-if="technology_top_order == 'returning_users_desc'" @click="technology_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'returning_users_asc'" @click="technology_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Total Pageviews
            <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined>
              <b-button label="?" />
            </b-tooltip>
            <a v-if="technology_top_order == 'total_pageviews_desc'" @click="technology_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'total_pageviews_asc'" @click="technology_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Unique Pageviews
            <b-tooltip label="Times the page was viewed by a unique user." position="is-top" append-to-body multilined>
              <b-button label="?" />
            </b-tooltip>
            <a v-if="technology_top_order == 'unique_pageviews_desc'" @click="technology_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
            <a v-else-if="technology_top_order == 'unique_pageviews_asc'" @click="technology_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
            <a v-else @click="technology_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a>
          </th>
          <th colspan="2">Avg. Time
            <b-tooltip label="The average time spent on your page by users." position="is-top" append-to-body multilined>
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
          <td>{{row['Unique Users']}}</td>
          <td><comparison-bar :value="row['Unique Users']" :max="report.technology.data.max['Unique Users']" color="#268699" /></td>
          <td>{{row['New Users']}}</td>
          <td><comparison-bar :value="row['New Users']" :max="report.technology.data.max['New Users']" color="#268699" /></td>
          <td>{{row['Returning Users']}}</td>
          <td><comparison-bar :value="row['Returning Users']" :max="report.technology.data.max['Returning Users']" color="#268699" /></td>
          <td>{{row['Total Pageviews']}}</td>
          <td><comparison-bar :value="row['Total Pageviews']" :max="report.technology.data.max['Total Pageviews']" color="#268699" /></td>
          <td>{{row['Unique Pageviews']}}</td>
          <td><comparison-bar :value="row['Unique Pageviews']" :max="report.technology.data.max['Unique Pageviews']" color="#268699" /></td>
          <td>{{(row['Avg. Time'] / 1000).toFixed(2)}}</td>
          <td><comparison-bar :value="row['Avg. Time']" :max="report.technology.data.max['Avg. Time']" color="#268699" /></td>
        </tr>
      </tbody>
    </table>
  </div>
  </div>

  <div v-else-if="state=='traffic'">

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

    <!-- <table class="data-table"> -->
    <!--   <thead> -->
    <!--     <tr> -->
    <!--       <th>Top Referrers</th> -->
    <!--       <th colspan="2">Unique Users <b-tooltip label="Individual users only counted once." position="is-top" append-to-body multilined> -->
    <!--       <b-button label="?" /> -->
    <!--     </b-tooltip> -->
    <!--         <a v-if="traffic_top_order == 'unique_users_desc'" @click="traffic_top_order = 'unique_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
    <!--         <a v-else-if="traffic_top_order == 'unique_users_asc'" @click="traffic_top_order = 'unique_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
    <!--         <a v-else @click="traffic_top_order = 'unique_users_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
    <!--       </th> -->
    <!--       <th colspan="2">New Users <b-tooltip label="First time visitors." position="is-top" append-to-body multilined> -->
    <!--       <b-button label="?" /> -->
    <!--     </b-tooltip> -->
    <!--         <a v-if="traffic_top_order == 'new_users_desc'" @click="traffic_top_order = 'new_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
    <!--         <a v-else-if="traffic_top_order == 'new_users_asc'" @click="traffic_top_order = 'new_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
    <!--         <a v-else @click="traffic_top_order = 'new_users_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
    <!--       </th> -->
    <!--       <th colspan="2">Returning Users <b-tooltip label="Visitors who have viewed more than once." position="is-top" append-to-body multilined> -->
    <!--       <b-button label="?" /> -->
    <!--     </b-tooltip> -->
    <!--         <a v-if="traffic_top_order == 'returning_users_desc'" @click="traffic_top_order = 'returning_users_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
    <!--         <a v-else-if="traffic_top_order == 'returning_users_asc'" @click="traffic_top_order = 'returning_users_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
    <!--         <a v-else @click="traffic_top_order = 'returning_users_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
    <!--       </th> -->
    <!--       <th colspan="2">Total Pageviews <b-tooltip label="The amount of times your page was viewed." position="is-top" append-to-body multilined> -->
    <!--       <b-button label="?" /> -->
    <!--     </b-tooltip> -->
    <!--         <a v-if="traffic_top_order == 'total_pageviews_desc'" @click="traffic_top_order = 'total_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
    <!--         <a v-else-if="traffic_top_order == 'total_pageviews_asc'" @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
    <!--         <a v-else @click="traffic_top_order = 'total_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
    <!--       </th> -->
    <!--       <th colspan="2">Unique Pageviews <b-tooltip label="Times the page was viewed by a unique user." position="is-top" append-to-body multilined> -->
    <!--       <b-button label="?" /> -->
    <!--     </b-tooltip> -->
    <!--         <a v-if="traffic_top_order == 'unique_pageviews_desc'" @click="traffic_top_order = 'unique_pageviews_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
    <!--         <a v-else-if="traffic_top_order == 'unique_pageviews_asc'" @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
    <!--         <a v-else @click="traffic_top_order = 'unique_pageviews_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
    <!--       </th> -->
    <!--       <th colspan="2">Avg. Time <b-tooltip label="The average time spent on your page by users." position="is-top" append-to-body multilined> -->
    <!--       <b-button label="?" /> -->
    <!--     </b-tooltip> -->
    <!--         <a v-if="traffic_top_order == 'average_time_desc'" @click="traffic_top_order = 'average_time_asc'"><i class="sort sort-asc"><sort-icon /></i></a> -->
    <!--         <a v-else-if="traffic_top_order == 'average_time_asc'" @click="traffic_top_order = 'average_time_desc'"><i class="sort sort-desc"><sort-icon /></i></a> -->
    <!--         <a v-else @click="traffic_top_order = 'average_time_desc'"><i class="sort sortable"><sortable-icon /></i></a> -->
    <!--       </th> -->
    <!--     </tr> -->
    <!--   </thead> -->
    <!--   <tbody> -->
    <!--     <tr v-for="row in traffic_top_sorted"> -->
    <!--       <td>{{row['name']}}</td> -->
    <!--       <td class="table-num">{{row['Unique Users']}}</td> -->
    <!--       <td class="table-bar"><comparison-bar :value="row['Unique Users']" :max="report.traffic.data.max['Unique Users']" color="#268699" /></td> -->
    <!--       <td class="table-num">{{row['New Users']}}</td> -->
    <!--       <td class="table-bar"><comparison-bar :value="row['New Users']" :max="report.traffic.data.max['New Users']" color="#268699" /></td> -->
    <!--       <td class="table-num">{{row['Returning Users']}}</td> -->
    <!--       <td class="table-bar"><comparison-bar :value="row['Returning Users']" :max="report.traffic.data.max['Returning Users']" color="#268699" /></td> -->
    <!--       <td class="table-num">{{row['Total Pageviews']}}</td> -->
    <!--       <td class="table-bar"><comparison-bar :value="row['Total Pageviews']" :max="report.traffic.data.max['Total Pageviews']" color="#268699" /></td> -->
    <!--       <td class="table-num">{{row['Unique Pageviews']}}</td> -->
    <!--       <td class="table-bar"><comparison-bar :value="row['Unique Pageviews']" :max="report.traffic.data.max['Unique Pageviews']" color="#268699" /></td> -->
    <!--       <td class="table-num">{{row['Avg. Time']}}</td> -->
    <!--       <td class="table-bar"><comparison-bar :value="row['Avg. Time']" :max="report.traffic.data.max['Avg. Time']" color="#268699" /></td> -->
    <!--     </tr> -->
    <!--   </tbody> -->
    <!-- </table> -->
  </div>
  </div>
  <div v-else-if="state=='domain'">
    <h2>Engagement Overlap</h2>
    <!-- views are the only interaction we can practicably collect -->
    <!-- <div> -->
    <!--   <b-select :value="report.crossover.data.engagement_type" @input="log('TBD download from server')" v-model="e_type"> -->
    <!--     <option v-for="e_type in report.crossover.engagement_types" :key="e_type" :value="e_type"> -->
    <!--       {{e_type}} -->
    <!--     </option> -->
    <!--   </b-select> -->
    <!-- </div> -->

    <client-only>
      <chord-diagram :chart_data="report.crossover.data.chart" :metric="e_type" />
    </client-only>
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
import PlusIcon from '~/assets/img/plus.svg?inline'
import AtomIcon from '~/assets/img/atom.svg?inline'
import CalendarIcon from '~/assets/img/calendar.svg?inline'
import LikeIcon from '~/assets/img/like.svg?inline'
import ShareIcon from '~/assets/img/share.svg?inline'
import TelescopeIcon from '~/assets/img/astronomy-and-space.svg?inline'
import SaveIcon from '~/assets/img/saved-science-opportunities.svg?inline'


export default {
    name: "MySNMDataOverview",

    components: {
      EyeIcon,
      LinkIcon,
      SortIcon,
      SortableIcon,
      PlusIcon,
      AtomIcon,
      CalendarIcon,
      LikeIcon,
      ShareIcon,
      TelescopeIcon,
      SaveIcon
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

        const report = await context.$axios.$get("/api/ui/organization/analytics", {
            params: {
                about: "00000000-0000-0000-0000-000000000000",
                kind: 0,
                period: "All Time",
                status: "Live and Closed"
            },
            ...context.store.state.auth
        });

        return {
            report,
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
            e_type: "Views"
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
                    borderWidth: 0,
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

        female_pie() {
            const pieces = ["", ""];
            const colors = ["#dedede","#7cb4bf"];
            const fields = ["", ""];

            let ret = {
                labels: pieces,
                datasets: [
                  {
                      label: fields,
                      hoverOffset: 0,
                      backgroundColor: colors,
                      borderWidth:0,
                      data: [1 - this.report.demographics.sex.female.proportion,this.report.demographics.sex.female.proportion],
                  }
                ],
            };
            return ret;
        },

        male_pie() {
            const pieces = ["", ""];
            const colors = ["#dedede","#165e6f"];
            const fields = ["", ""];

            let ret = {
                labels: pieces,
                datasets: [
                  {
                      label: fields,
                      hoverOffset: 0,
                      backgroundColor: colors,
                      borderWidth:0,
                      data: [1 - this.report.demographics.sex.male.proportion,this.report.demographics.sex.male.proportion],
                  }
                ],
            };
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

        sorted_kv(obj) {
            return Object.entries(obj).sort();
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
small {
    font-size: 10pt;
    color: #999;
}

table {
    width: 100%;
    min-width: 300px;

    th.narrow-column,td.narrow-column {
        width: 20rem;
    }
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
    margin: 1em 0px;
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
    
    &.center {
      align-items: center;
    }
    h3 {
      text-align:center;
    }
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

$linework : #dee2e6;
$lightblue: #BFDCE2;
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
  min-width: 45px;
  background-color: $lightblue;
  display: flex;
    justify-content: center;
    align-items: center;
    margin-right: 10px;
    position: relative;

    svg {
      height: 24px;
    }

    &.calendar {
      svg {
        height: 32px;
        width:22px;
      }
      rect {
        fill:transparent;
      }
    }
    strong {
      position: absolute;
      bottom:2px;
      right:10px;
      color: $snm-color-element-med;
      font-size: 12px;
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

.flex2, .flex3 {
  justify-content: space-between;
  > div {
    margin-right:1rem;
    border:1px solid $linework;
    flex-grow: 1;
    flex-basis: 0%;
    padding:1rem;
    margin-bottom: 1rem;
    &:last-child {
      margin-right: 0;
    }
  }
  .data-head {
    margin: -1rem;
    margin-bottom: 1rem;
  }

}

#snm-unique-visitors, #snm-accounts {
  h2 {
    font-size: 2.5rem;
    line-height: 1;
  }
  h3 {
    font-size: 1.2rem;
  }
  svg {
    height: 80px;
    width: 100px;
    margin-bottom: 0.5rem;
}
  }

  .push {
    margin-bottom: 2rem!important;
  }

  .sex.flex2 > div {
      border:0;
      margin-bottom: 0;
  }
  .sex {
    padding-top: 2rem;
  }

  .bar-viz {
    display: flex;
    justify-content: center;
    margin-bottom: 1rem;
    label, span {
      width: 130px;
      font-size: 12px;
      font-weight: bold;
      text-align: right;
    }
    span {
      text-align: left;
      width: 60px;
    }
    
  }

  :deep(.fill .bar-viz .outer){
      background-color: $linework!important;
      margin: 0 8px;
    }


.data-head + .bar-viz {
  margin-top: 1.5rem;
}

.sex {
  .bar-viz {
    justify-content: flex-start;
    label {
      width:50px;
      margin-right: 8px;
    }
    span {
      margin-left: 8px;
      display: block;
    }
  }
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
    .nav-tabs.nav-tabs-alt li {
      width: 200px;
    }
  }
  .nav-tab-wrapper {
    min-width: 840px;
  }

  .data-update {
    margin-top: 2rem;
  }

  .donut {
    text-align: center;
    label {
      margin-bottom: 4px;
      font-weight: bold;
      display: block;
    }
  }
  .donut-first {
    margin-bottom: 4rem;
  }
  .donut-wrap {
    position: relative;
    width:100px;
    height: 100px;
    margin: 0 auto 2rem;
    span {
      font-weight: bold;
      position: absolute;
      top: 39px;
      left: 30px;
    }
  }
  @media (max-width:599px) {
    .engagement-divs {
      .ll-icon {
        border-radius: 100%;
        height:28px;
        width: 28px;
        background-color: $lightblue;
        display: flex;
          justify-content: center;
          align-items: center;
          margin-right: 10px;
          position: relative;
          min-width: 28px;

          svg {
            height: 16px;
          }

          &.calendar {
            svg {
              height: 13px;
              width:15px;
            }
            rect {
              fill:transparent;
            }
          }
          strong {
            position: absolute;
            bottom:2px;
            right:10px;
            color: $snm-color-element-med;
            font-size: 10px;

          }
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
            font-size:14px;
          }
          &.bl-yellow {
            h2, h3 {
              color: $snm-color-action;
            }
          }
        }
    }

  }

  @media (max-width:500px) {
    .audience.flex2 {
      flex-direction: column;
    }
    .donuts {
      display: flex;
      justify-content: space-around;
      .donut {
        margin-bottom: 0;
      }
    }
  }

  @media (max-width:475px){
    .engagement-divs {
      .big-legend {
        flex-direction: column;

        .ll-icon {
          margin-bottom: 4px;
        }
      }
    }
  }
</style>
