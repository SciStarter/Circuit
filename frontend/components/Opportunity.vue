<template>
<article class="opportunity">
  <img v-if="has_value(opportunity.image_url)" :src="opportunity.image_url" class="opportunity-image" :title="opportunity.image_credit">

  <div class="map">
    map
  </div>

  <strong>{{ subtitle }}</strong>
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
    class="find-out-more"
    >
    <strong>Find out more</strong>
    <span>{{ opportunity.partner_opp_url }}</span>
  </external>

  <div class="partner-and-org">
    <figure v-if="opportunity.partner_logo_url || opportunity.partner_name">
      <figcaption>As Featured On</figcaption>
      <component :is="opportunity.partner_website ? 'external' : 'span'" :href="opportunity.partner_website" campaign="opp-page" content="featured-on" new-tab>
        <img v-if="opportunity.partner_logo_url" :src="opportunity.partner_logo_url" :alt="opportunity.partner_name + ' logo'">
        <span v-else>{{ opportunity.partner_name }}</span>
      </component>
    </figure>
    <figure v-if="opportunity.organization_logo_url || opportunity.organization_name">
      <figcaption>Hosted By</figcaption>
      <component :is="opportunity.organization_website ? 'external' : 'span'" :href="opportunity.organization_website" campaign="opp-page" content="hosted-by" new-tab>
        <img v-if="opportunity.organization_logo_url" :src="opportunity.organization_logo_url" :alt="opportunity.organization_name + ' logo'">
        <span v-else>{{ opportunity.organization_name }}</span>
      </component>
    </figure>
  </div>

  <div class="more-info">
    <h2>More Information</h2>
    <p v-if="has_value(opportunity.cost)" class="item">
      Cost: {{ opportunity.cost !== 'free' ? 'Yes' : 'No' }}
    </p>
    <p v-if="has_value(opportunity.ticket_required)" class="item">
      Ticket Required: {{ opportunity.ticket_required ? 'Yes' : 'No' }}
    </p>
    <p v-if="has_value(opportunity.opp_venue)" class="item">
      Venue Type: {{ venue_type }}
    </p>
    <p v-if="has_value(opportunity.min_age) && opportunity.min_age > 0" class="item">
      Minimum Age: {{ opportunity.min_age }}
    </p>
    <p v-if="has_value(opportunity.max_age) && opportunity.max_age < 999" class="item">
      Maximum Age: {{ opportunity.max_age }}
    </p>
    <p v-if="has_value(opportunity.languages)" class="item">
      Languages: {{ opportunity.languages.join(', ') }}
    </p>
  </div>

  <vue-markdown :source="opportunity.description" class="description" />

  <div v-if="has_value(opportunity.tags)" class="tags">
    <span v-for="tag in opportunity.tags" :key="tag">{{ tag }}</span>
  </div>

  <div class="social">
    {{ opportunity.opp_hashtags.join(', ') }}
    <pre>
      {{ JSON.stringify(opportunity.opp_social_handles, null, 2) }}
    </pre>
  </div>

  <div v-if="has_value(reviews)" class="reviews">
    <h2>Reviews</h2>
    {{ reviews }}
  </div>

  <div v-if="has_value(recommended)" class="related">
    <h2>Nearby &amp; Similar Opportunities</h2>
    {{ recommended }}
  </div>
</article>
</template>

<script>
import VueMarkdown from "vue-markdown"

import OpportunityLocation from "~/components/OpportunityLocation"
import OpportunityTime from "~/components/OpportunityTime"
import OpportunityKeywords from "~/components/OpportunityKeywords"
import OpportunityNotice from "~/components/OpportunityNotice"
import External from "~/components/External"

import LocationIcon from '~/assets/img/location-marker.svg?inline'
import TimeIcon from '~/assets/img/calendar.svg?inline'
import KeywordsIcon from '~/assets/img/speech-bubble.svg?inline'

export default {
    components: {
        VueMarkdown,

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

    data() {
        return {
            reviews: null,
            recommended: null
        }
    },

    async fetch() {
        const fetch_reviews = async () => {
            let resp = this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/reviews');
            this.reviews = resp.payload;
        }

        const fetch_recommended = async () => {
            let resp = this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/recommended');
            this.recommended = resp.payload;
        }

        await Promise.all([
            fetch_reviews(),
            fetch_recommended()
        ]);
    },

    computed: {
        opportunity() {
            return this.entity;
        },

        subtitle() {
            return this.opportunity.organization_name || this.opportunity.partner_name;
        },

        elevator_pitch() {
            return this.opportunity.short_desc || (this.opportunity.description.split('. ')[0].slice(0, 117) + '...');
        },

        venue_type() {
            const indoors = this.opportunity.opp_venue.indexOf('indoors') >= 0;
            const outdoors = this.opportunity.opp_venue.indexOf('outdoors') >= 0;

            if(indoors && outdoors) {
                return "Indoors and outdoors";
            }
            else if(indoors) {
                return "Indoors";
            }
            else if(outdoors) {
                return "Outdoors";
            }
            else {
                return "We don't know";
            }
        }
    },

    methods: {
        has_value(item, test_result) {
            if(test_result !== undefined) {
                return test_result;
            }

            if(item === undefined || item === null) {
                return false;
            }

            if(Array.isArray(item)) {
                if(item.length == 0) {
                    return false;
                }
                if(item.length == 1 && item[0] === '') {
                    return false;
                }
            }

            if(typeof(item) === 'string') {
                if(item === '') {
                    return false;
                }
            }

            return true;
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
