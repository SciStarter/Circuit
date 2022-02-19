<template>
<div class="your-opportunities snm-container">
  <div class="flex-header">
    <h1>Your Opportunities</h1>
    <action-button primary><div class="icon"><add-icon /></div>Add a new opportunity</action-button>
  </div>

  <ul class="nav-tabs">
      <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Current, Live Opportunities</a></li>
      <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Draft &amp; Unpublished</a></li>
      <li><a class="tab-link":class="{'active':state==3}" @click="state=3">Past Opportunities</a></li>
      <li class="push-right"><action-button text2>Export Records</action-button></li>
  </ul>

  <div v-if="state==1">
    <div class="flex-header">
      <h2>Current, Live Opportunities</h2>
      <div class="flex header-actions">
        <b-field label="Search" label-position="inside" data-context="find-keywords">
          <b-input ref="search_keywords" v-model="text_proxy" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
        </b-field>
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="beginning_proxy"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="ending_proxy"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
      </div>
    </div>

    <section id="results">
      <template v-if="matches.length > 0">
        <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" owner="live" />
      </template>
      <template v-else>
        <div class="alert no-results" style="margin-bottom:2rem;">
          <p>No live opportunties. Add some!</p>
        </div>
      </template>
    </section>

  </div><!-- state 1 -->

  <div v-if="state==2">
    <div class="flex-header">
      <h2>Draft &amp; Unpublished Opportunities</h2>
      <div class="flex header-actions">
        <b-field label="Search" label-position="inside" data-context="find-keywords">
          <b-input ref="search_keywords" v-model="text_proxy" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
        </b-field>
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="beginning_proxy"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="ending_proxy"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
      </div>
    </div>
    <section id="results">
      <template v-if="matches.length > 0">
        <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" owner="draft" />
      </template>
      <template v-else>
        <div class="alert no-results" style="margin-bottom:2rem;">
          <p>No results.</p>
        </div>
      </template>
    </section>
  </div><!-- state 2 -->

  <div v-if="state==3">
    <div class="flex-header">
      <h2>Past Opportunities</h2>
      <div class="flex header-actions">
        <b-field label="Search" label-position="inside" data-context="find-keywords">
          <b-input ref="search_keywords" v-model="text_proxy" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
        </b-field>
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="beginning_proxy"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="ending_proxy"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
      </div>
    </div>
    <section id="results">
      <template v-if="matches.length > 0">
        <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" owner="past" />
      </template>
      <template v-else>
        <div class="alert no-results" style="margin-bottom:2rem;">
          <p>No results.</p>
        </div>
      </template>
    </section>
  </div><!-- state 3 -->


  <b-modal
    v-model="show_delete_confirm"
    has-modal-card
    trap-focus
    :destroy-on-hide="false"
    aria-role="dialog"
    aria-label="Show tooltip"
    aria-modal
    >
    <div class="card">
      <h2>Confirm Delete <span class="close" @click="show_delete_confirm = false">&times;</span></h2>
      <p>Once deleted, this opportunity and all of its data will be removed from Science Near Me.</p>
      <div>
          <action-button primary>Confirm Delete</action-button>
          <action-button tertiary @click="show_delete_confirm = false">Cancel</action-button>
      </div>

    </div>
  </b-modal>

</div>
</template>

<script>

import AddIcon from '~/assets/img/submit-opportunity.svg?inline'

