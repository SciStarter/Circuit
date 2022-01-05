<template>
<div class="widget" :class="[style,layout]">

  <!--
    style:  header
            no-header
    layout: one-thin - shows one project @ 200px wide
            two-thin - shows two projects stacked @ 200px wide
            one-wide - shows one project @ 375px wide
            two-wide - shows two projects stacked @ 375px wide
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
        <general-filters widget :widgetLayout="layout" />
    </div>
  </div>


  <div class="powered">
    Powered by <a href="https://sciencenearme.org">Science Near Me</a>
  </div>

</div>
</template>

<script>
//////// REMOVE /////////////////////////////////////////
function randomIntFromInterval(min, max) { // min and max included
  return Math.floor(Math.random() * (max - min + 1) + min)
}
//////// END REMOVE /////////////////////////////////////////

  export default {
      layout: "empty",
      data() {
        return {
          //////// REMOVE /////////////////////////////////////////
          matches: [{"address_city":"Kenton","address_country":"United States","address_state":"OK","address_street":"","address_zip":"73946","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"  Please Note the first day of the event is now Friday instead of the normal Saturday. Ending on Saturday instead of the normal Sunday. \n\nOklahoma City Astronomy Club host the  Okie-Tex Star Party  which is consistently rated as one of America’s Top Ten Star Parties! Join us on facebook  www.facebook.com/OkieTexStarParty \n\n","end_datetimes":[],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"","is_online":false,"languages":["en-US"],"location_name":"Camp Billy Joe, Kenton Oklahoma","location_point":{"coordinates":[-102.951186,36.896937],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"","organization_type":"club","organization_website":"","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-03-27T18:57:01-05:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=116840","partner_updated":"2021-03-29T18:26:26-05:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"okie-tex-star-party","start_datetimes":[],"tags":[],"ticket_required":false,"title":"Okie-Tex Star Party","uid":"379871eb-1bce-57b2-907a-6eb022ffb24c"},{"address_city":"Mayodan","address_country":"United States","address_state":"NC","address_street":"500 Old Mayo Park Road","address_zip":"27027","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"The Greensboro Astronomy Club in conjunction with the mayo River State Park will host a free public viewing session on Saturday October 2 beginning at dark.&nbsp; Please see the Mayo River Park site for details.","end_datetimes":[],"entity_type":"opportunity","has_end":false,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/1.jpg","is_online":false,"languages":["en-US"],"location_name":"Mayo River State Park","location_point":{"coordinates":[-79.947238,36.436543],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Greensboro Astronomy Club","organization_type":"club","organization_website":"http://www.greensboroastronomyclub.org","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-07-15T09:00:58-04:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118044","partner_updated":"2021-07-15T09:00:58-04:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"stars-and-planets","start_datetimes":[],"tags":[],"ticket_required":false,"title":"Stars and Planets","uid":"e4e43143-ba34-52f2-8fc0-5c03e4956945"},{"address_city":"Michigan City","address_country":"United States","address_state":"MS","address_street":"9714 Highway 72","address_zip":"38647","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"Public Viewing.\n\nTelescope mentoring: New Telescope? Telescope proglems? M.A.S. members are ready to help.","end_datetimes":[],"entity_type":"opportunity","has_end":false,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/MasLogo.gif","is_online":false,"languages":["en-US"],"location_name":"Burton's Sugar Farm","location_point":{"coordinates":[-89.232456,34.945788],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Memphis Astronomical Society","organization_type":"club","organization_website":"http://www.memphisastro.org","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-09-26T19:02:59-05:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118998","partner_updated":"2021-09-26T19:02:59-05:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"burton-s-sugar-farm-observing-event","start_datetimes":[],"tags":[],"ticket_required":false,"title":"Burton's Sugar Farm Observing Event","uid":"ffd9a2ca-ae7a-513e-8d0e-828b0ff6dee2"},{"address_city":"Rocksprings","address_country":"United States","address_state":"TX","address_street":"101 N. Sweeten Street","address_zip":"78880","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"","end_datetimes":["2021-10-02T22:30:00-06:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/SAAA_Logo.jpg","is_online":false,"languages":["en-US"],"location_name":"Devil's Sinkhole State Park","location_point":{"coordinates":[-100.208655,30.015827],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"San Antonio Astronomical Association","organization_type":"club","organization_website":"http://www.sanantonioastronomy.org","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-05-13T08:45:07-05:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=117219","partner_updated":"2021-05-13T08:45:46-05:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"devil-s-sinkhole-50th-anniversary-star-party","start_datetimes":["2021-10-02T20:00:00-06:00"],"tags":[],"ticket_required":false,"title":"Devil's Sinkhole 50th Anniversary Star Party","uid":"b616a58a-9cc1-5004-8cf7-5ddf22baa341"},{"address_city":"Goddard","address_country":"United States","address_state":"KS","address_street":"25000 West 39th South","address_zip":"67052","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"The Lake Afton Public Observatory is open to the public on Friday and Saturday nights. Some club members will set up their telescopes on the north side of the building and visitors are welcome to come observe with them. Visitors are also welcome to bring their own telescope to view with us. If you are having trouble setting up your telescope bring it with you and we would gladly help.\n\nUnder the dome of LAPO is a 16\" telescope for viewing distant objects light years away or planets in our solar system. There are many displays and hands on activities to explore the outer regions of space.","end_datetimes":["2021-10-01T22:30:00-06:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/KAOLOGO2_[Converted].jpg","is_online":false,"languages":["en-US"],"location_name":"Lake Afton Public Observatory (LAPO)","location_point":{"coordinates":[-97.626945,37.622236],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Kansas Astronomical Observers","organization_type":"club","organization_website":"https://www.kaowichita.com/","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-03-12T18:51:26-06:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=116561","partner_updated":"2021-03-12T18:51:49-06:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"lake-afton-public-observatory","start_datetimes":["2021-10-01T20:30:00-06:00"],"tags":[],"ticket_required":false,"title":"Lake Afton Public Observatory","uid":"357378d1-7f28-5404-9276-07f560d45647"},{"address_city":"Lagrange","address_country":"United States","address_state":"OH","address_street":"13630 Diagonal Rd","address_zip":"","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"Observing the night sky through club telescopes.","end_datetimes":["2021-10-01T22:00:00-05:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"","is_online":false,"languages":["en-US"],"location_name":"Nielsen Observatory","location_point":{"coordinates":[-82.145262,41.28782],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Black River Astronomical Society","organization_type":"club","organization_website":"http://www.blackriverastro.org/","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-09-03T19:40:56-04:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118602","partner_updated":"2021-09-03T19:40:56-04:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"bras-public-observing","start_datetimes":["2021-10-01T20:00:00-05:00"],"tags":[],"ticket_required":false,"title":"BRAS public observing.","uid":"794b6127-6022-5529-986b-f47319ab0386"},{"address_city":"West Chester","address_country":"United States","address_state":"PA","address_street":"1760 Unionville Wawaset RdWest Chester, PA 19382","address_zip":"19382","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"Club members and guests are invited to Brandywine Red Clay Alliance to view the night sky.","end_datetimes":["2021-10-01T22:00:00-05:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/logo10.jpg","is_online":false,"languages":["en-US"],"location_name":"Brandywine Red Clay Alliance","location_point":{"coordinates":[-75.680909,39.912929],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Chester County Astronomical Society","organization_type":"club","organization_website":"http://www.ccas.us","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-09-23T21:28:50-04:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118986","partner_updated":"2021-09-23T21:28:50-04:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"october-brc-observing-for-club-members","start_datetimes":["2021-10-01T19:00:00-05:00"],"tags":[],"ticket_required":false,"title":"October BRC observing for club members","uid":"495f2b37-483e-57c1-b2bb-90e26328c798"},{"address_city":"Willcox","address_country":"United States","address_state":"AZ","address_street":"12856 E Rhyolite Creek Road","address_zip":"85643","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"Tucson Amateur Astronomy Association will host an evening star party at the Chiricahua National Monument. We will observe planets, galaxies, star clusters, nebulae, and lots more. The star party will take place at either Faraway Ranch or Echo Canyon Trail head. FREE","end_datetimes":["2021-10-02T21:00:00-07:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"","is_online":false,"languages":["en-US"],"location_name":"Chiricahua National Monument","location_point":{"coordinates":[-109.361378,32.007886],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Tucson Amateur Astronomy Association","organization_type":"club","organization_website":"http://www.tucsonastronomy.org","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-07-17T09:11:15-07:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=118069","partner_updated":"2021-07-17T09:11:42-07:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"chiricahua-national-monument-star-party","start_datetimes":["2021-10-02T19:00:00-07:00"],"tags":[],"ticket_required":false,"title":"Chiricahua National Monument Star Party","uid":"70f9ef45-392a-5152-91a3-c0306295ae57"},{"address_city":"Goddard","address_country":"United States","address_state":"KS","address_street":"25000 West 39th South","address_zip":"67052","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"The Lake Afton Public Observatory is open to the public on Friday and Saturday nights. Some club members will set up their telescopes on the north side of the building and visitors are welcome to come observe with them. Visitors are also welcome to bring their own telescope to view with us. If you are having trouble setting up your telescope bring it with you and we would gladly help.\n\nUnder the dome of LAPO is a 16\" telescope for viewing distant objects light years away or planets in our solar system. There are many displays and hands on activities to explore the outer regions of space.","end_datetimes":["2021-10-02T22:30:00-06:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/KAOLOGO2_[Converted].jpg","is_online":false,"languages":["en-US"],"location_name":"Lake Afton Public Observatory (LAPO)","location_point":{"coordinates":[-97.626945,37.622236],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Kansas Astronomical Observers","organization_type":"club","organization_website":"https://www.kaowichita.com/","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-03-12T18:52:33-06:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=116566","partner_updated":"2021-03-12T18:52:56-06:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"lake-afton-public-observatory-1","start_datetimes":["2021-10-02T20:30:00-06:00"],"tags":[],"ticket_required":false,"title":"Lake Afton Public Observatory","uid":"c676fdb9-8251-5dab-b703-a6d9944aec12"},{"address_city":"Lima","address_country":"United States","address_state":"OH","address_street":"670 Jefferson St","address_zip":"45802","attraction_hours":{"friday":null,"monday":null,"saturday":null,"sunday":null,"thursday":null,"tuesday":null,"wednesday":null},"cost":"free","description":"May meeting of the Lima Astronomical Society&nbsp; at Schoonover Observatory.","end_datetimes":["2021-10-01T23:00:00-05:00"],"entity_type":"opportunity","has_end":true,"image_credit":"","image_url":"https://nightsky.jpl.nasa.gov/club/logos/LAS_Logo.jpg","is_online":false,"languages":["en-US"],"location_name":"Schoonover Observatory","location_point":{"coordinates":[-84.098655,40.747636],"type":"Point"},"location_polygon":{},"location_type":"at","max_age":999,"min_age":0,"opp_descriptor":["star_party"],"opp_hashtags":[],"opp_social_handles":{},"opp_topics":[],"opp_venue":[],"organization_logo_url":null,"organization_name":"Lima Astronomical Society","organization_type":"club","organization_website":"http://www.limaastro.com","partner":"a844e7ee-6417-5bbc-b97c-f85575836442","partner_created":"2021-05-12T23:10:16-04:00","partner_logo_url":null,"partner_name":"Night Sky Network","partner_opp_url":"https://nightsky.jpl.nasa.gov/event-view.cfm?Event_ID=117198","partner_updated":"2021-05-12T23:11:35-04:00","partner_website":null,"pes_domain":"unspecified","short_desc":"","slug":"las-monthly-meeting","start_datetimes":["2021-10-01T20:00:00-05:00"],"tags":[],"ticket_required":false,"title":"LAS Monthly Meeting","uid":"19f3b0b4-8e4c-54f6-b4bd-0441bb44e485"}]
          //////// END REMOVE /////////////////////////////////////////
        }
      },
      computed: {
        style() {
          return this.$route.query.style
        },
        layout() {
          return this.$route.query.layout
        },
        type() {
          return this.$route.query.type
        },
        //////// REMOVE /////////////////////////////////////////
        opps() {
          // HELPS GET ME OPPORTUNITIES IN LOCAL
          let rand = randomIntFromInterval(1,9);
          if (this.layout == 'two-thin' || this.layout == 'two-wide') {
            return this.matches.slice(rand - 2,rand);
          } else {
            return this.matches.slice(rand - 1,rand);
          }
        }
        //////// END REMOVE /////////////////////////////////////////
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
.one-thin, .two-thin, .finder-thin, .combo-thin {
  width:200px;
  border:1px solid $border;
  border-radius:6px;
}
.one-wide, .two-wide, .finder-wide, .combo-wide {
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

.one-thin.header {
  height: 325px;
  .widget-content {
    max-height:325px - $header - $powered;
  }
}
.one-thin.no-header {
  height:275px;
  .widget-content {
    max-height:275px - $powered;
  }
}
.one-wide.header {
  height:250px;
  .widget-content {
    max-height:250px - $header - $powered;
  }
}
.one-wide.no-header {
  height:200px;
  .widget-content {
    max-height:200px - $powered;
  }
}
.two-thin.header {
  height:575px;
  .widget-content {
    max-height:575px - $header - $powered;
  }
}
.two-thin.no-heder {
  height:525px;
  .widget-content {
    max-height:525px - $powered;
  }
}
.two-wide.header {
  height:475px;
  .widget-content {
    max-height:475px - $header - $powered;
  }
}
.two-wide.no-header {
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
