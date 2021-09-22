<template>
<a :href="href" target="_blank" rel="noopener" class="social-button" @click="$emit('click')">
  <twitter-icon v-if="mode == 'twitter'" />
  <facebook-icon v-else-if="mode == 'facebook'" />
  <linkedin-icon v-else-if="mode == 'linkedin'" />
  <span v-else>unknown social network</span>
</a>
</template>

<script>
import TwitterIcon from '~/assets/img/twitter-app.svg?inline'
import FacebookIcon from '~/assets/img/facebook-app.svg?inline'
import LinkedinIcon from '~/assets/img/linkedin-app.svg?inline'

export default {
    name: "SocialButton",

    components: {
        TwitterIcon,
        FacebookIcon,
        LinkedinIcon,
    },

    props: {
        mode: {
            type: String,
            required: true,
            default: 'twitter',
        },

        opportunity: {
            type: Object,
            required: false,
            default: null,
        },

        url: {
            type: String,
            required: false,
            default: '',
        },

        title: {
            type: String,
            required: false,
            default: '',
        },

        hashtags: {
            type: Array,
            required: false,
            default: [],
        },
    },

    computed: {
        href() {
            if(this.opportunity === null && !this.url) {
                return 'missing';
            }

            const encode = encodeURIComponent;

            const url = encode(this.url || ('https://sciencenearme.org/' + this.opportunity.slug));
            const title = encode(this.title || this.opportunity.title);
            const via = encode('SciNearMe_US');
            const hashtags = encode((this.hashtags.length ? this.hashtags : this.opportunity.opp_hashtags).join(',').replace(/#/g, ''));

            switch(this.mode) {
            case 'twitter':
                return 'https://twitter.com/share?url=' + url + '&text=' + title + '&via=' + via + '&hashtags=' + hashtags;
            case 'facebook':
                return 'https://www.facebook.com/sharer.php?u=' + url;
            case 'linkedin':
                return 'https://www.linkedin.com/shareArticle?url=' + url + '&title=' + title;
            default:
                return 'unknown';
            }
        }
    },
}
</script>

<style lang="scss" scoped>
a {
    display: inline-block;
    width: 26px;
    height: 26px;

    svg {
        width: 100%;
        height: 100%;
    }
}
</style>
