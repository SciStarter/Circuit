<template>
<section id="homepage">
  <general-filters
    class="no-mobile"
    :text="search_text"
    :place="search_place"
    :beginning="search_beginning"
    :ending="search_ending"
    :include-online="search_online"
    search-button
    quick-links
    @text="search_text=$event"
    @place="search_place=$event"
    @beginning="search_beginning=$event"
    @ending="search_ending=$event"
    @include-online="search_online=$event"
    />
  <h1>What would you like to do <near-icon class="inline-sign" /> {{ city }}?</h1>
  <div id="atom-placement">
    <sideways-slider>
      <div v-for="intent in intents" :key="intent.title" class="intent-card">
        <nuxt-link :to="intent.link">
          <img :title="intent.title" :src="intent.image" :srcset="intent.image + ' 1x,' + intent.image2x + ' 2x'">
        </nuxt-link>
        <nuxt-link :to="intent.link">
          {{ intent.title }}
        </nuxt-link>
        <p>
          {{ intent.description }}
        </p>
      </div>
    </sideways-slider>
    <atom-icon id="hydrogen" />
    <atom-icon id="helium" />
  </div>
  <div id="here-now">
    <h2>Here &amp; Now near {{ city }}</h2>
    <div id="opportunity-cards">
      <opportunity-card v-for="opp in here_and_now" :key="opp.uid" :opportunity="opp" />
    </div>
    <nuxt-link :to="here_and_now_link">
      See More Here &amp; Now Opportunities
    </nuxt-link>
  </div>
  <div id="by-topic">
    <h2>Find &amp; Do Science By Topic</h2>
    <b-dropdown aria-role="list" class="mobile-only">
      <template #trigger="{ active }">
        <b-button
          label="Select Topic"
          type="is-info"
          :icon-right="active ? 'menu-up' : 'menu-down'"
          />
      </template>
      <b-dropdown-item v-for="topic in topics" :key="topic[0] + '-mobile'" custom aria-role="listitem">
        <nuxt-link :to="here_and_now_link + '&topics[]=' + topic[0]">
          {{ topic[1].replaceAll(' And ', ' & ') }}
        </nuxt-link>
      </b-dropdown-item>
    </b-dropdown>
    <div class="topics no-mobile">
      <nuxt-link v-for="topic in topics" :key="topic[0]" :to="here_and_now_link + '&topics[]=' + topic[0]">
        <component :is="topic[0].replaceAll('_', '-') + '-icon'" />
        <span>{{ topic[1].replaceAll(' And ', ' & ') }}</span>
      </nuxt-link>
    </div>
  </div>
  <div v-if="!username" id="benefits">
    <h2>Benefits of Creating an Account</h2>

    <div>
      <benefits-recommendations />
      <div>
        <strong>Get Customized Recommendations</strong>
        <p>
          As you search and save events and projects, our system will
          learn your interests and help you find better science events
          and projects for you!
        </p>
      </div>
    </div>

    <div>
      <benefits-save />
      <div>
        <strong>Save Opportunities</strong>
        <p>
          Find something you’d like to do in the future? Save events and
          projects, and even set reminders, by creating an account.
        </p>
      </div>
    </div>

    <div>
      <benefits-research />
      <div>
        <strong>Help Science Research</strong>
        <p>
          ScienceNearMe is a National Science Foundation funded project
          to study public engagement with science and informal science
          learning. Reporting your science will help scientists
          understand how people do science!
        </p>
      </div>
    </div>

    <action-button principal arrow @click="$parent.$emit('signup')">
      Create an Account Now
    </action-button>
  </div>
</section>
</template>

<script>
import Structures from '~/assets/lib/structures'
import DynamicBlock from '~/components/DynamicBlock'
import SidewaysSlider from '~/components/SidewaysSlider'
import OpportunityCard from '~/components/OpportunityCard'
import ActionButton from '~/components/ActionButton'
import GeneralFilters from '~/components/GeneralFilters'

import NearIcon from '~/assets/img/near.svg?inline'
import AtomIcon from '~/assets/img/atom.svg?inline'
import CelebrateScienceImage from '~/assets/img/celebrate-science.jpg'
import CelebrateScienceImage2x from '~/assets/img/celebrate-science@2x.jpg'
import CreateBuildImage from '~/assets/img/create-build.jpg'
import CreateBuildImage2x from '~/assets/img/create-build@2x.jpg'
import ExploreSpaceImage from '~/assets/img/explore-space.jpg'
import ExploreSpaceImage2x from '~/assets/img/explore-space@2x.jpg'
import ForKidsImage from '~/assets/img/for-kids.jpg'
import ForKidsImage2x from '~/assets/img/for-kids@2x.jpg'
import LearnDiscussImage from '~/assets/img/learn-discuss.jpg'
import LearnDiscussImage2x from '~/assets/img/learn-discuss@2x.jpg'
import MakeDifferenceImage from '~/assets/img/make-difference.jpg'
import MakeDifferenceImage2x from '~/assets/img/make-difference@2x.jpg'

