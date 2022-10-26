<template>
<div :style="custom_props">
    <nuxt-child :partner="partner" :exchange="exterior" />
</div>
</template>

<script>
export default {
    name: "ExchangeUID",

    async asyncData(context) {
        let partner = null;
        let exterior = null;

        const user = await context.store.dispatch('get_user');

        try {
            exterior = partner = await context.$axios.$get(
                '/api/ui/organization/' + context.params.uid,
                context.store.state.auth,
            );
        }
        catch(err) {
            // Just means the user is not a partner member
            exterior = await context.$axios.$get(
                '/api/ui/organization/' + context.params.uid + '/public',
            );

            let split = context.route.fullPath.indexOf('?');
            if(split > -1) {
                exterior.default_query = context.route.fullPath.slice(split + 1);
            }
        }

        return {
            partner,
            exterior,
            show_login: false,
            show_signup: false,
        }
    },

    computed: {
        custom_props() {
            if(!this.exterior) {
                return {}
            }

            return {
                'color': this.exterior.primary_color,
                '--background-color': this.exterior.background_color,
                '--primary-color': this.exterior.primary_color,
                '--secondary-color': this.exterior.secondary_color,
                '--tertiary-color': this.exterior.tertiary_color,
                '--logo-url': this.exterior.image_url ? 'url(' + this.exterior.image_url + ')' : '',
            };
        },
    },

    watch: {
        "$store.state.auth": async function(val) {
            try {
                this.partner = await this.$axios.$get(
                    '/api/ui/organization/' + this.$route.params.uid,
                    val,
                );
            }
            catch(err) {}
        },
    },
}
</script>

<style lang="scss" scoped>
* {
    background-color: var(--background-color, #fff);
}
</style>

<style lang="scss">
a {
    color: var(--secondary-color, #087a91)
}
</style>
