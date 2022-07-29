<template>
<div class="your-data-overview snm-container">
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
      <li><a class="tab-link" :class="{'active':state=='audience'}" @click="state='audience'">Audience</a></li>
      <li><a class="tab-link" :class="{'active':state=='traffic'}" @click="state='traffic'">Traffic</a></li>
    </ul>
  </div>

  <aside>Date updated: {{updated_local}}</aside>

  <div v-if="state=='engagement'">
    <h2>Engagement</h2>

    <div class="filters">
      <div class="stack">
        <label>Opportunity Status</label>
        <b-select :value="report[org].engagement.data.opportunity_status" @input="console.log('TBD download from server')">
          <option v-for="status in report[org].engagement.opportunity_statuses" :key="status" :value="status">
            {{status}}
          </option>
        </b-select>
      </div>
      <div class="stack">
        <label>Time Period</label>
        <b-select :value="report[org].engagement.data.time_period" @input="console.log('TBD download from server')">
          <option v-for="period in report[org].engagement.time_periods" :key="period" :value="period">
            {{period}}
          </option>
        </b-select>
      </div>
      <div class="extra">
        <a>export csv</a>
      </div>
    </div>

    <line-chart
      :rows="report[org].engagement.data.rows"
      :xaxis="d => new Date(d.date)"
      :yaxes="['Views', 'Unique', 'Clicks to Website']"
      :colors="['#268699', '#BFDCE2', '#FABF40']"
      />

  </div>

  <div v-else-if="state=='audience'">
    <h2>Audience</h2>
  </div>

  <div v-else-if="state=='traffic'">
    <h2>Traffic</h2>
  </div>

</div>
</template>

<script>
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
                            "rows": [
                                {"date": "2022-07-29", "Views": 15, "Unique": 8, "Clicks to Website": 4},
                                {"date": "2022-07-28", "Views": 8, "Unique": 2, "Clicks to Website": 7},
                                {"date": "2022-07-27", "Views": 13, "Unique": 11, "Clicks to Website": 1},
                            ],
                        },
                    },
                },
            }
        }
    },

    data() {
        return {
            state: 'engagement',
            org: "Demo Org",
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
    },

    methods: {

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

aside {
    display: block;
    text-align: right;
    color: #ccc;
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