import AgricultureIcon from '~/assets/img/agriculture.svg?inline'
import AnimalsIcon from '~/assets/img/animals.svg?inline'
import ArchaeologyAndCulturalIcon from '~/assets/img/archaeology-and-cultural.svg?inline'
import ArtIcon from '~/assets/img/art.svg?inline'
import AstronomyAndSpaceIcon from '~/assets/img/astronomy-and-space.svg?inline'
import AwardsIcon from '~/assets/img/awards.svg?inline'
import BiologyIcon from '~/assets/img/biology.svg?inline'
import BirdsIcon from '~/assets/img/birds.svg?inline'
import ChemistryIcon from '~/assets/img/chemistry.svg?inline'
import ClimateAndWeatherIcon from '~/assets/img/climate-and-weather.svg?inline'
import ComputersAndTechnologyIcon from '~/assets/img/computers-and-technology.svg?inline'
import CrowdFundingIcon from '~/assets/img/crowd-funding.svg?inline'
import DesignIcon from '~/assets/img/design.svg?inline'
import DisasterResponseIcon from '~/assets/img/disaster-response.svg?inline'
import EcologyAndEnvironmentIcon from '~/assets/img/ecology-and-environment.svg?inline'
import EducationIcon from '~/assets/img/education.svg?inline'
import EngineeringIcon from '~/assets/img/engineering.svg?inline'
import FoodIcon from '~/assets/img/food.svg?inline'
import GeneralScienceIcon from '~/assets/img/atom.svg?inline'
import GeographyIcon from '~/assets/img/geography.svg?inline'
import GeologyAndEarthScienceIcon from '~/assets/img/geology-and-earth-science.svg?inline'
import HealthAndMedicineIcon from '~/assets/img/health-and-medicine.svg?inline'
import InsectsAndPollinatorsIcon from '~/assets/img/insects-and-pollinators.svg?inline'
import MathematicsIcon from '~/assets/img/mathematics.svg?inline'
import NatureAndOutdoorsIcon from '~/assets/img/nature-and-outdoors.svg?inline'
import OceanWaterMarineIcon from '~/assets/img/ocean-water-and-marine-science.svg?inline'
import PaleontologyIcon from '~/assets/img/paleontology.svg?inline'
import PhysicsIcon from '~/assets/img/physics.svg?inline'
import PolicyIcon from '~/assets/img/policy.svg?inline'
import PsychologyIcon from '~/assets/img/psychology.svg?inline'
import ReligionIcon from '~/assets/img/religion.svg?inline'
import RoboticsIcon from '~/assets/img/robotics.svg?inline'
import SocialScienceIcon from '~/assets/img/social-science.svg?inline'
import SoundIcon from '~/assets/img/sound.svg?inline'
import TechnologyIcon from '~/assets/img/computers-and-technology.svg?inline'
import TransportationIcon from '~/assets/img/transportation.svg?inline'

import BenefitsRecommendations from '~/assets/img/benefits-1.svg?inline'
import BenefitsSave from '~/assets/img/benefits-2.svg?inline'
import BenefitsResearch from '~/assets/img/benefits-3.svg?inline'

