<template>
<article class="opportunity">
  <img v-if="has_value(opportunity.image_url)" :src="opportunity.image_url" class="opportunity-image" :title="opportunity.image_credit">

  <div class="map" :class="{'open': show_map}">
    <a @click="show_map = false">&laquo; back</a>
    <div ref="map_display" />
  </div>

  <div class="opportunity-name">
    <strong>{{ subtitle }}</strong>
    <h1>{{ opportunity.title }}</h1>
  </div>

  <p class="elevator-pitch">
    {{ elevator_pitch }}
  </p>

  <div class="involvement">
    <div class="reviews-likes">
      <span v-if="reviews !== null">
        <stars v-model="reviews.average" />
        {{ reviews.reviews.length }} reviews
      </span>
      <span v-if="likes !== null">
        <like-icon />
        {{ likes }} likes
      </span>
    </div>
    <p>
      {{ saves }} People Interested
    </p>
    <p>
      {{ didit }} People Report Doing This Opportunity
    </p>
  </div>

  <div class="secondary">
    <div class="info location">
      <location-icon />
      <div>
        <opportunity-location :opportunity="opportunity" />
        <opportunity-notice :opportunity="opportunity" mode="place" />
      </div>
      <a v-if="has_value(location_geojson)" @click="show_map = true">see&nbsp;on&nbsp;map</a>
    </div>
    <div class="info time">
      <time-icon />
      <div>
        <opportunity-time :opportunity="opportunity" @upcoming="upcoming = $event" />
        <opportunity-notice :opportunity="opportunity" mode="time" />
        <b-tooltip v-if="has_value(opportunity.start_datetimes)" :triggers="['click']" :auto-close="['outside', 'escape']" type="is-light">
          <template #content>
            <div v-for="pair in upcoming" :key="pair[0].toISOString()" class="calendar-row">
              <label>
                {{ pair[0].toLocaleString() }}
              </label>
              <calendar-add calendar="google" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" />
              <calendar-add calendar="outlook" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" />
              <calendar-add calendar="365" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" />
              <calendar-add calendar="yahoo" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" />
            </div>
          </template>
          <action-button secondary>
            Add to calendar
          </action-button>
        </b-tooltip>
      </div>
    </div>
    <div class="info keywords">
      <keywords-icon />
      <opportunity-keywords :opportunity="opportunity" />
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

  <div class="description">
    <h2>About This Science Opportunity</h2>
    <vue-markdown :source="opportunity.description" class="content" :class="{'closed': description_closed}" />
    <a v-if="description_closed" @click="description_closed = false">read more</a>
  </div>

  <div v-if="has_value(opportunity.tags)" class="tags">
    <h2>Tags</h2>
    <nuxt-link v-for="tag in opportunity.tags" :key="tag" :to="'/find?text=' + encodeURIComponent(tag)">
      {{ tag }}
    </nuxt-link>
  </div>

  <div class="social">
    <h2>Social Media</h2>
    <p>
      <strong>Hashtags:</strong>
      {{ opportunity.opp_hashtags.join(', ') || '#science' }}
    </p>
    <p v-for="(value, key) in opportunity.opp_social_handles" :key="key">
      <strong>{{ title_case(key) }}:</strong>
      {{ value }}
    </p>
  </div>

  <div class="reviews">
    <h2>Reviews</h2>
    <template v-if="!loading_reviews">
      <div v-for="review in reviews.reviews" :key="review.id" class="review">
        <stars v-model="review.rating" />
        <a class="report" @click="report_review(review.id)">Report</a>
        {{ review.username }} &bull; {{ (new Date(review.when)).toLocaleString() }}
        <vue-markdown :source="review.comment" />
      </div>
    </template>
    <b-loading v-model="loading_reviews" :is-full-page="false" />
  </div>

  <div class="related">
    <h2>Nearby &amp; Similar Opportunities</h2>
    <template v-if="!loading_recommended">
      <nuxt-link v-for="rec in recommended" :key="rec.uid" :to="'/' + rec.slug">
        <h3>{{ rec.title }}</h3>
        <div>
          <location-icon />
          <opportunity-location :opportunity="rec" short />
        </div>
        <div>
          <time-icon />
          <opportunity-time :opportunity="rec" />
        </div>
      </nuxt-link>
    </template>
    <b-loading v-model="loading_recommended" :is-full-page="false" />
  </div>
