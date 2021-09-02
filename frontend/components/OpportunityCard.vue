<template>
<article class="opportunity-card">
  <nuxt-link :to="'/' + opportunity.slug" class="primary">
    <img :src="image">
    <div>
      <h2>{{ subtitle }}</h2>
      <h1>{{ opportunity.title }}</h1>
      <small>bar{{ opportunity.short_desc }}</small>
      <opportunity-notice :opportunity="opportunity" mode="all" />
    </div>
  </nuxt-link>
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
      <opportunity-keywords :opportunity="opportunity" />
    </div>
  </div>
</article>
</template>

<script>
import OpportunityLocation from "~/components/OpportunityLocation"
import OpportunityTime from "~/components/OpportunityTime"
import OpportunityKeywords from "~/components/OpportunityKeywords"
import OpportunityNotice from "~/components/OpportunityNotice"

import NoImage from "~/assets/img/no-image-thumb.jpg"
import LocationIcon from '~/assets/img/location-marker.svg?inline'
import TimeIcon from '~/assets/img/calendar.svg?inline'
import KeywordsIcon from '~/assets/img/speech-bubble.svg?inline'

export default {
    components: {
        OpportunityLocation,
        OpportunityTime,
        OpportunityKeywords,
        OpportunityNotice,

        LocationIcon,
        TimeIcon,
        KeywordsIcon
    },

    props: {
        opportunity: {
            type: Object,
            required: true
        }
    },

    computed: {
        image() {
            return this.opportunity.image_url || NoImage;
        },

        subtitle() {
            return this.opportunity.organization_name || this.opportunity.partner_name;
        },
    }
}
</script>

<style lang="scss" scoped>
.opportunity-card {
    padding: 16px;
    border-bottom: 2px solid $snm-color-border;
}

.primary {
    display: flex;

    img {
        width: 80px;
        height: 80px;
        border: none;
        border-radius: 10px;
        overflow: hidden;
        flex-shrink: 0;
        flex-grow: 0;
        object-fit: cover;
        object-position: center center;
        margin: 0px 16px 0px 0px;
    }

    div {
        h2 {
            font-size: 14px;
            font-weight: bold;
            font-family: $snm-font-content;
            letter-spacing: 0px;
            color: $snm-color-element-dark;
            line-height: 16px;
        }

        h1 {
            text-decoration: underline;
            color: $snm-color-element-med;
            font-family: $snm-font-heading;
            letter-spacing: 0px;
            font-size: 16px;
            font-weight: bold;
            line-height: 22px;
            margin-bottom: 8px;
        }

        small {
            display: none;
        }
    }
}

.secondary {
    margin-top: 15px;

    .info {
        display: flex;
        align-items: flex-start;

        svg,img {
            display: block;
            width: 10px;
            height: 12px;
            flex-grow: 0;
            flex-shrink: 0;
            margin-right: 5px;
            margin-top: 0.4em;
        }

        .opportunity-notice {
            display: none;
            font-size: 12px;
            line-height: 14px;
        }
    }
}

@media (min-width: $fullsize-screen) {
    .primary {
        div {
            small {
                display: block;
            }

            .opportunity-notice {
                display: none;
            }
        }
    }

    .secondary {
        .info {
            .opportunity-notice {
                display: block;
            }
        }
    }
}
</style>
