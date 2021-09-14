<template>
<p>
  {{ selected_partner }}
</p>
</template>

<script>
export default {
    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });
        }

        let partners = [];

        try {
            partners = await context.$axios.$get('/api/ui/profile/partners', this.$store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        return {
            partners,
        }
    },

    data() {
        return {
            partner_index: 0,
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },

        choose_partner() {
            return this.partners.length > 1;
        },

        selected_partner() {
            return this.partners[this.partner_index] || null;
        },
    },
}
</script>

<style lang="scss" scoped>

</style>