</article>
</template>

<script>
import 'mapbox-gl/dist/mapbox-gl.css'
import mapboxgl from 'mapbox-gl'
import extent from 'geojson-extent'
import startCase from 'lodash/startCase'
import VueMarkdown from "vue-markdown"

import OpportunityLocation from "~/components/OpportunityLocation"
import OpportunityTime from "~/components/OpportunityTime"
import OpportunityKeywords from "~/components/OpportunityKeywords"
import OpportunityNotice from "~/components/OpportunityNotice"
import External from "~/components/External"
import Stars from "~/components/Stars"
import CalendarAdd from "~/components/CalendarAdd"

import LocationIcon from '~/assets/img/location-marker.svg?inline'
import TimeIcon from '~/assets/img/calendar.svg?inline'
import KeywordsIcon from '~/assets/img/speech-bubble.svg?inline'
import LikeIcon from '~/assets/img/like.svg?inline'
import MapMarker from '~/assets/img/marker.png'

export default {
    components: {
        VueMarkdown,

        OpportunityLocation,
        OpportunityTime,
        OpportunityKeywords,
        OpportunityNotice,
        External,
        Stars,
        CalendarAdd,

        LocationIcon,
        TimeIcon,
        KeywordsIcon,
        LikeIcon,
    },

    props: {
        entity: {
            type: Object,
            required: true
        }
    },

    data() {
        return {
            upcoming: [],
            map_widget: null,
            reviews: null,
            likes: null,
            recommended: null,
            saves: null,
            didit: null,
            show_map: false,
            description_closed: true,
        }
    },

    async fetch() {
        const fetch_likes = async () => {
            this.likes = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/likes');
        };

        const fetch_reviews = async () => {
            this.reviews = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/reviews');
        };

        const fetch_recommended = async () => {
            this.recommended = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/recommended');
        };

        const fetch_saves = async () => {
            this.saves = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/saves');
        };

        const fetch_didit = async () => {
            this.didit = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/didit');
        };

        await Promise.all([
            fetch_reviews(),
            fetch_recommended(),
            fetch_likes(),
            fetch_saves(),
            fetch_didit(),
        ]);
    },

    computed: {
        loading_reviews() {
            return this.reviews === null;
        },

        loading_recommended() {
            return this.recommended === null;
        },

        opportunity() {
            return this.entity;
        },

        subtitle() {
            return this.opportunity.organization_name || this.opportunity.partner_name;
        },

        elevator_pitch() {
            if(this.opportunity.short_desc) {
                return this.opportunity.short_desc;
            }

            const first_sentence = this.opportunity.description.split('. ')[0];

            if(first_sentence.length < 120) {
                return first_sentence;
            }

            return first_sentence.slice(0, 117) + '…';
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
        },

        location_geojson() {
            let geom;
            let props = {};

            if(this.has_value(this.opportunity.location_polygon)) {
                geom = this.opportunity.location_polygon;
                console.log('X', geom);
                props.mode = 'polys';
            }
            else if(this.has_value(this.opportunity.location_point)) {
                geom = this.opportunity.location_point;
                props.mode = 'points';
            }
            else {
                return null;
            }

            return {
                'type': 'Feature',
                'geometry': geom,
                'properties': props,
            };
        },
    },

    mounted() {
        if(this.location_geojson) {
            this.map_widget = new mapboxgl.Map({
                accessToken: process.env.MAPBOX_TOKEN,
                container: this.$refs.map_display,
                style: 'mapbox://styles/mapbox/streets-v11',
                center: [-98, 39],
                zoom: 2
            });

            this.map_widget.on('load', () => {
                this.map_widget.loadImage(MapMarker, (error, image) => {
                    if(error) {
                        throw error;
                    }

                    this.map_widget.addImage("snm-marker", image);

                    this.map_widget.addSource('opportunity', {
                        'type': 'geojson',
                        'data': this.location_geojson
                    });

                    if(this.location_geojson.properties.mode === 'points') {
                        // https://docs.mapbox.com/mapbox-gl-js/example/geojson-markers/
                        this.map_widget.addLayer({
                            'id': 'opportunity',
                            'type': 'symbol',
                            'source': 'opportunity',
                            'layout': {
                                'icon-image': 'snm-marker',
                                'icon-allow-overlap': true,
                            },
                        });
                    }
                    else if(this.location_geojson.properties.mode === 'polys') {
                        // https://docs.mapbox.com/mapbox-gl-js/example/geojson-polygon/
                        this.map_widget.addLayer({
                            'id': 'opportunity',
                            'type': 'fill',
                            'source': 'opportunity',
                            'layout': {},
                            'paint': {
                                'fill-color': '#ffbf40',
                                'fill-opacity': 0.5,
                            },
                        });
                    }
                    else {
                        console.warn("Unrecognized map mode: ", this.location_geojson.properties.mode);
                        return;
                    }

                    let bounds = extent(this.location_geojson);
                    this.map_widget.fitBounds([[bounds[0]-0.01, bounds[1]-0.01], [bounds[2]+0.01, bounds[3]+0.01]]);
                });
            });
        }
    },

    methods: {
        report_review(id) {
            this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/report-review', {id: id}).then(() => {
                this.$buefy.toast.open({
                    message: 'Reported review',
                    type: 'is-success'
                });
            });
        },

        title_case(s) {
            return startCase(s);
        },

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
                else if(item.length == 1 && item[0] === '') {
                    return false;
                }
                else {
                    return true;
                }
            }

            if(typeof(item) === 'string') {
                if(item === '') {
                    return false;
                }
                else {
                    return true;
                }
            }

            if(!Object.keys(item).some(x => x[0] != '_')) {
                return false;
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

.map {
    position: fixed;
    top: 10vh;
    right: 0vw;
    width: 0vw;
    opacity: 0;
    background-color: $snm-color-background;
    overflow: hidden;
    transition: width 0.5s, opacity 0.5s, right 0.5s;
    box-sizing: border-box;
    border: 2px solid $snm-color-border;
    padding: 5px 1rem 1rem 1rem;

    &.open {
        right: 1vw;
        width: 98vw;
        opacity: 1;
    }

    div {
        display: block;
        width: calc(98vw - 2rem);
        height: calc(98vw - 2rem);
    }
}

.opportunity-name {
    margin: 17px;

    strong {
        font-family: $snm-font-content;
        font-weight: bold;
        font-size: 14px;
        color: $snm-color-element-dark;
        line-height: 16px;
        letter-spacing: 0px;
    }

    h1 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 24px;
        color: $snm-color-info;
        line-height: 28px;
        letter-spacing: 0px
    }
}

