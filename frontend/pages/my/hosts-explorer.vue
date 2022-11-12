<template>
<div class="hosts-explorer">
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

<div class="tree-wrap">
  <treemap :treemap_data="report.data.hosts" />
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
    <tbody>
      <tr v-for="row in hosts_top_sorted">
        <td>{{row.name}}</td>
        <td class="table-num">{{row.total}}</td>
        <td class="table-bar"><comparison-bar :value="row.total" :max="report.data.max.total" color="#268699" /></td>
        <td class="table-num">{{row.live}}</td>
        <td class="table-bar"><comparison-bar :value="row.live" :max="report.data.max.live" color="#268699" /></td>
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
    <tbody>
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
        return {
            report: {
                "updated": "2022-07-28T14:33:27.12343242-07:00",
                "data": {
                    "total_hosts": 43,
                    "total_opportunities": 4212,
                    "max": {"total": 10345, "live": 10345, "views": 23442, "opportunity_exits": 2313, "didits": 1321, "saves": 1332, "likes": 1331, "shares": 433, "calendar_adds": 132},
                    "hosts": [
                        {"name": "Nerd Nite Atlanta", "total": 10345, "live": 10345, "views": 23442, "opportunity_exits": 2313, "didits": 1321, "saves": 1332, "likes": 1331, "shares": 433, "calendar_adds": 132},
                        {"name": "ASDF 01", "total": 10332, "live": 10331, "views": 23397, "opportunity_exits": 2290, "didits": 1298, "saves": 1219, "likes": 1208, "shares": 400, "calendar_adds": 109},
                        {"name": "ASDF 02", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 03", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 04", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 05", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 06", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 07", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 08", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 09", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 10", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 11", "total": 10332, "live": 10332, "views": 23429, "opportunity_exits": 2300, "didits": 1308, "saves": 1319, "likes": 1318, "shares": 420, "calendar_adds": 119},
                        {"name": "ASDF 12", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 13", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 14", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 15", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 16", "total": 10319, "live": 10319, "views": 23416, "opportunity_exits": 2287, "didits": 1295, "saves": 1306, "likes": 1305, "shares": 407, "calendar_adds": 106},
                        {"name": "ASDF 17", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 18", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 19", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 20", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 21", "total": 10306, "live": 10306, "views": 23403, "opportunity_exits": 2274, "didits": 1282, "saves": 1293, "likes": 1292, "shares": 394, "calendar_adds": 93},
                        {"name": "ASDF 22", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 23", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 24", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 25", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 26", "total": 10293, "live": 10293, "views": 23390, "opportunity_exits": 2261, "didits": 1269, "saves": 1280, "likes": 1279, "shares": 381, "calendar_adds": 80},
                        {"name": "ASDF 27", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 28", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 29", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 30", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 31", "total": 10280, "live": 10280, "views": 23377, "opportunity_exits": 2248, "didits": 1256, "saves": 1267, "likes": 1266, "shares": 368, "calendar_adds": 67},
                        {"name": "ASDF 32", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 33", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 34", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 35", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 36", "total": 10267, "live": 10267, "views": 23364, "opportunity_exits": 2235, "didits": 1243, "saves": 1254, "likes": 1253, "shares": 355, "calendar_adds": 54},
                        {"name": "ASDF 37", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 38", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 39", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 40", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 41", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                        {"name": "ASDF 42", "total": 10254, "live": 10254, "views": 23351, "opportunity_exits": 2222, "didits": 1230, "saves": 1241, "likes": 1240, "shares": 342, "calendar_adds": 41},
                    ],
                },
            },
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
