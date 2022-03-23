<template>
<div class="exchange exchange-index">
  <div v-if="$store.state.user.authenticated">
    <a @click="$store.dispatch('logout')">Logout</a>
  </div>
  <div v-else>
    <nuxt-link :to="{name: 'exchange-uid-login', params: {uid: $route.params.uid}, query: {next: $route.path}}">Login</nuxt-link> |
    <nuxt-link :to="{name: 'exchange-uid-signup', params: {uid: $route.params.uid}, query: {next: $route.path}}">Signup</nuxt-link>
  </div>

  <div v-if="partner !== null">
    <nuxt-link :to="{name: 'exchange-uid-submit', params: {uid: partner.uid}}">Add an Opportunity</nuxt-link> |
    <nuxt-link :to="{name: 'exchange-uid-partner', params: {uid: partner.uid}}">Manage Partner Organization</nuxt-link> |
    <nuxt-link :to="{name: 'exchange-uid-opps', params: {uid: partner.uid}}">Manage Opportunities</nuxt-link>
  </div>

  <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :partner="partner" previous-page="find" />
</div>
</template>

<script>
export default {
    name: "ExchangeIndex",

    props: {
        partner: {
            type: [Object, null],
            required: true,
        },
    },

    async asyncData(context) {
        let query = {...context.query};

        if(Object.getOwnPropertyNames(query).length == 0) {
            query.partner = context.params.uid;
        }

        let opps = await context.$axios.$get('/api/ui/finder/search', { params: query });

        return {
            opportunities: opps,
        };
    },
}
</script>

<style lang="scss" scoped>

</style>