.elevator-pitch {
    margin: 17px;
    font-family: $snm-font-content;
    font-weight: normal;
    font-size: 16px;
    line-height: 22px;
    letter-spacing: 0.16px;
    color: $snm-color-glance;
}

.involvement {
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;
    padding: 17px;

    .reviews-likes {
        span {
            font-family: $snm-font-content;
            font-size: 16px;
            line-height: 19px;
            letter-spacing: 0px;
            color: $snm-color-element-dark;

            > :first-child {
                margin-right: 0.75rem;
            }
        }

        span:not(:first-of-type) {
            margin-left: 3rem;
        }
    }

    > p {
        margin-top: 10px;
    }
}

.secondary {
    padding: 17px;
}

.info {
    margin-top: 5px;
    display: flex;
    align-items: top;

    > svg {
        height: 1rem;
        width: 16px;;
        margin: 0.25rem 20px 0px 0px;
        flex-grow: 0;
        flex-shrink: 0;
    }

    > a {
        display: block;
        margin-left: 2rem;
        flex-grow: 0;
    }
}

.calendar-row {
    display: flex;
    flex-wrap: wrap;
    width: 200px;

    label {
        display: block;
        font-family: $snm-font-content;
        font-weight: bold;
        color: $snm-color-element-dark;
        width: 100%;
    }

    .calendar-add {
        margin-right: 8px;
    }
}

