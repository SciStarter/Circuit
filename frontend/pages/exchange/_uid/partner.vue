<template>
<div v-if="authorized">

  <div class="exchange-actions">

    <button  v-if="$store.state.user.authenticated" class="toggle-menu mobile-only" title="Toggle menu" :aria-pressed="String(menu)" data-context="header-menu" @click="toggle_mobile_nav = !toggle_mobile_nav">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data">
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <div v-if="partner !== null" class="exchange-nav" :class="{'show':toggle_mobile_nav}">
      <nuxt-link :to="{name: 'exchange-uid-submit', params: {uid: partner.uid}}" class="button"><submit-opportunity-icon/> Add an Opportunity</nuxt-link>
      <nuxt-link :to="{name: 'exchange-uid-partner', params: {uid: partner.uid}}">Manage Organization</nuxt-link>
      <nuxt-link :to="{name: 'exchange-uid-opps', params: {uid: partner.uid}}">Manage Opportunities</nuxt-link>
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
<div class="exchange-wrapper">
<div class="snm-container">
  <h1>Your ScienceNearMe Partner Organization</h1>
</div>

<partner-form
  :partner="partner"
  :org_types="org_types"
  :managers="managers"
  :pending="pending"/>
</div>
</div>
<div v-else class="snm-container">
  <nuxt-link :to="{name: 'exchange-uid', params: {uid: $route.params.uid}}">Home</nuxt-link>
  Sorry, you're not authorized to edit this partner's data.
</div>
</template>

<script>
import SubmitOpportunityIcon from '~/assets/img/submit-opportunity.svg?inline'
export default {
    name: "ExchangePartner",
    components: {
      SubmitOpportunityIcon
    },
    props: {
        partner: {
            type: Object,
            required: true,
        },
    },
    data() {
        return {
            toggle_mobile_nav: false
        };
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
    color: var(--secondary-color, $snm-color-element-med);
    margin-bottom:2rem;
    margin-top:1rem;
}
@media (max-width:600px) {
  h1 {
    font-size:21px;
    margin-left:16px;
    margin-right:16px;
  }
}
@media (min-width:601px) AND (max-width:1310px){
  .exchange-wrapper {
    padding-left:1rem;
    padding-right:1rem;
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

.exchange-search {
  display:flex;
  flex-direction:column;
  align-items:center;
  margin:20px 0;
  padding:16px;
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
