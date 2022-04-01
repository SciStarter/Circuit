<template>
<div class="exchange-wrapper">
<opportunity-form
  v-if="authorized"
  v-model="opp"
  :partner="partner"
  :timezones="timezones"
  :descriptors="descriptors"
  :topics="topics"
  in-exchange 
  />

<div v-else>
  Sorry, but you are not authorized to add opportunities to this Science Near Me partner.
</div>
</div>
</template>

<script>
export default {
    name: "ExchangeSubmit",

    props: {
        partner: {
            type: Object,
            required: true,
        }
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            return { authenticated: false };
        }

        let timezones = [];
        let descriptors = [];
        let topics = [];
        let opp = null;

        try {
            timezones = await context.$axios.$get('/api/ui/timezone', context.store.state.auth);
            descriptors = await context.$axios.$get('/api/ui/finder/descriptors');
            topics = await context.$axios.$get('/api/ui/finder/topics');
            opp = await context.$axios.$get('/api/ui/opportunity/');
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        return {
            authenticated: true,
            timezones,
            descriptors,
            topics,
            opp,
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
.exchange-wrapper {
  padding:1rem;
}
  .exchange .form-actions {
    width:100%!important;
  }
</style>
