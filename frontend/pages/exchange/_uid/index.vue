<template>
<div class="exchange exchange-index">

  <div class="exchange-actions">

    <button  v-if="$store.state.user.authenticated" class="toggle-menu mobile-only" title="Toggle menu" :aria-pressed="String(menu)" data-context="header-menu" @click="toggle_mobile_nav = !toggle_mobile_nav">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data">
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <div v-if="partner !== null" class="exchange-nav" :class="{'show':toggle_mobile_nav}">
      <nuxt-link :to="{name: 'exchange-uid', params: {uid: $route.params.uid}}" class="home" title="home"><home-icon /><span class="home-text">Home</span></nuxt-link>
      <nuxt-link :to="{name: 'exchange-uid-partner', params: {uid: partner.uid}}">Manage Organization</nuxt-link>
      <nuxt-link :to="{name: 'exchange-uid-opps', params: {uid: partner.uid}}">Manage Opportunities</nuxt-link>
      <nuxt-link :to="{name: 'exchange-uid-submit', params: {uid: partner.uid}}" class="button"><submit-opportunity-icon/> Add an Opportunity</nuxt-link>
    </div>

    <div class="exchange-logins">
      <div v-if="$store.state.user.authenticated">
        <a @click="$store.dispatch('logout')">Logout</a>
      </div>
      <div v-else class="e">
        <nuxt-link :to="{name: 'exchange-uid-login', params: {uid: $route.params.uid}, query: {next: $route.path}}">Login</nuxt-link> |
        <nuxt-link :to="{name: 'exchange-uid-signup', params: {uid: $route.params.uid}, query: {next: $route.path}}">Signup</nuxt-link>
      </div>
    </div>

  </div><!-- .exchange-actions -->


  <!-- <div class="partner-logo"></div> -->


<div class="exchange-wrapper">
  <div class="exchange-search">
    <div class="ex-search">
      <b-input v-model="search_text"/>
      <b-button @click="search({text: search_text, page: 0})">Search</b-button>
    </div>
    <div class="search-snm">
      <b-checkbox @input="search({all: $event, page: 0})">Search all of of the Science Near Me network</b-checkbox>
    </div>
  </div>

  <div class="exchange-results">
    <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :partner="partner" previous-page="find" />

    <Pagination
      v-if="opportunities.matches.length > 0"
      :page-index="opportunities.pagination.page_index"
      :last-page="opportunities.pagination.last_page"
      @switch="search({page: $event})" />

  </div>
</div><!-- .exchange-wrapper -->
</div>
</template>

<script>
import HomeIcon from '~/assets/img/home.svg?inline'
import SubmitOpportunityIcon from '~/assets/img/submit-opportunity.svg?inline'
export default {
    name: "ExchangeIndex",
    components: {
      SubmitOpportunityIcon,
      HomeIcon
    },
    props: {
        partner: {
            type: [Object, null],
            required: true,
        },
    },

    data() {
        return {
            search_text: this.$route.query.text || '',
            toggle_mobile_nav: false
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
.exchange-search {
  display:flex;
  flex-direction:column;
  align-items:center;
  margin:20px 0;
  padding:16px;
}
.ex-search {
  display:flex;
  justify-content:center;
  width:100%;
  .control {
    max-width:800px;
    flex-grow:1;
    margin-right:16px;
  }
}
.search-snm {
  margin-top:10px;
  label {
    color: #999;
  }
  .b-checkbox.checkbox input[type=checkbox] + .check {
    border:1px solid #999;
  }
}

.exchange-results {
  display:flex;
  flex-direction:column;
  justify-content:center;
  align-items:center;
  > article {
    width:100%!important;
    max-width: 900px;
  }
}

/*********** NAVIGATION *****/
.exchange-actions {
  display:flex;
  justify-content:space-between;
  background-color: #efefef;
  padding:8px 20px;

  .button {
    color: #087a91;
    svg {
      vertical-align: middle;
      position: relative;
      top: -2px;
      margin-right:10px;
      path {
        fill: #087a91;
      }
    }
  }
  a:not(.button):hover {
    text-decoration:underline;
  }
  .home {
    width:20px;
    svg {
      width:20px;
      height:20px;
      path {
        fill: #087a91!important;
      }
    }
  }
}


.exchange-logins {
  margin-left: auto;
  display: flex;
  align-items: center;
}
.exchange-nav {
  display: flex;
  align-items: center;
}
.exchange-nav a {
  margin-right:10px;
  margin-left:10px;
  &:first-child {
    margin-left:0;
  }
}

@media (min-width:701px){
  .toggle-menu,.home-text {
    display:none!important;
  }
}
@media (max-width:700px){
  .exchange-nav {
    flex-direction:column;
    position:absolute;
    top:47px;
    left:0;
    width:100%;
    z-index:100;
    background-color:#efefef;
    display:none;
    a {
      width:100%;
      margin:0;

      &:not(.button){
        padding:16px;
        border-top:1px solid #fff;
      }
      &.button {
        width: calc(100% - 32px);
        margin: 10px auto;
      }
    }
    .home {
      width:100%;
      svg {
        display:none;
      }
    }
  }

  .exchange-nav {
    align-items: flex-start;
    &.show {
      display:flex;
    }
  }
  .toggle-menu {
    border:0;
  }
}

</style>
