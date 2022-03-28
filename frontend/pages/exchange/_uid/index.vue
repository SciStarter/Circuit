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

  <div class="partner-logo"></div>
  <b-input v-model="search_text"/>
  <b-button @click="search({text: search_text, page: 0})">Search</b-button>
  <b-checkbox @input="search({all: $event, page: 0})">Search all of of the Science Near Me network</b-checkbox>
  <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :partner="partner" previous-page="find" />
  <Pagination
    :page-index="opportunities.pagination.page_index"
    :last-page="opportunities.pagination.last_page"
    @switch="search({page: $event})" />
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

    data() {
        return {
            search_text: this.$route.query.text || '',
        };
    },

    async asyncData(context) {
        let query = {...context.query};

        if(!query.all) {
            query.partner = context.params.uid;
        }

        if(query.page === undefined) {
            query.page = 0;
        }

        let opps = await context.$axios.$get('/api/ui/finder/search', { params: query });

        return {
            opportunities: opps,
        };
    },

    methods: {
        search(assign) {
            this.$router.push({name: 'exchange-uid', params: this.$route.params, query: {...this.$route.query, ...assign}});
        }
    },

    watchQuery: true,
}
</script>

<style lang="scss" scoped>
.partner-logo {
    width: 300px;
    height: 200px;
    background: var(--logo-url);
    background-size: contain;
    background-repeat: no-repeat;
}
</style>
