<template>
<article class="opportunity">
  <div class="mobile-menu" :class="{'closed': !mobile_menu_open}" data-context="mobile-menu">
    <external-link
      :href="opportunity.partner_opp_url"
      title="Find out more"
      campaign="opp-page"
      content="find-out-more"
      new-tab
      >
      <link-icon /> Find Out More
    </external-link>
    <a @click="do_save">
      <saved-icon /> Save for Later
    </a>
    <a @click="do_like">
      <like-icon /> Like
    </a>
    <a @click="show_calendar_add = true">
      <time-icon /> Add to Calendar
    </a>
    <a @click="show_review_add = true">
      <star-icon /> Add Review
    </a>
    <div>
      <social-button mode="facebook" :opportunity="opportunity" />
      <social-button mode="twitter" :opportunity="opportunity" />
      <social-button mode="linkedin" :opportunity="opportunity" />
    </div>
  </div>

  <button class="mobile-menu-toggle mobile-only" title="Toggle menu" :aria-pressed="String(mobile_menu_open)" @click="mobile_menu_open = !mobile_menu_open">
    <span v-if="mobile_menu_open" title="close mobile menu">&times;</span>
    <img v-else src="~assets/img/hamburger.svg?data" title="open mobile menu">
  </button>

  <div class="snm-container">

    <div class="opportunity-left">

      <img v-if="has_value(opportunity.image_url)" :src="opportunity.image_url" class="opportunity-image" :title="opportunity.image_credit">


      <div class="opportunity-section">
        <div class="opportunity-name">
          <strong>{{ subtitle }}</strong>
          <h1>{{ opportunity.title }}</h1>
        </div>

        <div class="elevator-pitch">
          <vue-markdown :source="elevator_pitch" class="content" />
        </div>
      </div>

      <div class="involvement opportunity-section">
        <div class="reviews-likes">
          <span v-if="reviews !== null">
            <stars v-model="reviews.average" />
            {{ reviews.reviews.length }} reviews
          </span>
          <span v-if="likes !== null">
            <like-icon :class="{'liked': did.like}" />
            {{ likes }} likes
          </span>
        </div>
        <div class="numbers">
          <p>
            {{ saves }} People Interested
          </p>
          <p>
            {{ didit }} People Report Doing This Opportunity
          </p>
        </div>
        <b-modal
          v-model="show_bookmark_add"
          has-modal-card
          trap-focus
          :destroy-on-hide="false"
          aria-role="dialog"
          aria-label="Add a Review"
          aria-modal
          >
          <div class="card">
            <h2>You must be signed in to save an opportunity</h2>
            <action-button primary @click="(show_bookmark_add = false) || $emit('login')">
              Sign In
            </action-button>
            <h2>Don't have an account?</h2>
            <action-button secondary @click="(show_bookmark_add = false) || $emit('signup')">
              Create an Account
            </action-button>
          </div>
        </b-modal>
        <action-button class="no-mobile" principal :disabled="did.save" @click="do_save">
          <saved-icon /> <span v-if="did.save">Saved</span><span v-else>Save for Later</span>
        </action-button>
        <action-button :disabled="did.like" class="no-mobile" secondary @click="do_like">
          <like-icon /> <span v-if="did.like">You liked this</span><span v-else>Like</span>
        </action-button>
        <social-button mode="facebook" :opportunity="opportunity" class="no-mobile" />
        <social-button mode="twitter" :opportunity="opportunity" class="no-mobile" />
        <social-button mode="linkedin" :opportunity="opportunity" class="no-mobile" />
      </div>

      <div class="secondary opportunity-section">
        <div class="info location">
          <location-icon />
          <div>
            <opportunity-location :opportunity="opportunity" is-opportunity />
            <opportunity-notice :opportunity="opportunity" mode="place" />
          </div>
          <a v-if="(opportunity.location_type == 'at' || opportunity.location_type == 'near') && has_value(location_geojson)" @click="show_map = true">see&nbsp;on&nbsp;map</a>
        </div>
        <div class="info time">
          <time-icon />
          <div>
            <opportunity-time :opportunity="opportunity" @upcoming="upcoming = $event" />
            <opportunity-notice :opportunity="opportunity" mode="time" />
            <b-modal
              v-model="show_calendar_add"
              has-modal-card
              trap-focus
              :destroy-on-hide="false"
              aria-role="dialog"
              aria-label="Add to calendar"
              aria-modal
              >
              <div class="card">
                <div v-for="pair in upcoming" :key="pair[0].toISOString()" class="calendar-row">
                  <label>
                    {{ pair[0].toLocaleString() }}
                  </label>
                  <calendar-add calendar="google" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" @before="register_interest" />
                  <calendar-add calendar="outlook" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" @before="register_interest" />
                  <calendar-add calendar="365" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" @before="register_interest" />
                  <calendar-add calendar="yahoo" :title="opportunity.title" :location="opportunity.location_name" :begin="pair[0]" :end="pair[1]" :description="opportunity.partner_opp_url" @before="register_interest" />
                </div>
              </div>
            </b-modal>
            <action-button v-if="has_value(opportunity.start_datetimes)" secondary class="no-mobile" @click="show_calendar_add = true">
              Add to calendar
            </action-button>
          </div>
        </div>
        <div class="info keywords">
          <keywords-icon />
          <opportunity-keywords :opportunity="opportunity" />
        </div>
      </div>

      <external-link
        :href="opportunity.partner_opp_url"
        title="Find out more"
        campaign="opp-page"
        content="find-out-more"
        class="find-out-more"
        @before="register_interest"
        new-tab
        >
        <strong>Find out more</strong>
        <span>{{ opportunity.partner_opp_url }}</span>
      </external-link>

      <div class="partner-and-org opportunity-section">
        <figure v-if="opportunity.partner_logo_url || opportunity.partner_name">
          <figcaption>As Featured On</figcaption>
          <component :is="opportunity.partner_website ? 'external-link' : 'span'" :href="opportunity.partner_website" campaign="opp-page" content="featured-on" new-tab>
            <img v-if="opportunity.partner_logo_url" :src="opportunity.partner_logo_url" :alt="opportunity.partner_name + ' logo'">
            <span v-else>{{ opportunity.partner_name }}</span>
          </component>
        </figure>
        <figure v-if="opportunity.organization_logo_url || opportunity.organization_name">
          <figcaption>Hosted By</figcaption>
          <component :is="opportunity.organization_website ? 'external-link' : 'span'" :href="opportunity.organization_website" campaign="opp-page" content="hosted-by" new-tab>
            <img v-if="opportunity.organization_logo_url" :src="opportunity.organization_logo_url" :alt="opportunity.organization_name + ' logo'">
            <span v-else>{{ opportunity.organization_name }}</span>
          </component>
        </figure>
      </div>

      <div class="more-info opportunity-section">
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

      <div class="description opportunity-section">
        <h2>About This Science Opportunity</h2>
        <read-more v-model="description_open">
          <vue-markdown :source="opportunity.description" class="content" />
        </read-more>
      </div>

      <div v-if="has_value(opportunity.tags)" class="tags opportunity-section">
        <h2>Tags</h2>
        <nuxt-link v-for="tag in opportunity.tags" :key="tag" :to="'/find?text=' + encodeURIComponent(tag)">
          {{ tag }}
        </nuxt-link>
      </div>

      <div class="social opportunity-section">
        <h2>Social Media</h2>
        <p>
          <strong>Hashtags:</strong>
          {{ opportunity.opp_hashtags.join(', ') || '#science' }}
        </p>
        <p v-for="(value, key) in opportunity.opp_social_handles" :key="key">
          <strong>{{ title_case(key) }}:</strong>
          <a :href="value">{{ value }}</a>
        </p>
      </div>

      <div class="reviews">

        <b-modal
          v-model="show_review_add"
          has-modal-card
          trap-focus
          :destroy-on-hide="false"
          aria-role="dialog"
          aria-label="Add a Review"
          aria-modal
          >
          <div class="card">
            <div v-if="user.authenticated">
              <h2>Rate &amp; Review</h2>
              <div>
                <stars v-model="new_review.rating" editable />
              </div>
              <b-input v-model="new_review.comment" type="textarea" />
              <div class="buttons">
                <action-button tertiary @click="show_review_add = false">
                  Cancel
                </action-button>
                <action-button primary @click="do_review">
                  Submit
                </action-button>
              </div>
            </div>
            <div v-else>
              <h2>You must be signed in to add a review</h2>
              <action-button primary @click="(show_review_add = false) || $emit('login')">
                Sign In
              </action-button>
              <h2>Don't have an account?</h2>
              <action-button tertiary @click="(show_review_add = false) || $emit('signup')">
                Create an Account
              </action-button>
            </div>
          </div>
        </b-modal>
        <div class="reviews-header">
          <h2>Reviews</h2>
          <action-button class="no-mobile" secondary @click="show_review_add = true">
            <star-icon /> Add Review
          </action-button>
        </div>
        <template v-if="!loading_reviews">
          <div v-for="review in reviews.reviews" :key="review.id" class="review">
            <div>
              <stars v-model="review.rating" />
              <a class="report" @click="report_review(review.id)"><flag-icon /> Report</a>
            </div>
            <div>
              {{ review.username }} &bull; {{ (new Date(review.when)).toLocaleString() }}
            </div>
            <read-more v-model="review.expanded">
              <vue-markdown :source="review.comment" />
            </read-more>
          </div>
        </template>
        <b-loading v-model="loading_reviews" :is-full-page="false" />
      </div>
    </div>
    <div class="opportunity-right">

      <div class="ididthis no-mobile">
        <h2>
          <atom-icon /> <span v-if="did.didit">Thanks for letting us know!</span><span v-else>Help Scientists!</span>
        </h2>
        <p v-if="did.didit">
          You're helping scientists study public engagement in science!
          You can now find this logged in your
          <nuxt-link to="/my/science">
            My Science
          </nuxt-link>
          activity list.
        </p>
        <p v-else>
          You can help scientists studying public participation in science
          by logging your participation in this science opportunity.
        </p>
        <b-modal
          v-model="show_didit_logged_out"
          has-modal-card
          trap-focus
          :destroy-on-hide="false"
          aria-role="dialog"
          aria-label="Add a Review"
          aria-modal
          >
          <div class="card">
            <h2>Thanks for letting us know! But&hellip; <span class="close" @click="show_didit_logged_out = false">&times;</span></h2>
            <p>
              We love hearing about people engaged in science, and to
              better support these opportunities we could use some
              additional information about you!
            </p>
            <p>
              For now we'll save that you've done this activity in your
              browser's storage, but making an account will make sure you
              don't lose credit for your participation.
            </p>
            <p>
              Besides helping science, you'll get better recommendations
              plus the ability to save opportunities for later and track
              your progress in science learning.
            </p>
            <div>
              <action-button tertiary @click="(show_didit_logged_out = false) || $emit('signup')">
                Create an account
              </action-button>
              <action-button primary @click="(show_didit_logged_out = false) || $emit('login')">
                Sign In
              </action-button>
            </div>
          </div>
        </b-modal>
        <action-button v-if="!did.didit" principal @click="do_didit">
          I Did This!
        </action-button>
      </div>

      <div class="map" :class="{'open': show_map}">
        <a @click="show_map = false">&laquo; back</a>
        <div ref="map_display" />
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
    </div>

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
import ExternalLink from "~/components/ExternalLink"
import Stars from "~/components/Stars"
import CalendarAdd from "~/components/CalendarAdd"
import SocialButton from "~/components/SocialButton"
import ReadMore from "~/components/ReadMore"

