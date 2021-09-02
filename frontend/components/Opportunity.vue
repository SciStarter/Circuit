<template>
<article class="opportunity">
  <img :src="image" class="opportunity-image">

  <div class="map">
    map
  </div>

  <h2>{{ subtitle }}</h2>
  <h1>{{ opportunity.title }}</h1>

  <p>
    {{ elevator_pitch }}
  </p>

  <div class="involvement">
    <reviews />
    <likes />
    <interested />
    <participated />
  </div>

  <div class="secondary">
    <div class="info location">
      <location-icon />
      <div>
        <opportunity-location :opportunity="opportunity" />
        <opportunity-notice :opportunity="opportunity" mode="place" />
      </div>
    </div>
    <div class="info time">
      <time-icon />
      <div>
        <opportunity-time :opportunity="opportunity" />
        <opportunity-notice :opportunity="opportunity" mode="time" />
      </div>
    </div>
    <div class="info keywords">
      <keywords-icon />
    </div>
  </div>

  <external
    :href="opportunity.partner_opp_url"
    title="Find out more"
    campaign="opp-page"
    content="find-out-more"
    class="find-out-more">
    <strong>Find out more</strong>
    <span>{{ opportunity.partner_opp_url }}</span>
  </external>

  <div class="partner-and-org">
    <figure v-if="opportunity.partner_logo_url || opportunity.partner_name">
      <figcaption>As Featured On</figcaption>
      <img v-if="opportunity.partner_logo_url" :src="opportunity.partner_logo_url" :alt="opportunity.partner_name + ' logo'">
      <span v-else>{{ opportunity.partner_name }}</span>
    </figure>
    <figure v-if="opportunity.organization_logo_url || opportunity.organization_name">
      <figcaption>Hosted By</figcaption>
      <img v-if="opportunity.organization_logo_url" :src="opportunity.organization_logo_url" :alt="opportunity.organization_name + ' logo'">
      <span v-else>{{ opportunity.organization_name }}</span>
    </figure>
  </div>

  <div class="more-info">
    
  </div>

  <pre>
    {{ JSON.stringify(opportunity, null, 2) }}
    </pre>
  </article>
</template>

<script>
import OpportunityLocation from "~/components/OpportunityLocation"
import OpportunityTime from "~/components/OpportunityTime"
import OpportunityKeywords from "~/components/OpportunityKeywords"
import OpportunityNotice from "~/components/OpportunityNotice"
import External from "~/components/External"

import NoImage from "~/assets/img/no-image.svg?data"
import LocationIcon from '~/assets/img/location-marker.svg?inline'
import TimeIcon from '~/assets/img/calendar.svg?inline'
import KeywordsIcon from '~/assets/img/speech-bubble.svg?inline'

export default {
    components: {
        OpportunityLocation,
        OpportunityTime,
        OpportunityKeywords,
        OpportunityNotice,
        External,

        LocationIcon,
        TimeIcon,
        KeywordsIcon
    },

    props: {
        entity: {
            type: Object,
            required: true
        }
    },

    computed: {
        opportunity() {
            return this.entity;
        },

        image() {
            return this.opportunity.image_url || NoImage;
        },

        subtitle() {
            return this.opportunity.organization_name || this.opportunity.partner_name;
        },

        elevator_pitch() {
            return this.opportunity.short_desc || (this.opportunity.description.split('. ')[0].slice(0, 117) + '...');
        }
    }
}
</script>

<style lang="scss" scoped>
img.opportunity-image {
    width: 100vw;
    height: 158px;
    object-fit: contain;
    object-position: center center;
    overflow: hidden;
}

.find-out-more {
    display: block;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;

    strong {
        display: block;
    }
}

@media (min-width: $fullsize-screen) {

}
</style>
