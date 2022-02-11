<template>
<div class="widget" :class="[style,layout]">

  <!--
    style (projects):
            header
            no-header
    layout (projects):
            short-thin - shows one project @ 200px wide
            tall-thin - shows two projects stacked @ 200px wide
            short-wide - shows one project @ 375px wide
            tall-wide - shows two projects stacked @ 375px wide
    max (projects):
            [integer] - select a number between 1 and 10??  to limit the amount returned

    layout (finder):
            finder-thin - shows finder @ 200px wide - does not have 'no-header'
            finder-wide - shows finder @ 375px wide - does not have 'no-header'

            combo-thin - shows one project and finder stacked @ 200px wide - not doing for now
            combo-wide - shows one project and finder stacked @ 375px wide - not doing for now
  -->

  <template v-if="style=='header'">
      <div v-if="layout.includes('thin')" class="widget-header thin">
          <img src="~assets/img/logo.svg?data" title="Science Near Me">
      </div>
      <div v-else class="widget-header">
          <img src="~assets/img/logo.svg?data" title="Science Near Me">
          <h1>Opportunities For All Ages!</h1>
      </div>
  </template>

  <div v-if="layout.includes('finder')" class="widget-header">
    <img src="~assets/img/logo.svg?data" title="Science Near Me">
    <h1>Opportunities For All Ages!</h1>
  </div>

  <div class="widget-content">
    <div v-if="!layout.includes('finder')">
      <opportunity-card v-for="opp in opps" :key="opp.uid" :opportunity="opp" widget :widgetlayout="layout" />
    </div>
    <div v-else>
      <general-filters
        widget
        :widget-layout="layout"
        :text="search_text"
        :place="search_place"
        :beginning="search_beginning"
        :ending="search_ending"
        @text="search_text=$event"
        @place="search_place=$event"
        @beginning="search_beginning=$event"
        @ending="search_ending=$event"
        />
    </div>
  </div>


  <div class="powered">
    <template v-if="!layout.includes('finder')">
    See More at <a href="https://sciencenearme.org/">Science Near Me</a>
    </template>
    <template v-else>
    Powered by <a href="https://sciencenearme.org/">Science Near Me</a>
    </template>
  </div>

</div>
</template>

<script>
  export default {
      layout: "empty",

      async asyncData(context) {
          let analytics = null;

          if(process.server) {
              analytics = context.$axios.$post('/api/ui/activity/widget', {'site': context.req.headers['referer']});
          }

          let query = {
              beginning: new Date().toISOString(),
              sample: 'true',
          };

          if(context.route.query['partner']) {
              query['partner'] = context.route.query['partner'];
          }

          if(context.route.query['physical']) {
              query['physical'] = context.route.query['physical'];
          }

          if(context.route.query['max_age']) {
              query['max_age'] = context.route.query['max_age'];
          }

          if(context.route.query['longitude']) {
              query['longitude'] = context.route.query['longitude'];
              query['sort'] = 'closest';
          }

          if(context.route.query['latitude']) {
              query['latitude'] = context.route.query['latitude'];
              query['sort'] = 'closest';
          }

          if(context.route.query['proximity']) {
              query['proximity'] = context.route.query['proximity'];
          }

          if(context.route.query['descriptors'] && context.route.query['descriptors'].length) {
              query['descriptors'] = context.route.query['descriptors'];
          }

          const results = await context.$axios.$get('/api/ui/finder/search', { params: query });

          if(analytics !== null) {
              await analytics;
          }

          return {
              matches: results.matches,
          }
      },

      data() {
          return {
              search_beginning: new Date().toISOString().slice(0, 10),
              search_ending: null,
          };
      },

      computed: {
        style() {
          return this.$route.query.style
        },
        layout() {
          return this.$route.query.layout
        },
        max() {
          if (this.$route.query.max) {
            return this.$route.query.max
          }
          return 1;
        },
        type() {
          return this.$route.query.type
        },
        opps() {
          return this.matches.slice(0,this.max);
        }
      }

    }
</script>

<style lang="scss" scoped>
$border: #D0D0D0;
$header:40px;
$powered:18px;
* {
  box-sizing: border-box;
}
.widget {
  display: flex;
  flex-direction: column;
}
.short-thin, .tall-thin, .finder-thin, .combo-thin {
  width:200px;
  border:1px solid $border;
  border-radius:6px;
}
.short-wide, .tall-wide, .finder-wide, .combo-wide {
  width:375px;
  border:1px solid $border;
  border-radius:6px;
}

.widget-header {
  flex: 0 0 $header;
  max-height: $header;
  background-color:$snm-color-background-medlight;
  position:relative;

  img {
    height: 42px;
    position: absolute;
    top:9px;
    left:9px;
  }

  h1 {
    font-size:14px;
    font-weight:bold;
    font-family:$snm-font-heading;
    padding:10px 0 0 98px;
  }

  &.thin {
    text-align:center;
    img {
      position:relative;
      top:9px;
      left:-3px;
    }
  }
}

.finder-thin {
  .widget-header {
    img {
      height:26px;
    }
    h1 {
      font-size:10px;
      line-height:1.2;
      padding-left:66px;
      margin-top:4px;
    }
  }
  .widget-content {
    padding-top:0;
  }
}


.finder-wide {
  .widget-content {
    padding-top:0;
  }
}

.widget-content {
  flex:1 1 auto;
  padding-top:18px;
  overflow:auto;
}
.no-header .widget-content {
  padding-top:10px;
}

.short-thin.header {
  height: 325px;
  .widget-content {
    max-height:325px - $header - $powered;
  }
}
.short-thin.no-header {
  height:275px;
  .widget-content {
    max-height:275px - $powered;
  }
}
.short-wide.header {
  height:250px;
  .widget-content {
    max-height:250px - $header - $powered;
  }
}
.short-wide.no-header {
  height:200px;
  .widget-content {
    max-height:200px - $powered;
  }
}
.tall-thin.header {
  height:575px;
  .widget-content {
    max-height:575px - $header - $powered;
  }
}
.tall-thin.no-heder {
  height:525px;
  .widget-content {
    max-height:525px - $powered;
  }
}
.tall-wide.header {
  height:475px;
  .widget-content {
    max-height:475px - $header - $powered;
  }
}
.tall-wide.no-header {
  height:425px;
  .widget-content {
    max-height:425px - $powered;
  }
}
.finder-thin {
  height:445px;
  .widget-content {
    max-height:445px - $header - $powered;
    overflow:hidden;
  }
}
.finder-wide {
  height:310px;
  .widget-content {
    max-height:310px - $header - $powered;
    background-color: $snm-color-background-medium;
    overflow:hidden;
  }
}
// .finder-thin.no-header {
//   height:225px;
//   .widget-content {
//     max-height:225px - $powered;
//   }
// }
// .combo-thin.header {
//   height:530px;
//   .widget-content {
//     max-height:530px - $header - $powered;
//   }
// }
// .combo-thin.no-header {
//   height:485px;
//   .widget-content {
//     max-height:485px - $powered;
//   }
// }
// .combo-wide.header {
//   height:515px;
//   .widget-content {
//     max-height:515px - $header - $powered;
//   }
// }
// .combo-wide.no-header {
//   height:460px;
//   .widget-content {
//     max-height:460px - $powered;
//   }
// }

.powered {
  font-size:10px;
  font-weight:bold;
  text-align:center;
  flex:0 0 $powered;
  height:$powered;
  border-top:1px solid $border;
  a {
    text-decoration:underline;
  }
}

</style>
