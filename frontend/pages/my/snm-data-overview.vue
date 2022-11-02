<template>
<div class="your-data-overview snm-container">
  <div class="flex-header">
    <h1>SNM Data Overview</h1>
  </div>

  <div class="nav-tab-wrapper">
    <ul class="nav-tabs">
      <li><a class="tab-link" :class="{'active':state=='engagement'}" @click="state='engagement'">Engagement</a></li>
      <li><a class="tab-link" :class="{'active':state=='states'}" @click="state='states'">Audience</a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'">Traffic</a></li>
      <li><a class="tab-link" :class="{'active':state=='domain'}" @click="state='domain'">Domain Insights</a></li>
    </ul>
  </div>

  <aside>Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">
    <div id="snm-unique-visitors">
      {{report.engagement.data.stats.unique_visitors}}
      <label>Unique SNM Visitors</label>
    </div>
    <div id="snm-accounts">
      {{report.engagement.data.stats.accounts}}
      <label>SNM Accounts</label>
    </div>
    <div id="snm-total-opp-page-views">
      {{report.engagement.data.stats.opportunity_views}}
      <label>Total Opportunity Page Views</label>
    </div>
    <div id="snm-total-opp-unique">
      {{report.engagement.data.stats.opportunity_unique}}
      <label>Unique Opportunity Page Views</label>
    </div>
    <div id="snm-website-clicks">
      {{report.engagement.data.stats.opportunity_exits}}
      <label>Website4 Clicks</label>
    </div>
    <div id="snm-self-reports">
      {{report.engagement.data.stats.didits}}
      <label>Self-Reports</label>
    </div>
    <div id="snm-saves">
      {{report.engagement.data.stats.saves}}
      <label>Saves</label>
    </div>
    <div id="snm-likes">
      {{report.engagement.data.stats.likes}}
      <label>Likes</label>
    </div>
    <div id="snm-shares">
      {{report.engagement.data.stats.shares}}
      <label>Shares</label>
    </div>
    <div id="snm-calendar-adds">
      {{report.engagement.data.stats.calendar_adds}}
      <label>Calendar Adds</label>
    </div>

    <table>
      <tr>
        <th class="narrow-column">Top 30 Searches by Keyword</th>
        <th>Total Searches</th>
      </tr>
      <tr v-for="row in report.engagement.data.searches">
        <td class="narrow-column">{{row.phrase}}</td>
        <td><comparison-bar :value="row.searches" :max="report.engagement.data.search_max" color="#7CB4BF" background="#DEDEDE" width="100%" height="1rem" /></td>
      </tr>
    </table>
  </div>

  <div v-else-if="state=='states'">
    <div>
      <h2>Sex &amp; Age</h2> <!-- I used "sex" instead of "gender" since we're not talking about gay or straight, trans or cis, ace, aro, etc. -->
      <div>
        <label>Female <comparison-bar :value="report.demographics.sex.female.proportion" :max="1.0" color="#7CB4BF" background="#DEDEDE" width="100%" height="1rem" /></label>
        <label>Male <comparison-bar :value="report.demographics.sex.male.proportion" :max="1.0" color="#165E6F" background="#DEDEDE" width="100%" height="1rem" /></label>
      </div>
      <div>
        <label v-for="entry in sorted_kv(report.demographics.age)">
          {{entry[0]}} {{percent(entry[1].proportion)}}
          <comparison-bar :value="entry[1].male.proportion" :max="entry[1].proportion" color="#165E6F" background="#7CB4BF" width="100%" height="1rem" />
        </label>
      </div>
    </div>

    <div>
      <h2>Ethnicity</h2>
      <label>
        Caucasian
        <comparison-bar :value="report.demographics.ethnicity['Cauc.'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        Hispanic
        <comparison-bar :value="report.demographics.ethnicity['Hisp'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        African American
        <comparison-bar :value="report.demographics.ethnicity['Afr. Am.'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        Asian
        <comparison-bar :value="report.demographics.ethnicity['Asian'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        Other
        <comparison-bar :value="report.demographics.ethnicity['Other'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
    </div>

    <div>
      <h2>Education</h2>
      <label>
        No College
        <comparison-bar :value="report.demographics.education['No College'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        College
        <comparison-bar :value="report.demographics.education['College'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        Grad School
        <comparison-bar :value="report.demographics.education['Grad. Sch.'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
    </div>

    <div>
      <h2>Household Income</h2>
      <label>
        $0-50k
        <comparison-bar :value="report.demographics.income['$0-50k'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        $50-100k
        <comparison-bar :value="report.demographics.income['$50-100k'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        $100-150k
        <comparison-bar :value="report.demographics.income['$100-150k'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        $150k+
        <comparison-bar :value="report.demographics.income['$150k+'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
    </div>

    <div>
      <h2>Children</h2>
      <label>
        Has Kids
        <comparison-bar :value="report.demographics.children['Some Children under 17'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
      <label>
        No Kids
        <comparison-bar :value="report.demographics.children['No Children under 17'].proportion" :max="1.0" color="#165E6F" width="100%" height="1rem" />
      </label>
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
  <div v-else-if="state=='domain'">
    <h2>Engagement Overlap</h2>
    <div>
      <b-select :value="report.crossover.data.engagement_type" @input="log('TBD download from server')">
        <option v-for="e_type in report.crossover.engagement_types" :key="e_type" :value="e_type">
          {{e_type}}
        </option>
      </b-select>
    </div>

    <div style="box-shadow: 2px 2px 4px #999 inset; font-size: 18pt; margin: 2rem; padding: 2rem; border: 1px solid #999;">
      I don't know how to render this chart, but the data to drive it
      are in <code>report.crossover.data.chart</code>. Over to you,
      Kevin.
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