import MapMarker from '~/assets/img/marker.png'
import LocationIcon from '~/assets/img/location-marker.svg?inline'
import TimeIcon from '~/assets/img/calendar.svg?inline'
import KeywordsIcon from '~/assets/img/speech-bubble.svg?inline'
import LikeIcon from '~/assets/img/like.svg?inline'
import SavedIcon from '~/assets/img/saved-science-opportunities.svg?inline'
import StarIcon from '~/assets/img/star-on.svg?inline'
import FlagIcon from '~/assets/img/flag.svg?inline'
import LinkIcon from '~/assets/img/link.svg?inline'
import AtomIcon from '~/assets/img/atom.svg?inline'

export default {
    components: {
        VueMarkdown,

        OpportunityLocation,
        OpportunityTime,
        OpportunityKeywords,
        OpportunityNotice,
        ExternalLink,
        Stars,
        CalendarAdd,
        SocialButton,
        ReadMore,

        LocationIcon,
        TimeIcon,
        KeywordsIcon,
        LikeIcon,
        SavedIcon,
        StarIcon,
        FlagIcon,
        LinkIcon,
        AtomIcon,
    },

    props: {
        entity: {
            type: Object,
            required: true
        },

        user: {
            type: Object,
            required: true,
        },
    },

    data() {
        return {
            new_review: {
                rating: 3,
                comment: "",
            },
            did: {
                like: false,
                save: false,
                didit: false,
            },
            show_didit_logged_out: false,
            show_bookmark_add: false,
            show_review_add: false,
            show_calendar_add: false,
            upcoming: [],
            map_widget: null,
            reviews: null,
            likes: null,
            recommended: null,
            saves: null,
            didit: null,
            show_map: false,
            description_open: false,
            mobile_menu_open: false,
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
            // this.didit is how many people have done it
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

    watch: {
        "user.authenticated": async function(new_val, old_val) {
            if(new_val) {
                this.did = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/me', this.$store.state.auth);
            }
        }
    },

    async mounted() {
        if(this.location_geojson) {
            this.map_widget = new mapboxgl.Map({
                accessToken: this.$config.mapboxToken,
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

        if(this.user.authenticated) {
            this.did = await this.$axios.$get('/api/ui/entity/' + this.opportunity.slug + '/me', this.$store.state.auth);
        }
    },

    methods: {
        async register_interest() {
            await this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/interest', {}, this.$store.state.auth);
        },

        async do_like() {
            await this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/likes', {}, this.$store.state.auth);
            this.did.like = true;
            this.likes += 1;

            this.$buefy.toast.open({
                message: 'Liked it',
                type: 'is-success'
            });
        },

        async do_didit() {
            if(this.user.authenticated) {
                try {
                    await this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/didit', {}, this.$store.state.auth);

                    this.did.didit = true;

                    this.$buefy.toast.open({
                        message: 'Logged participation',
                        type: 'is-success',
                    });
                }
                catch(error) {
                    this.did.didit = true;

                    let state = await this.$store.dispatch('get_local');
                    let didit = state.didit || [];

                    didit.push(this.opportunity.slug);

                    state.didit = didit;
                    await this.$store.dispatch('set_local', state);

                    this.$buefy.toast.open({
                        message: "Saved in your browser, we'll log it later",
                        type: 'is-info',
                    });
                }
            }
            else {
                let state = await this.$store.dispatch('get_local');
                let didit = state.didit || [];

                didit.push(this.opportunity.slug);

                state.didit = didit;
                await this.$store.dispatch('set_local', state);

                this.show_didit_logged_out = true;
            }
        },

        async do_save() {
            if(this.user.authenticated) {
                await this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/saves', {}, this.$store.state.auth);
                this.did.save= true;

                this.$buefy.toast.open({
                    message: 'Opportunity saved',
                    type: 'is-success'
                });
            }
            else {
                this.show_bookmark_add = true;
            }
        },

        async do_review() {
            let {id} = await this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/reviews', this.new_review, this.$store.state.auth);

            // Consider handling the case where the id matches an id
            // in the existing reviews list -- the user updated their
            // review instead of creating a new one.

            this.show_review_add = false;

            this.reviews.average = ((this.reviews.average * this.reviews.reviews.length) + this.new_review.rating)
                / (this.reviews.reviews.length + 1);

            this.reviews.reviews.push({
                id,
                person: this.user.uid,
                username: this.user.username,
                image_url: this.user.image_url,
                rating: this.new_review.rating,
                comment: this.new_review.comment,
                when: (new Date()).toISOString(),
            });

            this.new_review = {
                rating: 3,
                comment: "",
            };

            this.$buefy.toast.open({
                message: 'Review saved',
                type: 'is-success'
            });
        },

        async report_review(id) {
            await this.$axios.$post('/api/ui/entity/' + this.opportunity.slug + '/report-review', {id: id}, this.$store.state.auth);

            this.$buefy.toast.open({
                message: 'Reported review',
                type: 'is-success'
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
    },
}
</script>

<style lang="scss" scoped>
.mobile-menu-toggle {
    position: fixed;
    overflow: hidden;
    bottom: 17px;
    right: 17px;
    background-color: $snm-color-action;
    color: $snm-color-element-dark;
    width: 40px;
    height: 40px;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 20;
    border: 0px;
    box-shadow: 0px 3px 6px $snm-color-shadow;
    font-size: $snm-font-larger;
}

.mobile-menu {
    display: flex;
    position: fixed;
    bottom: 74px;
    right: 17px;
    border-radius: 6px;
    overflow: hidden;
    width: 284px;
    flex-direction: column;
    background-color: $snm-color-background;
    border: 1px solid $snm-color-border-ondark;
    z-index: 20;

    &.closed {
        display: none;
    }

    > a {
        display: flex;
        background-color: $snm-color-action;
        color: $snm-color-element-dark;
        border-bottom: 1px solid $snm-color-background;
        font-family: $snm-font-content;
        font-weight: bold;
        font-size: $snm-font-small;
        letter-spacing: 0px;
        padding: 17px;
        align-items: center;

        svg {
            width: 20px;
            margin-right: 0.5rem;

            * {
                fill: currentColor;
            }
        }
    }

    div {
        display: flex;
        flex-direction: row;
        justify-content: space-between;

        a {
            width: 35px;
            height: 35px;
            margin: 15px 27px;
        }
    }
}

img.opportunity-image {
    width: 100vw;
    height: 158px;
    object-fit: contain;
    object-position: center center;
    overflow: hidden;
}

.ididthis {
    border: 1px solid $snm-color-border;
    border-radius: 6px;
    margin-bottom: 2rem;
    padding-bottom: 1rem;

    >h2 {
      padding: 1rem;
      border-radius: 6px 6px 0 0;
      font-family: $snm-font-heading;
      font-weight: bold;
      background-color: $snm-color-background-medium;
      vertical-align: middle;
      font-size: rem(21px);
      > svg {
        width: 30px;
    margin: 0 0.2rem;
    margin-right: 1rem;
    display: inline-block;
    vertical-align: middle;
      }
    }
    >p {
      padding: 0.5rem 1rem;
    }
    > button {
      width: calc(100% - 2rem);
      margin: 1rem;
    }

    .modal {
        .card {
            h2 {
                font-family: $snm-font-heading;
                font-weight: bold;
                font-size: $snm-font-medium;
                line-height: 26px;
                letter-spacing: 0px;
                color: $snm-color-background-dark;

                .close {
                    float: right;
                    font-size: $snm-font-larger;
                    position: relative;
                    top: -10px;
                    cursor: pointer;
                }
            }

            p {
                font-family: $snm-font-content;
                font-weight: normal;
                font-size: $snm-font-small;
                line-height: 22px;
                letter-spacing: 0px;
                color: $snm-color-tldr;
                margin: 1rem 0px;
            }

            div:last-child {
                display: flex;
                justify-content: right;

                > * {
                    flex-grow: 0;
                }
            }
        }
    }
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
    z-index: 50;

    &.open {
        right: 1vw;
        width: 98vw;
        opacity: 1;
    }

    > a {
        float: right;
    }

    > div {
        display: block;
        width: calc(98vw - 2rem);
        height: calc(98vw - 2rem);
    }
}

.opportunity-section {
  padding:1rem;
}

.opportunity-name {

    strong {
        font-family: $snm-font-content;
        font-weight: bold;
        font-size: $snm-font-smaller;
        color: $snm-color-element-dark;
        line-height: 16px;
        letter-spacing: 0px;
    }

    h1 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-large;
        color: $snm-color-info;
        line-height: 28px;
        letter-spacing: 0px
    }
}

.elevator-pitch {
    font-family: $snm-font-content;
    font-weight: normal;
    font-size: $snm-font-small;
    line-height: 22px;
    letter-spacing: 0.16px;
    color: $snm-color-glance;
}

.involvement {
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;

    .reviews-likes {
        span {
            font-family: $snm-font-content;
            font-size: $snm-font-small;
            line-height: 19px;
            letter-spacing: 0px;
            color: $snm-color-element-dark;

            > :first-child {
                margin-right: 0.75rem;
            }

            svg.liked * {
                fill: $snm-color-info;
            }
        }

        span:not(:first-of-type) {
            margin-left: 3rem;
        }


    }

    > .principal {
      margin-left:0;
    }

    > .numbers {
        margin-top: 0.5rem;
        margin-bottom: 0.2rem;
        display:flex;

        >p {
          margin-right:1rem;
        }
    }

    a.social-button {
      margin-left: 1rem;
    }
}




.info {
    margin-bottom: 1rem;
    display: flex;
    align-items: top;

    &.location {
      margin-bottom: 1.6rem
    }

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
    padding: 1rem;
    position:relative;
    margin:0 1rem;
    border-radius: 6px;
    width: calc(100% - 2rem);

    &::after {
      content: '>';
      display: block;
      width: 40px;
      height: 40px;
      background-color: #fff;
      color: $snm-color-element-med;
      position: absolute;
      top: 50%;
      margin-top: -20px;
      right:2rem;
      text-align: center;
      border-radius:100%;
      line-height:40px;
    }

    strong {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-medium;
        line-height: 26px;
        letter-spacing: 0px;
        display: block;
    }

    span {
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: $snm-font-smaller;
        line-height: 16px;
        letter-spacing: 0px;
    }
}

.find-out-more:hover {
    color: $snm-color-element-ondark;
    background-color: $snm-color-background-meddark;
}

.partner-and-org {
    display: flex;
    justify-content: space-around;
    figure {

        text-align: center;

        figcaption {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-smaller;
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

    > *:not(:first-child) {
        border-top: 1px solid $snm-color-border;
        margin-top: 17px;
        padding-top: 17px;
    }

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
        line-height: 22px;
        letter-spacing: 0px;
        color: $snm-color-background-dark;
    }
}

.description {
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;

    > h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
        line-height: 19px;
        letter-spacing: 0px;
        color: $snm-color-background-dark;
        margin-bottom: 17px;
    }
}

.tags {
    display: flex;
    flex-wrap: wrap;
    border-top: 1px solid $snm-color-border;
    border-bottom: 1px solid $snm-color-border;

    h2 {
        width: 100%;
        margin-bottom: 1rem;
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
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
    font-family: $snm-font-content;
    font-weight: normal;
    font-size: $snm-font-small;
    line-height: 19px;
    letter-spacing: 0px;

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
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
        font-size: $snm-font-small;
        line-height: 19px;
        padding: 17px;
    }

    .review {
        border-bottom: 1px solid $snm-color-border;
        border-top: 1px solid $snm-color-border;
        padding:1rem;

        > :nth-child(1) {
            display: flex;
            justify-content: space-between;

            > :last-child {
                display: flex;
                justify-content: space-between;
                align-items: center;
                width: 3.5rem;
                font-family: $snm-font-meta;
                font-weight: normal;
                font-size: $snm-font-smallest;
                line-height: 15px;
                color: $snm-color-element-med;
            }
        }

        > :nth-child(2) {
            font-family: $snm-font-meta;
            font-weight: normal;
            font-style: italic;
            font-size: $snm-font-smaller;
            line-height: 40px;
            letter-spacing: 0px;
            color: $snm-color-caption;
        }
    }

    .modal {
        h2 {
            font-size: $snm-font-medium;
            line-height: 26px;
            font-weight: bold;
            background-color: transparent;
            color: $snm-color-caption;
        }

        .buttons {
            display: flex;
            justify-content: right;

            > * {
                flex-grow: 0;
            }
        }
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
        font-size: $snm-font-small;
        line-height: 19px;
        padding: 17px;
    }


    > a {
        display: block;
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: $snm-font-smaller;
        line-height: 16px;
        color: $snm-color-element-dark;
        padding: 1rem;
        border-bottom: 1px solid $snm-color-border;

        h3 {
            font-family: $snm-font-heading;
            font-size: $snm-font-medium;
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

.opportunity-left {
  .opportunity-location, .opportunity-time, .opportunity-keywords {
    font-weight: bold;
  }
}

@media (min-width: $fullsize-screen) {
  .opportunity .snm-container {
    display: flex;
  }
  .opportunity-left {
    flex: 0 0 66.66%;
    padding:1rem 0.5rem 1rem 1rem;
  }
  .opportunity-right {
    padding: 1rem 1rem 1rem 0.5rem;
    position: sticky;
    top:0;
    align-self: flex-start;
  }
  .reviews {
      .modal {
          .buttons {
              width: 400px;
          }
      }
  }
  .opportunity-section {
    padding:1.5rem 0;

    h2 {
      font-size: rem(24px);
    }
  }
  .opportunity-name h1 {
    font-size: 2rem;
    margin-bottom: 1rem;
  }
  .opportunity-name strong {
    display: block;
    margin-bottom: 0.5rem;
  }

  .more-info {
    padding: 0;
    margin: 2rem 0;
    border-radius: 6px;
    border: 1px solid $snm-color-border;
    >* {
      padding: 1rem;
      margin: 0!important;
    }
  }
  .description {
    border-top:0;
  }
  .reviews-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
    > button {
      margin:0;
    }
  }
  .reviews{
    border-top: 1px solid $snm-color-border;
  }
  .reviews  h2 {
    display:inline-block;
    font-size: rem(24px);
    font-family: $snm-font-heading;
    font-weight: bold;
    line-height: 22px;
    letter-spacing: 0px;
    color: $snm-color-background-dark;
    background-color: transparent;
    padding:0;
  }
  .reviews > .review {
    padding:1rem;
    margin: 1rem 0;
    border: 1px solid $snm-color-border;
    border-radius: 6px;
  }
  .reviews, .description, .tags, .social {
    padding-top: 2rem;
    padding-bottom: 2rem;
  }
  .find-out-more {
    border-radius: 6px;
  }
  img.opportunity-image {
    height:250px;
  }
  .related {
    > h2 {
      color: $snm-color-element-dark;
      font-size: rem(24px);
      background-color: transparent;
      padding:0;
      margin-bottom:2rem;
      margin-top: 3rem;
    }
    >a {
      border:0;
      padding:0;
      margin-bottom:2rem;
    }
  }
  .find-out-more {
    width: 100%;
    margin:0;
  }
}

@media (min-width: 1200px) {
  .opportunity-left {
    padding:1rem 2rem 1rem 1rem;
  }
}


</style>