export default {
  components: {
    AddIcon
  },
  data(){
    return{
      state:1,
      show_delete_confirm: false,
      matches:[{"address_city":"Kenton","address_country":"United States","address_state":"OK","address_street":"","address_zip":"73946","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"  Please Note the first day of the event is now Friday instead of the normal Saturday. Ending on Saturday instead of the normal Sunday. \n\nOklahoma City Astronomy Club host the  Okie-Tex Star Party  which is consistently rated as one of America’s Top Ten Star Parties! Join us on facebook  www.facebook.com/OkieTexStarParty \n\n","end_datetimes":[],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"","is_online":false,"languages":["en-US"],"location_name":"Camp Billy Joe, Kenton Oklahoma","location_point":{"coordinates":[-102.951186,36.896937],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"","organization_type":"club","organization_website":"","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-03-27T18:57:01-05:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=116840","partner_updated":"2021-03-29T18:26:26-05:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"okie-tex-star-party","start_datetimes":[],"tags":[],"ticket_required":false,"title":"Okie-Tex Star Party","uid":"379871eb-1bce-57b2-907a-6eb022ffb24c"},{"address_city":"Mayodan","address_country":"United States","address_state":"NC","address_street":"500 Old Mayo Park Road","address_zip":"27027","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"The Greensboro Astronomy Club in conjunction with the mayo River State Park will host a free public viewing session on Saturday October 2 beginning at dark.&nbsp; Please see the Mayo River Park site for details.","end_datetimes":[],"entity_type":"opportunity","has_end":false,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/1.jpg","is_online":false,"languages":["en-US"],"location_name":"Mayo River State Park","location_point":{"coordinates":[-79.947238,36.436543],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Greensboro Astronomy Club","organization_type":"club","organization_website":"http://www.greensboroastronomyclub.org","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-07-15T09:00:58-04:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118044","partner_updated":"2021-07-15T09:00:58-04:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"stars-and-planets","start_datetimes":[],"tags":[],"ticket_required":false,"title":"Stars and Planets","uid":"e4e43143-ba34-52f2-8fc0-5c03e4956945"},{"address_city":"Michigan City","address_country":"United States","address_state":"MS","address_street":"9714 Highway 72","address_zip":"38647","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"Public Viewing.\n\nTelescope mentoring: New Telescope? Telescope proglems? M.A.S. members are ready to help.","end_datetimes":[],"entity_type":"opportunity","has_end":false,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/MasLogo.gif","is_online":false,"languages":["en-US"],"location_name":"Burton's Sugar Farm","location_point":{"coordinates":[-89.232456,34.945788],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Memphis Astronomical Society","organization_type":"club","organization_website":"http://www.memphisastro.org","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-09-26T19:02:59-05:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118998","partner_updated":"2021-09-26T19:02:59-05:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"burton-s-sugar-farm-observing-event","start_datetimes":[],"tags":[],"ticket_required":false,"title":"Burton's Sugar Farm Observing Event","uid":"ffd9a2ca-ae7a-513e-8d0e-828b0ff6dee2"}],
    }
  }
}

// export default {
//     httpHeaders() {
//         return {
//             'X-XSS-Protection': '1; mode=block',
//             'X-Frame-Options': 'DENY',
//             'X-Content-Type-Options': 'nosniff',
//             'Referrer-Policy': 'same-origin',
//         };
//     },
//
//     async asyncData(context) {
//         const user = await context.store.dispatch('get_user');
//
//         if(!user.authenticated) {
//             context.error({
//                 statusCode: 401,
//                 message: "Authentication required"
//             });
//         }
//
//         let partners = [];
//
//         try {
//             partners = await context.$axios.$get('/api/ui/profile/partners', this.$store.state.auth);
//         }
//         catch(err) {
//             context.error({
//                 statusCode: err.response.status,
//                 message: err.response.data
//             });
//         }
//
//         return {
//             partners,
//         }
//     },
//
//     data() {
//         return {
//             partner_index: 0,
//         }
//     },
//
//     computed: {
//         user() {
//             return this.$store.state.user;
//         },
//
//         choose_partner() {
//             return this.partners.length > 1;
//         },
//
//         selected_partner() {
//             return this.partners[this.partner_index] || null;
//         },
//     },
// }
</script>

<style lang="scss" scoped>
.flex {
  display:flex;
}
.flex-header {
  display:flex;
  align-items:center;
  justify-content:space-between;
  margin-bottom:1rem;

  h2 {
    color: $snm-color-element-med;
    font-size:1.4rem;
    font-weight:bold;
    font-family: $snm-font-heading;
  }
  .datepicker {
    width:150px;
  }
}

h1 {
  font-family: $snm-font-heading;
  font-size: 1.8rem;
  font-weight:bold;
  color: $snm-color-element-med;
  margin-bottom:2rem;
}
.header-actions > div {
  margin-left:1rem;
}
.push-right {
  margin-left:auto;
  font-size:16px!important;
  align-self:center;
}
#results {
  margin-bottom:4rem;
}
</style>
