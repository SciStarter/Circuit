<template>
  <div>
  <div class="exchange-actions">

    <button  v-if="$store.state.user.authenticated" class="toggle-menu mobile-only" title="Toggle menu" :aria-pressed="String(menu)" data-context="header-menu" @click="toggle_mobile_nav = !toggle_mobile_nav">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data">
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <div v-if="partner !== null" class="exchange-nav" :class="{'show':toggle_mobile_nav}">
      <nuxt-link :to="{name: 'exchange-uid', params: {uid: $route.params.uid}}" class="home" title="home"><home-icon /><span class="home-text">Home</span></nuxt-link>
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

<opportunity-form
  v-if="authorized"
  v-model="opp"
  :partner="exchange"
  :timezones="timezones"
  :descriptors="descriptors"
  :topics="topics"
  in-exchange
  />

<div v-else>
  Sorry, but you are not authorized to add opportunities to this Science Near Me partner.
</div>
</div>
</div>
</template>

<script>
import HomeIcon from '~/assets/img/home.svg?inline'
export default {
    name: "ExchangeSubmit",
    components: {
      HomeIcon
    },

    props: {
        partner: {
            type: Object,
            required: false,
        },

        exchange: {
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
            return this.authenticated && (this.partner || this.exchange.open_submission);
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

  /*********** NAVIGATION *****/
  .exchange-actions {
    display:flex;
    justify-content:space-between;
    background-color: #efefef;
    padding:8px 20px;
    position:sticky;
    top:0;
    z-index:999;

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
