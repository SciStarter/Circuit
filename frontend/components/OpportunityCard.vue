<template>
<article v-if="!hidden" class="opportunity-card" :class="{'rule': rule}">
  <button v-if="trash" class="trash" @click="$emit('trash', opportunity)">
    <trash-icon />
  </button>
  <nuxt-link :to="'/' + opportunity.slug" class="primary">
    <img :src="image">
    <div>
      <h2>{{ subtitle }}</h2>
      <h1>{{ opportunity.title }}</h1>
      <small>{{ opportunity.short_desc }}</small>
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
import TrashIcon from '~/assets/img/trash.svg?inline'

export default {
    components: {
        OpportunityLocation,
        OpportunityTime,
        OpportunityKeywords,
        OpportunityNotice,

        LocationIcon,
        TimeIcon,
        KeywordsIcon,
        TrashIcon,
    },

    props: {
        opportunity: {
            type: Object,
            required: true
        },

        trash: {
            type: Boolean,
            required: false,
            default: false,
        },

        rule: {
            type: Boolean,
            required: false,
            default: true,
        },

        hidden: {
            type: Boolean,
            required: false,
            default: false,
        },
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

    &.rule {
        border-bottom: 2px solid $snm-color-border;
    }
}

.trash {
    float: right;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    color: $snm-color-element-med;
    border: 1px solid $snm-color-element-med;
    border-radius: 10px;
    box-shadow: 0px 3px 6px $snm-color-shadow;
    cursor: pointer;

    svg * {
        fill: currentColor;
    }
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
            font-size: $snm-font-smaller;
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
            font-size: $snm-font-small;
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
            font-size: $snm-font-smallest;
            line-height: 14px;
        }
    }
}

@media (min-width: $fullsize-screen) {
    .opportunity-card {
        width: 20vw;
        border: 1px solid $snm-color-border;
        border-radius: 6px;
        margin: 0.5rem;
    }

    .primary {
        div {
            small {
                display: block;
                font-family: $snm-font-content;
                font-size: $snm-font-smaller;
                color: $snm-color-element-dark;
                letter-spacing: 0.14px;
            }

            .opportunity-notice {
                display: none;
            }
        }
    }

    .secondary {
        display: flex;
        flex-wrap: wrap;

        .info {
            width: 20vw;
            min-width: 13rem;

            .opportunity-notice {
                display: block;
            }
        }

        > :last-child {
            width: 100%;
        }
    }
}
</style>