export default {
    name: "MySNMDataOverview",

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
                "updated": "2022-07-28T14:33:27.12343242-07:00",
                "engagement": {
                    "data": {
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "search_max": 41363,
                        "stats": {
                            "unique_visitors": 5732,
                            "accounts": 1112,
                            "opportunity_views": 4214,
                            "opportunity_unique": 3214,
                            "opportunity_exits": 2341,
                            "didits": 123,
                            "saves": 632,
                            "likes": 423,
                            "shares": 343,
                            "calendar_adds": 211,
                        },
                        "searches": [
                            {"phrase": "Science Festival", "searches": 41363},
                            {"phrase": "mumblety-peg", "searches": 8123},
                            {"phrase": "kids and families", "searches": 712}
                        ],
                    },
                },

                "demographics": {
                    "sex": {
                        "male": {"index": 92, "proportion": 0.4507, "national": 0.4915},
                        "female": {"index": 108, "proportion": 0.5493, "national": 0.5085},
                    },
                    "age": {
                        "18-20": {"index": 96, "proportion": 0.0539, "national": 0.0562, "male": {"index": 108, "proportion": 0.0273, "national": 0.0253}, "female": {"index": 86, "proportion": 0.0266, "national": 0.0309}},
                        "21-24": {"index": 86, "proportion": 0.0679, "national": 0.0788, "male": {"index": 87, "proportion": 0.0307, "national": 0.0355}, "female": {"index": 86, "proportion": 0.0372, "national": 0.0433}},
                        "25-29": {"index": 135, "proportion": 0.1358, "national": 0.1005, "male": {"index": 105, "proportion": 0.0474, "national": 0.0453}, "female": {"index": 160, "proportion": 0.0884, "national": 0.0552}},
                        "30-34": {"index": 112, "proportion": 0.1149, "national": 0.1025, "male": {"index": 116, "proportion": 0.0535, "national": 0.0462}, "female":{"index": 109, "proportion": 0.0615, "national": 0.0563}},
                        "35-39": {"index": 112, "proportion": 0.1305, "national": 0.117, "male": {"index": 95, "proportion": 0.0501, "national": 0.0527}, "female": {"index": 125, "proportion": 0.0804, "national": 0.0642}},
                        "40-44": {"index": 101, "proportion": 0.1187, "national": 0.118, "male": {"index": 87, "proportion": 0.0463, "national": 0.0532}, "female": {"index": 112, "proportion": 0.0725, "national": 0.0648}},
                        "45-49": {"index": 94, "proportion": 0.1017, "national": 0.1076, "male": {"index": 104, "proportion": 0.0505, "national": 0.0485}, "female": {"index": 87, "proportion": 0.0512, "national": 0.0591}},
                        "50-54": {"index": 100, "proportion": 0.1093, "national": 0.1092, "male": {"index": 118, "proportion": 0.058, "national": 0.0492}, "female": {"index": 85, "proportion": 0.0512, "national": 0.06}},
                        "55-59": {"index": 89, "proportion": 0.058, "national": 0.0654, "male": {"index": 111, "proportion": 0.0326, "national": 0.0295}, "female": {"index": 71, "proportion": 0.0254, "national": 0.0359}},
                        "60-64": {"index": 76, "proportion": 0.0505, "national": 0.0668, "male": {"index": 92, "proportion": 0.0277, "national": 0.0301}, "female": {"index": 62, "proportion": 0.0228, "national": 0.0367}},
                        "65+": {"index": 75, "proportion": 0.0588, "national": 0.0781, "male": {"index": 75, "proportion": 0.0266, "national": 0.0352}, "female": {"index": 75, "proportion": 0.0322, "national": 0.0429}},
                    },
                    "education": {
                        "No College": {"index": 85, "proportion": 0.3642, "national": 0.4306},
                        "College": {"index": 103, "proportion": 0.4279, "national": 0.4158},
                        "Grad. Sch.": {"index": 135, "proportion": 0.2079, "national": 0.1536},
                    },
                    "income": {
                        "$0-50k": {"index": 84, "proportion": 0.3365, "national": 0.4028},
                        "$50-100k": {"index": 116, "proportion": 0.3657, "national": 0.3139},
                        "$100-150k": {"index": 114, "proportion": 0.1741, "national": 0.1532},
                        "$150k+": {"index": 95, "proportion": 0.1237, "national": 0.1301},
                    },
                    "children": {
                        "No Children under 17": {"index": 103, "proportion": 0.5247, "national": 0.5071},
                        "Some Children under 17": {"index": 96, "proportion": 0.4753, "national": 0.4929},
                    },
                    "ethnicity": {
                        "Cauc.": {"index": 98, "proportion": 0.7362, "national": 0.7506},
                        "Afr. Am.": {"index": 103, "proportion": 0.0961, "national": 0.0936},
                        "Asian": {"index": 140, "proportion": 0.0614, "national": 0.0437},
                        "Hisp": {"index": 99, "proportion": 0.0968, "national": 0.0978},
                        "Other": {"index": 66, "proportion": 0.0094, "national": 0.0143},
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

                "crossover": {
                    "engagement_types": ["Views", "Unique", "Clicks to Website"],
                    "data": {
                        "engagement_type": "Views",
                        "chart": {
                            "citizen_science": {
                                "proportion": 0.23,
                                "live_science": {"Views": 0.166, "Unique": 0.5, "Clicks to Website": 0.333},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.05, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.166, "Unique": 0.04, "Clicks to Website": 0.333},
                                "policy": {"Views": 0.166, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.4, "Clicks to Website": 0.333},
                                "formal_education": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications": {"Views": 0.166, "Unique": 0.01, "Clicks to Website": 0.0},
                                "unspecified": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                            "live_science": {
                                "proportion": 0.05,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "museum_or_science_center": {
                                "proportion": 0.17,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "maker": {
                                "proportion": 0.21,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "policy": {
                                "proportion": 0.08,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "out_of_school_time_program": {
                                "proportion": 0.22,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "formal_education": {
                                "proportion": 0.0,
                                "citizen_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "live_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "museum_or_science_center": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "policy": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "unspecified": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                            "science_communications": {
                                "proportion": 0.04,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "unspecified": {
                                "proportion": 0.0,
                                "citizen_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "live_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "museum_or_science_center": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "policy": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "formal_education": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications,": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
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
