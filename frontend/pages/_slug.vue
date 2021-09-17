<template>
<component :is="selected_component" :entity="entity" :user="user" @login="$parent.$emit('login')" @signup="$parent.$emit('signup')" />
</template>

<script>
// After creating a new layout component, update the
// selected_component() computed property to map a layout name to the
// component, and update the PageLayout or EntityType enumeration (as
// appropriate for the component) in common/src/model/opportunity/mod.rs

export default {
    name: 'Dynamic',

    async asyncData ({ params, $axios }) {
        const entity = await $axios.$get('/api/ui/entity/' + params.slug);
        const layout = entity.entity_type.page ? entity.entity_type.page.layout : entity.entity_type;

        return { entity, layout };
    },

    head () {
        return {
            'title': this.entity.title + ' - Science Near Me',
            'meta': [
                { hid: 'description', name: 'description', content: this.entity.short_desc || this.entity.description },
                { hid: 'og:description', property: 'og:description', content: this.entity.short_desc || this.entity.description },
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
            }[this.layout] || Opportunity;
        },

        user() {
            return this.$store.state.user;
        },
    },
}
</script>
