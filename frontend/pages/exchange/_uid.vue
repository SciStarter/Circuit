<template>
<nuxt-child :partner="partner"/>
</template>

<script>
export default {
    name: "ExchangeUID",

    async asyncData(context) {
        let partner = null;

        const user = await context.store.dispatch('get_user');

        try {
            partner = await context.$axios.$get(
                '/api/ui/organization/' + context.params.uid,
                context.store.state.auth,
            );
        }
        catch(err) {
            // Just means the user is not a partner member
        }

        return {
            partner,
            show_login: false,
            show_signup: false,
        }
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
