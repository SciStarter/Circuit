<template>
<div class="hosts-explorer">
  <b-select v-if="available_orgs.length > 1" :value="org" @input="select_org($event)" placeholder="Select Organization">
    <option v-for="avail_org in available_orgs" :value="avail_org" :key="avail_org.uid">
      {{avail_org.name}}
    </option>
  </b-select>

  <div class="flex-header">
    <h1>Hosts Explorer</h1>
  </div>

  <div class="flex hosts-header">
    <div><strong>{{report.data.total_hosts}}</strong><span>Hosts</span></div>
    <div><strong>{{report.data.total_opportunities}}</strong><span>Opportunities All Time</span></div>
  </div>
  <!-- <div class="area-display data-wrapper">
    <div class="area-column" v-for="chunk in [2, 4, 8, 16]">
      <div v-for="host in hosts_chunk(chunk)" class="area-host" :style="{'height': host.proportion}">
        {{host.name}} ({{host.value}})
      </div>
    </div>
  </div> -->

  <div class="tree-wrap" v-if="report.data.total_hosts > 0">
    <client-only>
      <treemap :treemap_data="report.data.hosts" />
    </client-only>
</div>
  <div class="data-table-wrapper">
  <table class="data-table">
    <thead>
      <tr>
        <th>Host</th>
        <th colspan="2">Total Opportunities <b-tooltip label="# of live and closed opportunities of the host." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="hosts_top_order == 'total_desc'" @click="hosts_top_order = 'total_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="hosts_top_order == 'total_asc'" @click="hosts_top_order = 'total_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="hosts_top_order = 'total_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Live Opportunities <b-tooltip label="# of opportunities of the host currently on SNM." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="hosts_top_order == 'live_desc'" @click="hosts_top_order = 'live_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="hosts_top_order == 'live_asc'" @click="hosts_top_order = 'live_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="hosts_top_order = 'live_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
      </tr>
    </thead>
    <tbody v-if="hosts_top_sorted.length > 0">
      <tr v-for="row in hosts_top_sorted">
        <td>{{row.name}}</td>
        <td class="table-num">{{row.total}}</td>
        <td class="table-bar"><comparison-bar :value="row.total" :max="report.data.max.total" color="#268699" /></td>
        <td class="table-num">{{row.live}}</td>
        <td class="table-bar"><comparison-bar :value="row.live" :max="report.data.max.live" color="#268699" /></td>
      </tr>
    </tbody>
    <tbody v-else>
      <tr>
        <td colspan="5">No Data to Display</td>
      </tr>
    </tbody>
  </table>
</div>

  <div class="data-table-wrapper">
  <table class="data-table">
    <thead>
      <tr>
        <th>Host</th>
        <th colspan="2">Views <b-tooltip label="Total number of times opportunities of host have been viewed." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'views_desc'" @click="engagement_top_order = 'views_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'views_asc'" @click="engagement_top_order = 'views_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'views_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Clicks to Website <b-tooltip label="Total number of times users have clicked on the website link of opportunities of host." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'opportunity_exits_desc'" @click="engagement_top_order = 'opportunity_exits_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'opportunity_exits_asc'" @click="engagement_top_order = 'opportunity_exits_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'opportunity_exits_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Self-Reports <b-tooltip label="Total number of users have reported doing opportunities of host." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'didits_desc'" @click="engagement_top_order = 'didits_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'didits_asc'" @click="engagement_top_order = 'didits_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'didits_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Saves <b-tooltip label="Total number of times users have saved opportunities of host." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'saves_desc'" @click="engagement_top_order = 'saves_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'saves_asc'" @click="engagement_top_order = 'saves_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'saves_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Likes <b-tooltip label="Total number of times users have liked opportunities of host." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'likes_desc'" @click="engagement_top_order = 'likes_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'likes_asc'" @click="engagement_top_order = 'likes_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'likes_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Shares <b-tooltip label="Total number of times users have shared the opportunities of host using our share button." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'shares_desc'" @click="engagement_top_order = 'shares_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'shares_asc'" @click="engagement_top_order = 'shares_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'shares_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
        <th colspan="2">Calendar Adds <b-tooltip label="Total number of times users have added opportunities of host to their calendar using our add to calendar button." position="is-top" append-to-body multilined>
          <b-button label="?" />
        </b-tooltip>
          <a v-if="engagement_top_order == 'calendar_adds_desc'" @click="engagement_top_order = 'calendar_adds_asc'"><i class="sort sort-asc"><sort-icon /></i></a>
          <a v-else-if="engagement_top_order == 'calendar_adds_asc'" @click="engagement_top_order = 'calendar_adds_desc'"><i class="sort sort-desc"><sort-icon /></i></a>
          <a v-else @click="engagement_top_order = 'calendar_adds_desc'"><i class="sort sortable"><sortable-icon /></i></a>
        </th>
      </tr>
    </thead>
    <tbody v-if="engagement_top_sorted.length > 0">
      <tr v-for="row in engagement_top_sorted">
        <td>{{row.name}}</td>
        <td class="table-num">{{row.views}}</td>
        <td class="table-bar"><comparison-bar :value="row.views" :max="report.data.max.views" color="#268699" /></td>
        <td class="table-num">{{row.opportunity_exits}}</td>
        <td class="table-bar"><comparison-bar :value="row.opportunity_exits" :max="report.data.max.opportunity_exits" color="#268699" /></td>
        <td class="table-num">{{row.didits}}</td>
        <td class="table-bar"><comparison-bar :value="row.didits" :max="report.data.max.didits" color="#268699" /></td>
        <td class="table-num">{{row.saves}}</td>
        <td class="table-bar"><comparison-bar :value="row.saves" :max="report.data.max.saves" color="#268699" /></td>
        <td class="table-num">{{row.likes}}</td>
        <td class="table-bar"><comparison-bar :value="row.likes" :max="report.data.max.likes" color="#268699" /></td>
        <td class="table-num">{{row.shares}}</td>
        <td class="table-bar"><comparison-bar :value="row.shares" :max="report.data.max.shares" color="#268699" /></td>
        <td class="table-num">{{row.calendar_adds}}</td>
        <td class="table-bar"><comparison-bar :value="row.calendar_adds" :max="report.data.max.calendar_adds" color="#268699" /></td>
      </tr>
    </tbody>
    <tbody v-else>
      <tr>
        <td colspan="13">No Data to Display</td>
      </tr>
    </tbody>
  </table>
