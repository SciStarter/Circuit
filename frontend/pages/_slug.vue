<template>
  <div>
<div v-if="authorization_required">
  This page requires you to <a @click="$router.push({name: 'login', query: {next: $route.path}})">log in</a>
  to an authorized account in order to view it, or perhaps it doesn't exist at all.
</div>
<component v-else :is="selected_component" :entity="entity" :user="user" :from-search="fromSearch" @login="$parent.$emit('login')" @signup="$parent.$emit('signup')" />
<PageView/>
</div>
</template>

<script>
// After creating a new layout component, update the
// selected_component() computed property to map a layout name to the
// component, and update the PageLayout or EntityType enumeration (as
// appropriate for the component) in common/src/model/opportunity/mod.rs

import showdown from 'showdown';

export default {
    name: 'Dynamic',

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'strict-origin-when-cross-origin',
        };
    },

    data() {
        return {
            fromSearch: false,
        };
    },

    beforeRouteEnter(to, from, next) {
        next(it => {
            it.fromSearch = (from.path == '/find');
        });
    },

    async asyncData ({ params, error, $axios, store }) {
        const user = await store.dispatch('get_user');

        try {
            const entity = await $axios.$get('/api/ui/entity/' + params.slug, store.state.auth ? store.state.auth : undefined);
            const layout = entity.entity_type.page ? entity.entity_type.page.layout : entity.entity_type;

            return { entity, layout, 'authorization_required': (entity.authorized !== 'manage') && (!entity.accepted || entity.withdrawn || entity.review_status === 'draft' || entity.review_status === 'pending' || entity.review_status === 'reject') };
        } catch(x) {
            console.warn(x);

            error({
                statusCode: 404,
                message: "Not Found"
            });

            return {};
        }
    },

    head () {
        let converter = new showdown.Converter();

        return {
            'title': this.entity.title + ' - Science Near Me',
            'meta': [
                { hid: 'description', name: 'description', content: converter.makeHtml(this.entity.short_desc || this.entity.description) },
                { hid: 'og:description', property: 'og:description', content: converter.makeHtml(this.entity.short_desc || this.entity.description) },
                { hid: 'og:title', property: 'og:title', content: this.entity.title + ' - Science Near Me'},
                { hid: 'og:url', property: 'og:url', content: 'https://sciencenearme.org/' + this.entity.slug },
                { hid: 'og:image', property: 'og:image', content: this.entity.image_url || require('~/assets/img/logo.jpg') },
                { hid: 'og:type', property: 'og:type', content: 'article' },
            ]
        };
    },

    computed: {
        selected_component() {
            return {
                'opportunity': () => import('~/components/Opportunity'),
                'just_content': () => import('~/components/JustContent'),
                'add_opportunities': () => import('~/components/AddOpportunities'),
            }[this.layout] || (() => import('~/components/Opportunity'));
        },

        user() {
            return this.$store.state.user;
        },
    },

    mounted() {
        this.$gtm.push({
            //event: 'view_entity',
            uid: this.entity.uid,
            title: this.entity.title,
            partner: this.entity.partner,
            partner_name: this.entity.partner_name,
            activity_types: this.entity.opp_descriptor,
            domain: this.entity.pes_domain,
        });

        let prior = window.localStorage.getItem('last-opportunity');
        let postor = this.entity.uid;

        if(!!postor && this.entity.entity_type == 'opportunity') {
            if(!!prior) {
                this.$axios.$post('/api/ui/activity/transit', {prior, postor}, this.$store.state.auth);
            }

            window.localStorage.setItem('last-opportunity', postor);
        }
    },
}
</script>
