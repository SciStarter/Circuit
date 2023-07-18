<template>
<a :href="href" target="_blank" rel="noopener" class="social-button" @click="$emit('click')">
  <twitter-icon v-if="mode == 'twitter'" />
  <facebook-icon v-else-if="mode == 'facebook'" />
  <linkedin-icon v-else-if="mode == 'linkedin'" />
  <link-icon v-else-if="mode == 'link'" />
  <span v-else>unknown social network</span>
</a>
</template>

<script>
import TwitterIcon from '~/assets/img/twitter-app.svg?inline'
import FacebookIcon from '~/assets/img/facebook-app.svg?inline'
import LinkedinIcon from '~/assets/img/linkedin-app.svg?inline'
import LinkIcon from '~/assets/img/link.svg?inline'

export default {
    name: "SocialButton",

    components: {
        TwitterIcon,
        FacebookIcon,
        LinkedinIcon,
        LinkIcon,
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
            default: () => [],
        },
    },

    computed: {
        href() {
            if(this.opportunity === null && !this.url) {
                return 'missing';
            }

            const encode = encodeURIComponent;

            let via = '';
            let handle = '';
            let title = '';
            const url = encode(this.url || ('https://sciencenearme.org/' + this.opportunity.slug));
            const hashtags = encode((this.hashtags.length ? this.hashtags : this.opportunity.opp_hashtags).join(',').replace(/#/g, ''));

            switch(this.mode) {
            case 'twitter':
                via = encode('science_near_me');
                handle = (!!this.opportunity?.opp_social_handles?.twitter) ? (' @' + this.opportunity.opp_social_handles.twitter) : "";
                title = encode((this.title || this.opportunity.title) + handle);
                return 'https://twitter.com/share?url=' + url + '&text=' + title + '&via=' + via + '&hashtags=' + hashtags;
            case 'facebook':
                via = encode('https://www.facebook.com/find.science.near.me');
                handle = (!!this.opportunity?.opp_social_handles?.facebook) ? (' from ' + this.opportunity.opp_social_handles.facebook) : "";
                title = encode((this.title || this.opportunity.title) + handle + ' via ' + via);
                return 'https://www.facebook.com/sharer.php?display=page&u=' + url + '&quote=' + title;
            case 'linkedin':
                via = encode('science-near-me');
                return 'https://www.linkedin.com/sharing/share-offsite/?url=' + url;
            case 'link':
                return this.url || ('https://sciencenearme.org/' + this.opportunity.slug);
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
