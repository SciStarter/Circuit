<template>
<section class="homepage">
  <h1>What would you like to do <near-icon class="inline-sign" /> {{ city }}?</h1>
  <sideways-slider>
    <div v-for="intent in intents" :key="intent.title" class="intent-card">
      <img :title="intent.title" :src="intent.image" :srcset="intent.image + ' 1x,' + intent.image2x + ' 2x'">
      <nuxt-link :to="intent.link">
        {{ intent.title }}
      </nuxt-link>
      <p>
        {{ intent.description }}
      </p>
    </div>
  </sideways-slider>
</section>
</template>

<script>
import Structures from '~/assets/lib/structures'
import DynamicBlock from '~/components/DynamicBlock'
import SidewaysSlider from '~/components/SidewaysSlider'

import NearIcon from '~/assets/img/near.svg?inline'
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

export default {
    name: 'HomePage',

    components: {
        DynamicBlock,
        SidewaysSlider,

        NearIcon,
    },

    async asyncData() {
        let intents = [
            {'link': '/find?', 'title': 'Listen, Learn, Discuss, Inform', 'description': 'Participate in live dialogues about current science and society issues', 'image': LearnDiscussImage, 'image2x': LearnDiscussImage2x},
            {'link': '/find?', 'title': 'Create or Build', 'description': 'Be creative and do something hands-on', 'image': CreateBuildImage, 'image2x': CreateBuildImage2x, 'order': Math.floor(Math.random() * 100)},
            {'link': '/find?', 'title': 'Explore Earth and Space', 'description': 'Feed your curiosity with an expert guide', 'image': ExploreSpaceImage, 'image2x': ExploreSpaceImage2x},
            {'link': '/find?', 'title': 'Celebrate Science', 'description': 'Go to a science festival', 'image': CelebrateScienceImage, 'image2x': CelebrateScienceImage2x},
            {'link': '/find?', 'title': 'Make a Difference', 'description': 'Participate in science or serve your community', 'image': MakeDifferenceImage, 'image2x': MakeDifferenceImage2x},
            {'link': '/find?', 'title': 'For Kids', 'description': 'Where kids can explore, learn, and get excited about science', 'image': ForKidsImage, 'image2x': ForKidsImage2x},
        ];

        Structures.random_order(intents);

        return {
            intents,
        };
    },

    data() {
        return {
            show_login: false,
            show_signup: false,
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
        }
    },
}
</script>

<style lang="scss" scoped>
.homepage {
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

        >img {
            width: 100%;
            height: auto;
        }

        >a {
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
}

@media (min-width: $fullsize-screen) {
    .homepage {
        h1 {
            font-size: $snm-font-largest;
            text-align: center;
        }

        .sideways-slider {
            max-width: calc(100vw - 25rem);
            width: 77rem;
        }

        .intent-card {
            width: 25rem;
        }
    }
}
</style>
