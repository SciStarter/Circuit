<template>
<div v-if="authorized">
<h1>Your ScienceNearMe Partner Organization</h1>

<partner-form
  :partner="partner"
  :org_types="org_types"
  :managers="managers"
  :pending="pending"/>
</div>
<div v-else>
  Sorry, you're not authorized to edit this partner's data.
</div>
</template>

<script>
export default {
    name: "ExchangePartner",

    props: {
        partner: {
            type: Object,
            required: true,
        },
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            return { authenticated: false };
        }

        let org_types = [];
        let managers = [];
        let pending = [];

        try {
            org_types = await context.$axios.$get('/api/ui/organization/types', context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        try {
            managers = await context.$axios.$get('/api/ui/organization/' + context.params.uid + '/managers', context.store.state.auth);
            pending = await context.$axios.$get('/api/ui/organization/' + context.params.uid + '/pending-managers', context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        return {
            authenticated: true,
            org_types,
            managers,
            pending,
        }
    },

    computed: {
        authorized() {
            return this.authenticated && this.partner;
        }
    }
}
</script>

<style lang="scss" scoped>
h1 {
    font-family: $snm-font-heading;
    font-size: 1.8rem;
    font-weight:bold;
    color: $snm-color-element-med;
    margin-bottom:2rem;
}
</style>