export default {
    name: 'HomePage',

    components: {
        DynamicBlock,
        SidewaysSlider,
        OpportunityCard,
        ActionButton,

        NearIcon,
        AtomIcon,
        AgricultureIcon,
        AnimalsIcon,
        ArchaeologyAndCulturalIcon,
        ArtIcon,
        AstronomyAndSpaceIcon,
        AwardsIcon,
        BiologyIcon,
        BirdsIcon,
        ChemistryIcon,
        ClimateAndWeatherIcon,
        ComputersAndTechnologyIcon,
        CrowdFundingIcon,
        DesignIcon,
        DisasterResponseIcon,
        EcologyAndEnvironmentIcon,
        EducationIcon,
        EngineeringIcon,
        FoodIcon,
        GeneralScienceIcon,
        GeographyIcon,
        GeologyAndEarthScienceIcon,
        HealthAndMedicineIcon,
        InsectsAndPollinatorsIcon,
        MathematicsIcon,
        NatureAndOutdoorsIcon,
        OceanWaterMarineIcon,
        PaleontologyIcon,
        PhysicsIcon,
        PolicyIcon,
        PsychologyIcon,
        ReligionIcon,
        RoboticsIcon,
        SocialScienceIcon,
        SoundIcon,
        TechnologyIcon,
        TransportationIcon,

        BenefitsRecommendations,
        BenefitsSave,
        BenefitsResearch,
    },

    async asyncData(context) {
        let now = new Date();
        let beginning = encodeURIComponent(now.toISOString());

        let topics = await context.$axios.$get('/api/ui/finder/topics');

        let intents = [
            {'link': '/find?physical=in-person-or-online&beginning=' + beginning + '&sort=closest&descriptors[]=policy&descriptors[]=forum', 'title': 'Listen, Learn, Discuss, Inform', 'description': 'Participate in live dialogues about current science and society issues', 'image': LearnDiscussImage, 'image2x': LearnDiscussImage2x},
            {'link': '/find?physical=in-person-or-online&beginning=' + beginning + '&sort=closest&descriptors[]=maker&descriptors[]=maker_faire', 'title': 'Create or Build', 'description': 'Be creative and do something hands-on', 'image': CreateBuildImage, 'image2x': CreateBuildImage2x, 'order': Math.floor(Math.random() * 100)},
            {'link': '/find?physical=in-person-or-online&beginning=' + beginning + '&sort=closest&descriptors[]=star_party', 'title': 'Explore Earth and Space', 'description': 'Feed your curiosity with an expert guide', 'image': ExploreSpaceImage, 'image2x': ExploreSpaceImage2x},
            {'link': '/find?physical=in-person-or-online&beginning=' + beginning + '&sort=closest&descriptors[]=festival', 'title': 'Celebrate Science', 'description': 'Go to a science festival', 'image': CelebrateScienceImage, 'image2x': CelebrateScienceImage2x},
            {'link': '/find?physical=in-person-or-online&beginning=' + beginning + '&sort=closest&descriptors[]=citizen_science', 'title': 'Make a Difference', 'description': 'Participate in science or serve your community', 'image': MakeDifferenceImage, 'image2x': MakeDifferenceImage2x},
            {'link': '/find?physical=in-person-or-online&beginning=' + beginning + '&sort=closest&max_age=13', 'title': 'For Kids', 'description': 'Where kids can explore, learn, and get excited about science', 'image': ForKidsImage, 'image2x': ForKidsImage2x},
        ];

        Structures.random_order(intents);

        return {
            topics,
            intents,
            here_and_now: [],
        };
    },

    data() {
        return {
            search_text: "",
            search_place_edit: null,
            search_beginning: new Date().toISOString().slice(0, 10),
            search_ending: null,
            search_online: true,
            show_login: false,
            show_signup: false,
            load_here_and_now: false,
        };
    },

    head() {
        return {
            'meta': [
                { hid: 'og:image', property: 'og:image', content: require('~/assets/img/logo.jpg') },
            ]
        };
    },

    computed: {
        search_place: {
            get() {
                if(this.search_place_edit === null) {
                    return this.$store.state.here;
                }

                return this._search_place;
            },

            set(val) {
                this._search_place = val;
            }
        },

        here_and_now_query() {
            if(!this.load_here_and_now) {
                return {};
            };

            let now = new Date();

            return {
                'beginning': now.toISOString(),
                'near': this.$store.state.here.near,
                'longitude': this.$store.state.here.longitude,
                'latitude': this.$store.state.here.latitude,
                'sort': 'closest',
            };
        },

        here_and_now_link() {
            const data = this.here_and_now_query;

            if(data.sort === undefined) {
                return '/find?sort=closest';
            }

            return '/find?beginning=' + encodeURIComponent(data['beginning']) +
                '&near=' + encodeURIComponent(data['near']) +
                '&longitude=' + data['longitude'] +
                '&latitude=' + data['latitude'] +
                '&sort=' + data['sort'];
        },

        username() {
            return this.$store.state.user.username;
        },

        city() {
            if(!this.$store.state.here.near) {
                return 'you';
            }

            const parts = this.$store.state.here.near.split(',');

            if(parts.length == 0) {
                return 'you';
            }

            if(parts.length <= 3) {
                return parts[0];
            }

            return parts.slice(-3)[0];
        },
    },

    watch: {
        async here_and_now_query(query) {
            let result = await this.$axios.$get('/api/ui/finder/search', { params: query });
            this.here_and_now = result.matches.slice(0, 6);
        },
    },

    mounted() {
        this.load_here_and_now = true;
    },
}
</script>

