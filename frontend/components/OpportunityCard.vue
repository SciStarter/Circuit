<template>
<article class="opportunity-card">
  <nuxt-link :to="'/' + opportunity.slug" class="primary">
    <img :src="image">
    <div>
      <h2>{{ subtitle }}</h2>
      <h1>{{ opportunity.title }}</h1>
      <small>bar{{ opportunity.short_desc }}</small>
      <aside>{{ notice_all }}</aside>
    </div>
  </nuxt-link>
  <div class="secondary">
    <div class="info location">
      <location-icon />
      <div>
        <opportunity-location :opportunity="opportunity" />
        <aside>{{ notice_place }}</aside>
      </div>
    </div>
    <div class="info time">
      <time-icon />
      <div>
        <opportunity-time :opportunity="opportunity" />
        <aside>{{ notice_time }}</aside>
      </div>
    </div>
    <div v-if="keywords.length" class="info keywords">
      <keywords-icon />
      <span v-for="kw in keywords" :key="kw">
        {{ kw }}
      </span>
    </div>
  </div>
</article>
</template>

<script>
import haversine from 'haversine'

import OpportunityLocation from "~/components/OpportunityLocation"
import OpportunityTime from "~/components/OpportunityTime"

import NoImage from "~/assets/img/no-image.svg?data"
import LocationIcon from '~/assets/img/search.svg?inline'
import TimeIcon from '~/assets/img/search.svg?inline'
import KeywordsIcon from '~/assets/img/search.svg?inline'

export default {
    components: {
        OpportunityLocation,
        OpportunityTime,

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

        notice_place() {
            let subject = this.$geolocation.coords;
            let object = this.opportunity.location_point;

            if(!(subject && subject.longitude && subject.latitude &&
                 object && object.geometry && object.geometry.longitude && object.geometry.latitude)
              ) {
                return "";
            }

            let distance = Math.floor(
                haversine(
                    {
                        longitude: subject.longitude,
                        latitude: subject.latitude,
                    },
                    {
                        longitude: object.geometry.coordinates[0],
                        latitude: object.geometry.coordinates[1],
                    },
                    {unit: 'mile'}
                )
            );

            if(distance < 1) {
                return "less than a mile!";
            }
            else if(distance <= 20) {
                return "" + distance + " miles away";
            }
            else {
                return "";
            }
        },

        notice_time() {
            const now = new Date();

            let future = this.opportunity.start_datetimes
                               .map(iso => new Date(iso))
                               .filter(dt => dt > now);

            if(future.length > 0) {
                future.sort();

                const until = (future[0] - now) / (60 * 60 * 1000);

                if(until > 24 && until < 168) {
                    const days = Math.floor(until / 24);
                    return "Happening in " + days + ((days > 1) ? " days" : " day");
                }
                else if(until > 1 && until < 24) {
                    const hours = Math.floor(until);
                    return "Happening in " + hours + ((hours > 1) ? " hours" : " hour");
                }
                else if(until < 1) {
                    return "Happening in a few minutes";
                }
            }

            return "";
        },

        notice_all() {
            return [this.notice_time, this.notice_place].filter(x => !!x).join(" and ");
        },

        subtitle() {
            return this.opportunity.organization_name || this.opportunity.partner_name;
        },

        keywords() {
            let ret = [];

            if(this.opportunity.opp_descriptor) {
                for(let desc of this.opportunity.opp_descriptor) {
                    if(desc) {
                        ret.push(desc);
                    }
                }
            }

            if(this.opportunity.opp_topics) {
                for(let topic of this.opportunity.opp_topics) {
                    if(topic) {
                        ret.push(topic);
                    }
                }
            }

            if(this.opportunity.tags) {
                for(let tag of this.opportunity.tags) {
                    if(tag) {
                        ret.push(tag);
                    }
                }
            }

            return ret;
        }
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
        border-radius: 20px;
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


        aside {
            display: block;
            font-family: $snm-font-content;
            font-size: 14px;
            color: $snm-color-info;
            letter-spacing: 0px;
            line-height: 16px;
        }
    }
}

.secondary {
    margin-top: 15px;

    .info {
        display: flex;
        align-items: flex-start;

        svg,img {
            width: 20px;
            height: auto;
            flex-grow: 0;
            flex-shrink: 0;
            margin-right: 5px;
        }

        aside {
            display: none;
            font-family: $snm-font-content;
            font-size: 12px;
            color: $snm-color-info;
            letter-spacing: 0px;
            line-height: 14px;
        }
    }

    .keywords {
        span:not(:first-of-type)::before {
            content: ", ";
        }
    }
}

@media (min-width: $fullsize-screen) {
    .primary {
        div {
            small {
                display: block;
            }

            aside {
                display: none;
            }
        }
    }

    .secondary {
        .info {
            aside {
                display: block;
            }
        }
    }
}
</style>
