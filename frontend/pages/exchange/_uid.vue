<template>
<nuxt-child :partner="partner" :exchange="exterior" :style="custom_props"/>
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