<style lang="scss" scoped>
#homepage {
    padding: 0.5rem 1rem;

    h1 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
        color: $snm-color-element-dark;
    }

    .inline-sign {
        vertical-align: middle;
        width: auto;
        height: 1.75em;
    }

    .sideways-slider {
        margin: 1rem auto;
    }

    .intent-card {
        display: flex;
        flex-direction: column;
        border-radius: 6px;
        overflow: hidden;
        width: 15rem;
        border: 1px solid $snm-color-card;

        &:not(:last-child) {
            margin-right: 1rem;
        }

        >a:first-child>img {
            width: 100%;
            height: auto;
        }

        >a:not(:first-child) {
            font-family: $snm-font-heading;
            font-weight: bold;
            text-decoration: underline;
            font-size: $snm-font-large;
            color: $snm-color-element-med;
            margin: 0.5rem 1rem;
        }

        >p {
            margin: 0.5rem 1rem;
        }
    }

    #atom-placement {
        position: relative;
        margin-bottom: 5rem;

        #hydrogen {
            display: none;
        }

        #helium {
            position: absolute;
            bottom: -4rem;
            right: 16rem;
            width: 2rem;
        }
    }

    #here-now {
        >h2 {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-small;
            color: $snm-color-element-dark;
        }

        >a {
            font-family: $snm-font-content;
            font-size: $snm-font-small;
            text-decoration: underline;
            color: $snm-color-element-med;
        }
    }

    #by-topic {
        >h2 {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-small;
            color: $snm-color-element-dark;
            margin-top: 2rem;
        }

        >div.dropdown::v-deep .dropdown-trigger button {
            width: 90vw;
            justify-content: space-between;
            background-color: $snm-color-element-med;

            >span.icon {
                font-size: 2rem;
            }
        }
    }

    #benefits {
        display: flex;
        flex-direction: column;
        position: relative;
        background-color: $snm-color-background-medlight;
        margin-top: 4rem;
        padding: 1rem;

        &::before {
            top: -2rem;
            left: 50%;
            background: linear-gradient(to right top, $snm-color-background-medlight 50%, transparent 50%);
            display: block;
            content: "";
            position: absolute;
            width: 50%;
            height: 2rem;
        }

        &::after {
            top: -2rem;
            left: 0px;
            background: linear-gradient(to right bottom, transparent 50%, $snm-color-background-medlight 50%);
            display: block;
            content: "";
            position: absolute;
            width: 50%;
            height: 2rem;
        }

        >h2 {
            width: 100%;
            text-align: center;
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-large;
            color: $snm-color-element-dark;
        }

        >div {
            display: flex;
            flex-direction: row;
            align-items: flex-start;
            margin: 1.5rem 0px;

            strong {
                font-family: $snm-font-heading;
                font-weight: bold;
                font-size: $snm-font-small;
                color: $snm-color-background-dark;
            }

            p {
                font-family: $snm-font-content;
                font-size: $snm-font-smaller;
                color: $snm-color-element-dark;
            }

            >svg {
                width: 5rem;
                margin-right: 1rem;
                flex-grow: 0;
                flex-shrink: 0;
            }
        }
    }
}

@media (min-width: $fullsize-screen) {
    #homepage {
        padding: 0px;

        h1 {
            font-size: $snm-font-largest;
            text-align: center;
            margin: 2rem;
        }

        .sideways-slider {
            max-width: calc(100vw - 25rem);
            width: 77rem;
        }

        .intent-card {
            width: 25rem;
        }

        #atom-placement {
            #hydrogen {
                display: block;
                position: absolute;
                bottom: -3rem;
                left: 18rem;
            }
        }

        #here-now {
            position: relative;
            margin: 0px auto;
            max-width: calc(100vw - 25rem);
            width: 77rem;

            >h2 {
                font-size: $snm-font-large;
                margin-bottom: 1rem;
            }

            >a {
                position: absolute;
                top: 0.5rem;
                right: 1rem;
            }

            #opportunity-cards {
                display: flex;
                flex-wrap: wrap;
            }
        }


        #by-topic {
            margin: 0px auto;
            max-width: calc(100vw - 25rem);
            width: 77rem;

            >.topics {
                display: flex;
                flex-wrap: wrap;

                >* {
                    display: flex;
                    align-items: center;
                    width: 12vw;
                    padding: 0.5rem;

                    svg {
                        width: 2rem;
                        margin-right: 1em;
                    }

                    span {
                        font-family: $snm-font-content;
                        font-size: $snm-font-small;
                        color: $snm-color-background-meddark;
                    }
                }
            }
        }

        #benefits {
            flex-direction: row;
            flex-wrap: wrap;
            justify-content: center;
            max-width: calc(100vw - 25rem);
            width: 77rem;
            margin-left: auto;
            margin-right: auto;
            border-bottom-left-radius: 1rem;
            border-bottom-right-radius: 1rem;
            top: 2rem;

            >div {
                width: 33%;
                flex-direction: column;
                align-items: center;
                padding: 1rem;

                >svg {
                    width: 7rem;
                }
            }
        }
    }
}
</style>