</div>
</div>
</template>

<script>

function cmp(k, a, b) {
    if(a[k] > b[k]) {
        return 1;
    }
    else if(a[k] < b[k]) {
        return -1;
    }
    else {
        return 0;
    }
}

import SortIcon from '~/assets/img/sort.svg?inline'
import SortableIcon from '~/assets/img/sortable.svg?inline'

export default {
    name: "MyHostsExplorer",

    components: {
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

        const available_orgs = await context.$axios.$get("/api/ui/organization/all", context.store.state.auth);
        const org = available_orgs[0];
        const report = await context.$axios.$get("/api/ui/organization/analytics", {
            params: {
                about: org.uid,
                kind: 1,
                period: "This Month",
                status: "Live and Closed"
            },
            ...context.store.state.auth
        });

        return {
            available_orgs,
            org,
            report,
        };
    },

    data() {
        return {
            hosts_top_order: "total_desc",
            engagement_top_order: "views_desc",
        };
    },

    computed: {
        hosts_top_sorted() {
            switch(this.hosts_top_order) {
                case "total_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('total', a, b));
                case "total_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('total', a, b));
                case "live_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('live', a, b));
                case "live_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('live', a, b));
                default:
                return this.report.data.hosts;
            }
        },

        engagement_top_sorted() {
            switch(this.engagement_top_order) {
                case "views_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('views', a, b));
                case "views_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('views', a, b));
                case "opportunity_exits_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('opportunity_exits', a, b));
                case "opportunity_exits_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('opportunity_exits', a, b));
                case "didits_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('didits', a, b));
                case "didits_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('didits', a, b));
                case "saves_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('saves', a, b));
                case "saves_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('saves', a, b));
                case "likes_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('likes', a, b));
                case "likes_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('likes', a, b));
                case "shares_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('shares', a, b));
                case "shares_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('shares', a, b));
                case "calendar_adds_asc":
                return this.report.data.hosts.slice().sort((a, b) => cmp('calendar_adds', a, b));
                case "calendar_adds_desc":
                return this.report.data.hosts.slice().sort((a, b) => -cmp('calendar_adds', a, b));
                default:
                return this.report.data.hosts;
            }
        },
    },

    methods: {
        async select_org(org) {
            const loading = this.$buefy.loading.open({container: null});
            this.report = await this.$axios.$get("/api/ui/organization/analytics", {
                params: {
                    about: org.uid,
                    kind: 1,
                    period: "This Month",
                    status: "Live and Closed"
                },
                ...this.$store.state.auth
            });
            this.org = org;
            loading.close();
        },

        hosts_chunk(count) {
            const begin = count - 2;
            const end = begin + count;
            const hosts = this.report.data.hosts.slice(begin, end);
            const total = hosts.reduce((total, item) => total + item.views, 0);
            return hosts.map(host => ({
                name: host.name,
                value: host.views,
                proportion: '' + (100 * host.views / total) + '%',
            }));
        }
    },
}
</script>

<style lang="scss" scoped>
.area-display {
    display: flex;

    > .area-column {
        height: 800px;
        display: flex;
        flex-direction: column;

        > .area-host {
            display: flex;
            width: 300px;
            align-items: center;
            justify-content: center;
            background-color: #268699;
            color: #fff;
            border: 1px solid #fff;
        }
    }
}

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

$linework : #dee2e6;
$lightblue: #BFDCE2;

.data-wrapper, .data-table {
  border: 1px solid $linework;
  margin-bottom: 2rem;
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

  .hosts-header {
    font-weight: bold;
    color: $snm-color-element-med;
    > div {
      margin-right: 1.2rem;
    }
    strong {
     font-size: 2rem;
     color: $snm-color-element-med;
     margin-right: .33rem;
    }
  }

  .tree-wrap {
    margin-bottom: 2rem;
  }

</style>
