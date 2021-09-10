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
            required: true,
            default: null,
        },
    },

    computed: {
        href() {
            if(this.opportunity === null) {
                return 'missing';
            }

            const encode = encodeURIComponent;

            const url = encode('https://sciencenearme.org/' + this.opportunity.slug);
            const title = encode(this.opportunity.title);
            const via = encode('sciencenearme_');
            const hashtags = encode(this.opportunity.opp_hashtags.join(',').replace(/#/g, ''));

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