.calendar-row:not(:last-child) {
    margin-bottom: 1rem;
}

.find-out-more {
    display: block;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    background-color: $snm-color-element-med;
    box-shadow: 0px 1px 7px $snm-color-shadow;
    color: $snm-color-element-ondark;
    padding: 17px;

    strong {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 21px;
        line-height: 26px;
        letter-spacing: 0px;
        display: block;
    }

    span {
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: 14px;
        line-height: 16px;
        letter-spacing: 0px;
    }
}

.find-out-more:hover {
    color: $snm-color-element-ondark;
}

.partner-and-org {
    display: flex;

    figure {
        margin: 17px;
        text-align: center;

        figcaption {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: 14px;
            line-height: 17px;
            letter-spacing: 0px;
            color: $snm-color-caption;
            margin-bottom: 1rem;
        }

        img {
            object-fit: contain;
            object-position: center center;
        }
    }
}

.more-info {
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;
    padding: 17px 0px 17px 17px;

    > *:not(:first-child) {
        border-top: 1px solid $snm-color-border;
        margin-top: 17px;
        padding-top: 17px;
    }

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 16px;
        line-height: 22px;
        letter-spacing: 0px;
        color: $snm-color-background-dark;
    }
}

.description {
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;
    padding: 17px;

    > h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 16px;
        line-height: 19px;
        letter-spacing: 0px;
        color: $snm-color-background-dark;
        margin-bottom: 17px;
    }

    > div {
        position: relative;
        overflow: hidden;

        &.closed {
            max-height: 8rem;

            &:after {
                content  : "";
                position : absolute;
                z-index  : 1;
                bottom   : 0;
                left     : 0;
                pointer-events   : none;
                background-image : linear-gradient(to bottom, change-color($snm-color-background, $alpha: 0), $snm-color-background 90%);
                width    : 100%;
                height   : 2rem;
            }
        }
    }
}

.tags {
    display: flex;
    flex-wrap: wrap;
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;
    padding: 17px;

    h2 {
        width: 100%;
        margin-bottom: 1rem;
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 16px;
        line-height: 22px;
        letter-spacing: 0px;
        color: $snm-color-background-dark;
    }

    a {
        display: block;
        padding: 8px;
        border: 1px solid $snm-color-element-med;
        border-radius: 10px;
        margin: 8px;
    }
}

.social {
    border-top: 1px solid $snm-color-border;
    padding: 17px;
    font-family: $snm-font-content;
    font-weight: normal;
    font-size: 16px;
    line-height: 19px;
    letter-spacing: 0px;

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 16px;
        line-height: 22px;
        letter-spacing: 0px;
        color: $snm-color-background-dark;

    }

    p {
        margin: 11px 0px;

        strong {
            font-weight: bold;
        }
    }
}

.reviews {
    position: relative;
    min-height: 3rem;

    h2 {
        color: $snm-color-element-light;
        background-color: $snm-color-element-med;
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 16px;
        line-height: 19px;
        padding: 17px;
    }
}

.related {
    position: relative;
    min-height: 3rem;

    h2 {
        color: $snm-color-element-light;
        background-color: $snm-color-info;
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 16px;
        line-height: 19px;
        padding: 17px;
    }

    > a {
        display: block;
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: 14px;
        line-height: 16px;
        color: $snm-color-element-dark;
        margin: 17px;

        h3 {
            font-family: $snm-font-heading;
            font-size: 21px;
            font-weight: bold;
            line-height: 26px;
            color: $snm-color-element-med;
            text-decoration: underline;
        }

        div {
            display: flex;
            margin: 3px 0px;

            svg {
                margin-right: 0.75rem;
                align-self: center;
            }
        }
    }
}

@media (min-width: $fullsize-screen) {

}
</style>
