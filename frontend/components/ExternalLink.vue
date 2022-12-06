<template><a class="external-link" :href="href" :title="title" rel="noopener" @click.stop.prevent="go"><slot /></a></template>

<script>
/*

  This component is used to create tracked links to external
  resources. It has parameters for setting the UTM query parameters on
  the links, and informs the server of the outgoing link before
  opening the new URL.

*/
export default {
    name: "ExternalLink",

    props: {
        href: {
            type: String,
            required: true
        },

        subject: {
            type: String,
            required: false,
            default: ''
        },

        title: {
            type: String,
            required: false,
            default: ''
        },

        // If true, open link target in a new tab or window
        newTab: {
            type: Boolean,
            required: false,
            default: false
        },

        // Identifies which site sent the traffic
        source: {
            type: String,
            required: false,
            default: 'sciencenearme'
        },

        // Identifies what type of link was used, such as cost per click or email.
        medium: {
            type: String,
            required: false,
            default: 'web'
        },

        // Identifies a specific product promotion or strategic campaign.
        campaign: {
            type: String,
            required: false,
            default: 'general'
        },

        // Identifies search terms.
        term: {
            type: String,
            required: false,
            default: ''
        },

        // Identifies what specifically was clicked to bring the user to the site, such as a banner ad or a text link.
        content: {
            type: String,
            required: false,
            default: 'link'
        },
    },

    computed: {
        linkage() {
            return '' + (Date.now() + Math.random());
        },

        joint () {
            if (!this.href.includes('?')) {
                return '?'
            } else {
                return '&'
            }
        },

        target () {
            let params = ''

            if (this.source) {
                params = params + (params ? '&' : '') + 'utm_source=' + this.source;
            }

            if (this.medium) {
                params = params + (params ? '&' : '') + 'utm_medium=' + this.medium;
            }

            if (this.campaign) {
                params = params + (params ? '&' : '') + 'utm_campaign=' + this.campaign;
            }

            if (this.term) {
                params = params + (params ? '&' : '') + 'utm_term=' + this.term;
            }

            if (this.content) {
                params = params + (params ? '&' : '') + 'utm_content=' + this.content;
            }

            params = params + (params ? '&' : '') + 'snml=' + this.linkage;

            if (params.length) {
                return this.href + this.joint + params
            } else {
                return this.href
            }
        }
    },

    methods: {
        async go () {
            this.$gtm.push({ event: 'external_link', url: this.href });

            // Running it directly rather than using $emit so that we
            // can wait on the promise if it's async
            if(this.$listeners.before) {
                await this.$listeners.before(this.href);
            }

            await this.$axios.$post('/api/ui/activity/external', {
                session: window.localStorage.getItem('token') || '',
                on_page: window.location.href,
                href: this.href,
                title: this.title,
                source: this.source,
                medium: this.medium,
                campaign: this.campaign,
                term: this.term,
                content: this.content,
                snml: this.linkage,
                subject: this.subject
            })

            if (this.newTab) {
                window.open(this.target, '_blank', "noopener=yes")
            } else {
                window.location = this.target
            }
        }
    }
}
</script>
