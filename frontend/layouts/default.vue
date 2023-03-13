<template>
<div id="page" :style="custom_props" :class="[$route.name,{'authenticated': authenticated, 'not-authenticated': !authenticated}]">
  <div class="beta-banner snm-wrapper">
    <div class="snm-container">
    <p><img src="~assets/img/atom.svg?data"><b>We're in beta!</b> If you find a bug or have feedback, you can email <a href="mailto:info@sciencenearme.org">info@sciencenearme.org</a>.</p>
  </div>
  </div>
  <header class="flex flex-align-center flex-justify-sb">
    <button class="toggle-menu mobile-only" title="Toggle menu" :aria-pressed="String(menu)" data-context="header-menu" @click="menu = !menu">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data">
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <nuxt-link to="/" class="logo" data-context="Science Near Me logo">
      <img src="~assets/img/logo-beta.svg?data" title="return to home page">
    </nuxt-link>

    <div class="flex">
        <nuxt-link :to="'/find' + search_query" class="toggle-search" data-context="header-search">
          <img src="~assets/img/search.svg?data"> <span class="no-mobile">Search Opportunities</span>
        </nuxt-link>

        <aside :class="{toggled: search}" class="search-box">
          <div class="search-box-container">
            <div class="full-only sbc-header flex flex-justify-sb">
              <h2>Search</h2>
              <button type="button" @click="toggle_search">
                &#10005;
              </button>
            </div>
          <b-field>
            <b-input ref="search_keywords" v-model="query.keywords" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
          </b-field>
          <lookup-place v-model="query.place" @input="store_here" />
          <div class="centered-row">
            <b-field>
              <b-checkbox v-model="query.include_online">
                Include Online Opportunities
              </b-checkbox>
            </b-field>
          </div>
          <div class="centered-row flex-justify-sb">
            <b-field label="From" label-position="on-border">
              <input v-model="query.date_from" class="control" type="date">
            </b-field>
            <b-field label="Until" label-position="on-border">
              <input v-model="query.date_until" class="control" type="date">
            </b-field>
          </div>
          <div class="centered-row">
            <action-button primary arrow @click="find">
              <search-icon class="button-icon" /> Search
            </action-button>
          </div>
        </div>
        </aside>

        <aside :class="{toggled: menu}" class="menu-box mobile-menu" @click="menu = !menu">
          <div v-if="authenticated" class="authenticated">
            <span class="no-mobile" data-context="header-username">{{ username }}</span>
            <ul>
              <li class="mobile-only">
                <nuxt-link to="/find">
                  <find-icon class="menu-icon" /> Find Opportunities
                </nuxt-link>
              </li>
              <li class="mobile-only">
                <nuxt-link to="/my/saved">
                  <saved-icon class="menu-icon" /> Saved Opportunities
                </nuxt-link>
              </li>
              <li class="mobile-only">
                <nuxt-link to="/my/science">
                  <science-icon class="menu-icon" /> My Activity Log<span v-if="user.reports_pending > 0" class="bubble">{{ user.reports_pending }}</span>
                </nuxt-link>
              </li>
              <li class="mobile-only">
                <nuxt-link to="/my/goals">
                  <goals-icon class="menu-icon" /> My Goals
                </nuxt-link>
              </li>
              <li class="mobile-only">
                <nuxt-link to="/my/profile">
                  <profile-icon class="menu-icon" /> My Profile &amp; Settings
                </nuxt-link>
              </li>
              <!-- <li class="mobile-only">
                <strong v-if="owner" class="nav-separate">Manage Opportunities</strong>
              </li> -->
              <li class="mobile-only">
                <nuxt-link v-if="owner" to="/my/opportunities">
                  <my-opportunities-icon class="menu-icon" /> Your Opportunities
                </nuxt-link>
              </li>
              <li class="mobile-only">
                <nuxt-link v-if="owner" to="/my/organization">
                  <my-organization-icon class="menu-icon" /> Your Partner Organization
                </nuxt-link>
              </li>

              <li class="mobile-only" v-if="beta_features && owner">
                <nuxt-link to="/my/data-overview">
                  <my-data-icon class="menu-icon" /> Data Insights
                </nuxt-link>

                <ul class="subnav">
                  <li><nuxt-link v-if="owner" to="/my/data-overview">Your Data Overview</nuxt-link></li>
                  <li><nuxt-link v-if="owner" to="/my/hosts-explorer">Hosts Explorer</nuxt-link></li>
                  <li><nuxt-link v-if="owner" to="/my/opportunity-data-explorer">Opportunity Data Explorer</nuxt-link></li>
                  <li><nuxt-link v-if="owner" to="/my/snm-data-overview"> SNM Data Overview</nuxt-link></li>
                </ul>

              </li>

             
              <li class="mobile-only">
                <nuxt-link v-if="owner" to="/my/submit-opportunity">
                  <submit-opportunity-icon class="menu-icon" /> Add an Opportunity
                </nuxt-link>
              </li>

              <li><span class="no-icon" /><a @click="logout">Log Out</a></li>
            </ul>
          </div>
          <div v-else class="not-authenticated">
            <nuxt-link class="action-button primary" :to="{name: 'login', query: {next: $route.fullPath}}">Login</nuxt-link>
            <nuxt-link class="action-button primary" :to="{name: 'signup', query: {next: $route.fullPath}}">Create Account</nuxt-link>
            <!-- <action-button primary @click="show_login = true">
              Login
            </action-button>
            <action-button primary @click="show_signup = true">
              Create Account
            </action-button> -->
          </div>
        </aside>
      </div>
      </header>

      <section id="main">
        <div id="content">
          <nuxt @login="show_login=true;show_signup=false;" @signup="show_signup=true;show_login=false;" />
          <aside v-if="show_cookie" id="cookie-notice">
            <div>
              <h1>We Care About Your Privacy</h1>
              <p>
                We and

                <nuxt-link to="/about#our-partners">
                  our partners
                </nuxt-link>

                store and/or access information on a device, such as
                unique IDs in cookies to process personal data. Find
                the details on our

                <nuxt-link to="/privacy">
                  privacy policy
                </nuxt-link>

                page.
              </p>
              <b-button type="is-primary" @click="cookie_consent">
                Accept All
              </b-button>
            </div>
            <div>
              <h2>We and our partners process data to provide:</h2>
              <p>
                Geolocation data, personalized content, audience
                insights, product development and

                <nuxt-link to="/research-participant">
                  research participant studies.
                </nuxt-link>
              </p>
            </div>
          </aside>
        </div>

        <div id="authenticated-nav">
          <nuxt-link to="/" class="logo" data-context="Science Near Me logo">
            <img src="~assets/img/logo-beta.svg?data" title="return to home page">
          </nuxt-link>
          <div class="an-overflow">
            <nav>
              <strong v-if="owner">My Participation</strong>

              <nuxt-link to="/find">
                <find-icon /> Find Opportunities
              </nuxt-link>

              <nuxt-link to="/my/saved">
                <saved-icon /> Saved Opportunities
              </nuxt-link>

              <nuxt-link to="/my/science">
                <science-icon /> My Activity Log<span v-if="user.reports_pending > 0" class="bubble">{{ user.reports_pending }}</span>
              </nuxt-link>

              <nuxt-link to="/my/goals">
                <goals-icon /> My Goals
              </nuxt-link>

              <nuxt-link to="/my/profile">
                <profile-icon /> My Profile &amp; Settings
              </nuxt-link>

              <strong v-if="owner" class="nav-separate">Manage Opportunities</strong>

              <nuxt-link v-if="owner" to="/my/opportunities">
                <my-opportunities-icon /> Your Opportunities
              </nuxt-link>

              <nuxt-link v-if="owner" to="/my/organization">
                <my-organization-icon /> Your Partner Organization
              </nuxt-link>

              <nuxt-link v-if="beta_features && owner" to="/my/data-overview" :class="{'nuxt-link-active': $route.name == 'my-hosts-explorer' ||  $route.name == 'my-opportunity-data-explorer' || $route.name == 'my-snm-data-overview'}">
                <my-data-icon class="menu-icon" /> Data Insights
              </nuxt-link>

                <ul v-if="beta_features && owner && ($route.name == 'my-data-overview' || $route.name == 'my-hosts-explorer' ||  $route.name == 'my-opportunity-data-explorer' || $route.name == 'my-snm-data-overview')" class="subnav">
                  <li><nuxt-link v-if="owner" to="/my/data-overview">Your Data Overview</nuxt-link></li>
                  <li><nuxt-link v-if="owner" to="/my/hosts-explorer">Hosts Explorer</nuxt-link></li>
                  <li><nuxt-link v-if="owner" to="/my/opportunity-data-explorer">Opportunity Data Explorer</nuxt-link></li>
                  <li><nuxt-link v-if="owner" to="/my/snm-data-overview"> SNM Data Overview</nuxt-link></li>
                </ul>


              <nuxt-link v-if="owner" to="/my/submit-opportunity">
                <submit-opportunity-icon /> Add an Opportunity
              </nuxt-link>
            </nav>
          </div>
        </div>
      </section>

      <Footer />
      <SubFooter />

      <b-modal v-model="show_login" :width="640" aria-role="dialog" aria-label="Log in" aria-modal>
        <div class="card">
          <login-form @close="show_login=false" @signup="show_login=false;show_signup=true;" :next="$route.path == 'slug' ? $route.params.slug : $route.path" :next_query="$route.query" in-modal>
            <dynamic-block group="login-modal" item="standard" class="content" />
          </login-form>
        </div>
      </b-modal>

      <b-modal v-model="show_signup" :width="640" aria-role="dialog" aria-label="Sign up" aria-modal>
        <div class="card">
          <signup-form @close="show_signup=false" @login="show_signup=false;show_login=true;" :next="$route.path == 'slug' ? $route.params.slug : $route.path" :next_query="$route.query" in-modal>
            <dynamic-block group="signup-modal" item="standard" class="content" />
          </signup-form>
        </div>
      </b-modal>

    </div>
</template>

<script>
// import Card from '~/components/Card'
import LoginForm from '~/components/LoginForm'
import SignupForm from '~/components/SignupForm'
import ExternalLink from '~/components/ExternalLink'
import DynamicBlock from '~/components/DynamicBlock'
import LookupPlace from '~/components/LookupPlace'
import ActionButton from '~/components/ActionButton'

import Footer from "~/components/Footer"
import SubFooter from "~/components/SubFooter"

import FindIcon from '~/assets/img/find-science-opportunities.svg?inline'
import SavedIcon from '~/assets/img/saved-science-opportunities.svg?inline'
import ScienceIcon from '~/assets/img/my-science.svg?inline'
import GoalsIcon from '~/assets/img/my-goals.svg?inline'
import ProfileIcon from '~/assets/img/my-profile-and-settings.svg?inline'
import SearchIcon from '~/assets/img/search.svg?inline'
import MyOpportunitiesIcon from '~/assets/img/current-opportunities.svg?inline'
import MyPastOpportunitiesIcon from '~/assets/img/past-opportunities.svg?inline'
import MyOrganizationIcon from '~/assets/img/your-organization.svg?inline'
import SubmitOpportunityIcon from '~/assets/img/submit-opportunity.svg?inline'
import MyDataIcon from '~/assets/img/data-insights.svg?inline'

export default {
    components: {
        // Card,
        LoginForm,
        SignupForm,
        ExternalLink,
        DynamicBlock,
        LookupPlace,
        ActionButton,

        FindIcon,
        SavedIcon,
        ScienceIcon,
        GoalsIcon,
        ProfileIcon,
        SearchIcon,
        MyOpportunitiesIcon,
        MyPastOpportunitiesIcon,
        MyOrganizationIcon,
        SubmitOpportunityIcon,

        Footer,
        SubFooter,
        MyDataIcon
    },

    data () {
        const now = new Date();
        return {
            menu: false,
            search: false,
            show_login: false,
            show_signup: false,
            show_person_dropdown: false,
            show_cookie: false,
            show_location_modal: false,

            query: {
                keywords: '',
                place: {
                    near: '',
                    longitude: 0,
                    latitude: 0,
                    proximity: 0
                },
                include_online: true,
                date_from: now.toISOString().split('T')[0],
                date_until: null,
                sort: 'closest'
            }
        }
    },

    async fetch () {
        await this.$store.dispatch('get_user')
    },

    computed: {
        custom_props() {
            return {
                '--background-color': null,
                '--primary-color': null,
                '--secondary-color': null,
                '--tertiary-color': null,
                '--logo-url': null,
            };
        },

        beta_features() {
            if(process.client) {
                return window.location.host.indexOf("beta.") >= 0;
            }
            else {
                return false;
            }
        },

        user() {
            return this.$store.state.user;
        },

        alert() {
            if(!this.user || !this.user.authenticated) {
                return false;
            }

            return this.user.reports_pending > 0;
        },

        search_query() {
            let joint = '?';
            let ret = '';

            if (this.query.keywords) {
                ret += joint + 'text=' + encodeURIComponent(this.query.keywords);
                joint = '&';
            }

            if (this.query.place.near) {
                ret += joint + 'near=' + encodeURIComponent(this.query.place.near);
                joint = '&';
            }

            if (this.query.place.longitude) {
                ret += joint + 'longitude=' + encodeURIComponent(this.query.place.longitude);
                joint = '&';
            }

            if (this.query.place.latitude) {
                ret += joint + 'latitude=' + encodeURIComponent(this.query.place.latitude);
                joint = '&';
            }

            if (this.query.place.proximity) {
                ret += joint + 'proximity=' + encodeURIComponent(this.query.place.proximity);
                joint = '&';
            }

            if (!this.query.include_online) {
                ret += joint + 'physical=in-person';
                joint = '&';
            }

            if (this.query.date_from !== null) {
                let date = this.query.date_from;
                if(date.constructor !== Date) {
                    date = new Date(date);
                }
                ret += joint + 'beginning=' + encodeURIComponent(date.toISOString());
                joint = '&';
            }

            if (this.query.date_until !== null) {
                let date = this.query.date_until;
                if(date.constructor !== Date) {
                    date = new Date(date);
                }
                ret += joint + 'ending=' + encodeURIComponent(date.toISOString());
                joint = '&';
            }

            if (this.query.sort !== null) {
                ret += joint + 'sort=' + this.query.sort;
                joint = '&';
            }

            return ret
        },

        authenticated() {
            return !!this.$store.state.user.authenticated;
        },

        owner() {
            if(!this.authenticated) {
                return false;
            }

            return this.$store.state.user.num_partners > 0;
        },

        username() {
            return this.$store.state.user.username;
        },

        show_location_cue() {
            return !(this.query.place.latitude || this.query.place.longitude);
        },

    },

    async mounted() {
        await this.$store.dispatch('sync_local_to_server');
        if(!window.localStorage.getItem("cookie-consent")) {
            this.show_cookie = true;
        }
    },

    methods: {
        cookie_consent() {
            this.show_cookie = false;
            window.localStorage.setItem("cookie-consent", "true");
        },

        toggle_search() {
            this.search = !this.search;

            if(this.search) {
                this.$refs.search_keywords.focus();
            }
        },

        find() {
            this.search = false;
            this.$router.push('/find' + this.search_query);
        },

        logout() {
            this.$store.dispatch('logout');
        },

        store_here(place) {
            this.$store.commit('here', place);
        },
    }
}
</script>

<style lang="scss" scoped>

/* Increase if wider menu items are added */
$user-menu-width: 10rem;

/* Increase if more menu items are added */
$user-menu-height: 2rem;

* {
  box-sizing: border-box;
}

#page {
    width: 100vw;
    display:flex;
    flex-direction:column;
    min-height:100vh;
}
#main {
  flex:1;
}

.full-only {
  display: none;
}

header {
    height: 52px;
    background-color: $snm-color-background-medlight;
    border-top: 2px solid $snm-color-background-dark;
    padding: 0 1rem;
    position: fixed;
    top:0;
    left:0;
    right:0;
    z-index: 100;

    .centered-row {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        margin-bottom: 0.75rem;
    }

    .button-icon {
        @include svg-fill;
        width: auto;
        height: 1.2em;
        vertical-align: middle;
    }

    .menu-icon {
        @include svg-fill;

        color: $snm-color-background-medium;
        width: auto;
        height: 1.2em;
        vertical-align: middle;
        margin: 0px 1rem;
        path {
          @include svg-fill;
        }
    }

    .no-icon {
        margin-left: 1rem;
    }

    .logo {
        height: 39px;
        width: auto;
        margin: 6px 0;

        img {
            width: 100%;
            height: 100%;
        }
    }

    .toggle-menu {
        background-color: transparent;
        border: 0px;
        width: rem(24px);
        height: rem(24px);
        padding:0;
    }

    .toggle-search {
        background-color: $snm-color-action;
        border: 1px solid $snm-color-action-border;
        border-radius: 5px;
        padding: 5px;
        height: 24px;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        position: relative;
        top: -1px;
        white-space: nowrap;
        color: var(--primary-color, $snm-color-element-dark);
        display: flex;
        align-items: center;
        flex-shrink: 0;
        font-weight:bold;

        img {
            display: block;
            width: 14px;
            height: 14px;
            // margin-left: 6px;
        }
    }

    .menu-box {
        display: none;
        position: absolute;
        top:50px;
        left: 0px;
        right: 0px;
        z-index: 20;
        background-color: var(--primary-color, $snm-color-element-dark);
        text-align: left;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        width: 100%;
        

        

        .not-authenticated {
            display: flex;
            justify-content: space-evenly;
            background-color: var(--secondary-color, $snm-color-element-med);
            padding: .75rem 0;

            button, a {
              position: relative;
              font-family: $snm-font-content;
              font-size: 0.85rem;
              border: 1px solid var(--border);
              border-radius: 6px;
              margin: 10px 0.5rem;
              padding: 10px;
              box-shadow: 0px 3px 6px $snm-color-shadow;
              font-weight: bold;
              color: var(--foreground);
              background-color: var(--background);
              flex-shrink: 0;
              flex-grow: 0;
              cursor: pointer;
              display: inline-flex;
              align-items: center;
              justify-content: center;
              padding: 0.75rem 1.5rem;
              box-sizing: border-box;
              height: rem(40px);
              line-height: 1;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                width: calc(50% - 3rem);
                box-shadow: 0px 3px 6px $snm-color-shadow;
                margin:0;
                background-color: $snm-color-background-meddark;
                border: 1px solid var(--background-color, #fff);
                text-align: center;

                &:hover,&:active {
                    color: var(--secondary-color, $snm-color-element-med);
                    background-color: $snm-color-element-light;
                }

                &.primary {
                    --border: #{$snm-color-background-dark};
                    --background: #{$snm-color-background-meddark};
                    --foreground: #{$snm-color-element-light};

                    &:hover,&:active {
                        --background: #{$snm-color-background-dark};
                    }

                }
            }
        }

        .authenticated {
            li {
                // border: 1px solid $snm-color-border-ondark;
                line-height: 48px;

                a {
                    color: $snm-color-element-ondark;

                    img {
                        color: $snm-color-background-medium;
                    }
                }
            }

            .subnav li {
              padding-left: 56px;
              background-color: #5694a2;

            }
        }
    }

    .search-box {
        display: none;
        position: absolute;
        top: 52px;
        left: 0px;
        right: 0px;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        background-color: $snm-color-background;
        min-height: 4rem;
        z-index: 99;
        box-sizing: border-box;
        padding: 1rem;

        input[type="text"] {
          height: auto;
          padding-top: 0.5rem;
          padding-bottom: 0.5rem;
        }

        input[type="date"] {
            padding: 0.75rem;
            border-radius: 10px;
            border: 1px solid $snm-color-border;

        }

        .centered-row.flex-justify-sb {
          justify-content: space-between;

          .field {
            width: calc(50% - 0.5rem);
            input {
              width: 100%;
            }
          }
        }





    }

    .toggled {
        display: block;
        // max-width: 25rem;
        margin: 0px auto;
    }
}

#cookie-notice {
    position: fixed;
    bottom: 0;
    left: 0;
    z-index: 500;
    width: 100vw;
    min-width: 300px;
    max-height: 55vh;
    overflow-y: auto;
    overflow-x: hidden;
    background-color: $snm-color-background;
    border-radius: 0;
    border: 1px solid $snm-color-background-medium;
    padding: 1rem;
    box-shadow: 0px 0px 6px $snm-color-shadow;
    display: flex;
    flex-direction: column;

    h1 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 1.25rem;
        margin:0;
    }

    p {
      margin:0 0 0.5rem 0;
    }

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: 1rem;
    }
    button {
      margin-bottom: 1rem;
    }

    >div {
        display: flex;
        flex-direction: column;
        align-items: flex-start;

        // >* {
        //     margin: 1rem;
        // }
    }
}

#main {
  padding-top: 52px;
}

.mobile-only .nav-separate {
    color: #7cb4bf;
    padding-left: 3px;
}

footer {
    background-color: $snm-color-background-dark;

    li {
        line-height: 1.2;
        padding-left: 1rem;
        border-bottom: 1px solid $snm-color-border-ondark;
        padding: 0.5rem 0;

        h1 {
            color: $snm-color-heading-ondark;
            font-family: $snm-font-heading;
            font-size: $snm-font-smaller;
            font-weight: bold;
            letter-spacing: 0.7px;
        }

        a {
            color: $snm-color-element-ondark;
            font-family: $snm-font-content;
            font-size: $snm-font-small;
            letter-spacing: 0px;
        }
    }

    .partner {
        display: flex;
        flex-direction: row;


        .nsf-logo {
            margin-right: 1rem;
            flex-shrink: 0;

            img {
                height: auto;
                width: 56px;
                vertical-align: top;
            }
        }

        .description {
            color: $snm-color-element-ondark;
            font-family: $snm-font-content;
            font-size: $snm-font-smaller;
            line-height: 22px;
            letter-spacing: 0px;
            margin: 16px 16px 16px 0px;
        }
    }
}

.subfooter {
  background-color: #1d3a40;

  li a {
    color: #F2C04B;
  }
}

#main {
    >nav,>.logo {
        display: none;
    }
}

@media only screen and (min-width: $fullsize-screen) {
    .full-only {
      display: block;
    }

    #main {
      padding-top: 0;
    }

    #cookie-notice {
      position: fixed;
      bottom: 1rem;
      left: 1rem;
      z-index: 500;
      width: 75vw;
      min-width: 300px;
      max-height: 50vh;
      overflow-y: auto;
      overflow-x: hidden;
      background-color: $snm-color-background;
      border-radius: 1rem;
      border: 1px solid $snm-color-background-medium;
      padding: 2rem;
      box-shadow: 0px 0px 6px $snm-color-shadow;
        flex-direction: row;
    }

  header .logo {
        width: 180px;
        height: auto;
        margin: 1rem;
        margin-top: 2rem;
        margin-bottom: -1.2rem;
    }

    header {
        height: 90px;
        border-top: 10px solid $snm-color-prehead;
        position: relative;
        overflow:visible;

        header a {
          display: block;
        }

        .toggle-menu {
          display: none;
        }



        .toggle-search {
            position: relative;
            // top: 30px;
            // right: 10px;
            order: 3;
            padding: 10px;
            min-width: 40px;
            height: 40px;
            margin-right: 1rem;

            &:hover,&:active {
              background-color: #F2C04B;
            }

            img {
                width: 20px;
                height: 20px;
                margin-right: 6px;
            }
        }

        .menu-box {
            display: inline-block;
            background-color: transparent;
            box-shadow: none;
            position:relative;
            left: unset;
            right: unset;
            top: unset;
            padding:0;

            .not-authenticated {
                display: inline-block;
                background-color: transparent;
                padding: 0;

                button,a {
                    width: auto;
                    box-shadow: 0px 3px 6px $snm-color-shadow;
                    background-color: $snm-color-background-meddark;
                    border: $snm-color-background-dark;
                    margin-right: 1rem;

                    &:hover,&:active {
                        color: var(--secondary-color, $snm-color-element-med);
                        background-color: $snm-color-element-light;
                    }
                }
            }

            .authenticated {
                display: flex;
                flex-wrap: wrap;
                align-items: center;
                justify-content: right;
                margin-right: 1rem;
                height: 40px;
                border-radius: 6px;
                background-color: $snm-color-background-meddark;
                color: $snm-color-element-light;
                box-sizing: border-box;
                border: 1px solid $snm-color-background-dark;
                transition: border-bottom-right-radius 0.5s, botder-bottom-left-radius 0.5s;
                padding: 0px 1rem;
                min-width: $user-menu-width;
                position: relative;

                > span {
                    font-family: $snm-font-content;
                    font-weight: bold;
                    font-size: $snm-font-small;
                    cursor: pointer;

                    &::after {
                        content: url(~assets/img/down-arrow.svg);
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        margin-left: 1rem;
                        width: 1.5em;
                        height: 1.5em;
                        border-radius: 1em;
                        color: var(--primary-color, $snm-color-element-dark);
                        background-color: $snm-color-element-light;
                        transform: rotate(0);
                        transition: transform;
                    }
                }

                > ul {
                    position: absolute;
                    top: 40px;
                    right: 0;
                    width: $user-menu-width;
                    overflow: hidden;
                    max-height: 0px;
                    max-width: 0px;
                    transition: max-height 0.5s, max-width 0.5s, right 0.5s;
                    background-color: $snm-color-background-meddark;
                    box-sizing: border-box;
                    border: 1px solid $snm-color-background-dark;
                    border-radius: 6px;
                    border-top-left-radius: 0px;
                    border-top-right-radius: 0px;
                    opacity: 0;

                    li {
                        border: 0px;
                        line-height: 24px;
                        display: flex;
                        align-items: center;
                        justify-content: right;
                        margin: 3px 1rem;

                        a {
                            color: $snm-color-element-ondark;
                        }
                    }
                }
            }

            &.toggled .authenticated {
                border-bottom-left-radius: 0px;
                border-bottom-right-radius: 0px;
                border-bottom: 0px;

                > span {
                    &::after {
                        transform: rotate(-0.5turn);
                        transition: transform;
                    }
                }

                ul {
                    max-width: $user-menu-width;
                    max-height: $user-menu-height;
                    top: 40px;
                    right: -1px;
                    opacity: 1;
                    transition: max-height 0.5s, max-width 0.5s, right 0.5s, opacity 0.5s;
                }
            }
        }
    }

    .authenticated #main {


        nav {
            // margin-top: 20px;
            display: flex;
            flex-direction: column;

            // clip-path: inset(0px -2rem 0px 0px);

            strong {
                font-family: $snm-font-content;
                font-weight: bold;
                font-size: $snm-font-smallest;
                line-height: 15px;
                letter-spacing: 0.39px;
                text-transform: uppercase;
                padding: 2rem 0px 0px 1rem;

                &.nav-separate {
                    margin-top: 2rem;
                    border-top: 1px solid var(--background-color, $snm-color-background-light);
                }
            }

            a {
                display: flex;
                color: var(--primary-color, $snm-color-element-dark);
                background-color: transparent;
                align-items: center;
                font-family: $snm-font-content;
                font-weight: normal;
                font-size: $snm-font-small;
                line-height: 0.9rem;
                padding-left: 1rem;
                position: relative;
                box-sizing: border-box;
                height: 48px;
                margin-top: 0;
                padding-right: 1rem;

                &.nuxt-link-active, &.nuxt-link-active:active {
                    background-color: $snm-color-background-meddark;
                    color: $snm-color-element-light!important;

                    // &::after {
                    //     position: absolute;
                    //     right: -26px;
                    //     content: "";
                    //     width: 0px;
                    //     height: 52px;
                    //     border-left: 26px solid $snm-color-background-meddark;
                    //     border-top: 26px solid transparent;
                    //     border-bottom: 26px solid transparent;
                    // }
                }

                svg {
                    margin-right: 1rem;
                    min-width: 20px;

                    * {
                        fill: currentColor;
                    }
                }
            }
            a:hover,a:active {
              color: $snm-color-background-meddark;
            }

            .subnav a {
              padding-left: 56px;
              &.nuxt-link-active {
                background-color: #5694a2;
              }
            }
            .subnav li:last-child {
                border-bottom: 1px solid #5694a2;
              }
        }
    }

    footer > .snm-container {
        display: flex;
        justify-content: space-between;
        margin: 0 auto;

        > ul {
          width: calc(25% - 0.5rem);

            li {
                border: 0px;

                h1 {
                    text-decoration: underline;
                }
            }
        }

        .partner {
          width: calc(50% - 0.5rem);
        }


    }


}

.not-authenticated #authenticated-nav, .authenticated #authenticated-nav {
  display: none;
}

@media (min-width: $fullsize-screen) {
  .authenticated {
    header {
      justify-content: flex-end!important;
      height: 80px;
      position: fixed;
    }

    #content {
      padding-top:80px;
    }

    header .logo {
      display: none!important;
    }

    .logo {
      width: 180px;
      margin: 0 auto;
      display: block;
      margin-top: 1rem;
      img {
        width: 180px;
      }
    }

  }

  .authenticated #authenticated-nav {
    display: block;
    position: fixed;
    top: 0;
    left: 0;
    bottom: 0;
    width: 200px;
    background-color: $snm-color-background-medium;
    box-shadow: 0px 0px 6px $snm-color-shadow;
    z-index:101;

    .an-overflow {
      overflow-x: visible;
      height: calc(100vh - 105px);
      scrollbar-width: none;
      overflow-y: auto;
      &::-webkit-scrollbar {
        display: none; /* for Chrome, Safari, and Opera */
      }
    }

    nav {
      margin-bottom: 2rem;
    }

  }



  .authenticated #content {
    margin-left: 200px;
  }

  .authenticated footer {
    padding-left: 200px;
    .snm-container {
      padding: 1rem;
    }
  }
  .authenticated .subfooter {
    padding-left: 200px;
  }

  .not-authenticated  .just-content {
    padding-top: 2rem;
  }

  .not-authenticated footer {
    padding-top:2rem;
  }

  .not-authenticated footer.homepage {
    padding-top:5rem;
  }

  .subfooter ul {
    display: flex;

    li {
      margin-right: 1rem;
    }
    li:not(:first-child)::before {
      content: '|';
      color: #087a91;
      margin-right: 8px;
    }
  }
  .authenticated .subfooter ul {
    padding-left: 2rem;
  }

}

@media (min-width: 1200px) {
  .authenticated #authenticated-nav {
    width: 280px;
  }
  .authenticated #content {
    margin-left: 280px;
  }

  .authenticated footer, .authenticated .subfooter {
    padding-left: 280px;
  }
}


.modal .card {
    margin: 1rem;
    padding: 1rem;
    max-height: 100%;
    overflow: auto;
}

@media (max-width: $mobile-screen) {
    .no-mobile {
        display: none !important;
    }
}

@media (max-width: 959px) {
  .mobile-menu {
          overflow: auto!important;;
        max-height: calc(100vh - 52px)!important;
        }
}

@media (min-width: $tablet-screen) {
  .no-tablet {
    display: none;
  }
}

@media (min-width: $fullsize-screen) {
    .mobile-only {
        display: none !important;
    }
    .no-tablet {
      display: block;
    }
}

.bubble {
    position: relative;
    top: -0.25em;
    display: inline-flex;
    font-size: 8pt;
    color: $snm-color-element-light;
    background-color: $snm-color-info;
    border-radius: 50%;
    align-items: center;
    justify-content: center;
    min-width: 1rem;
    height: 1rem;
    line-height: 1rem;
    margin-left: 0.5em;
}

@media (min-width:$fullsize-screen) {
  header .search-box.toggled {
    background-color: rgba(0,0,0,0.8);
    position: fixed;
    top:0;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .search-box-container {
      background-color: var(--background-color, #fff);
      padding: 1rem;
      border-radius: 6px;
    }

    .sbc-header {
      display: flex;
      color: $snm-color-background-meddark;

      h2 {
        font-size: 1.4rem;
        font-weight: bold;
      }
      button {
        border:0;
        background-color: transparent;
        font-size: 1.8rem;
        font-weight: bold;
      }
    }

  }

}

.beta-banner {
  text-align: center;
  padding: 0.5rem!important;
  border-bottom: 1px solid #efefef;
  z-index: 100;
  img {
    height: 20px;
    vertical-align:middle;
    position:relative;
    top: -2px;
    left: -2px;
  }
}
.authenticated .beta-banner, .not-authenticated .beta-banner {
  margin-top: 52px;
  margin-bottom: -52px;
}

@media (min-width:$fullsize-screen) {
  .not-authenticated .beta-banner {
    margin-top: 0;
    margin-bottom: 0;
  }
  .authenticated .beta-banner {
    margin-top: 80px;
    padding-left: 200px!important;
    margin-bottom: -80px;
  }
}



@media (min-width:1200px) {
  .authenticated .beta-banner {
    padding-left: 280px!important;
  }
}

.my-submit-opportunity, .my-edit-opportunity{
  footer, .subfooter {
    display:none;
  }
}

</style>

<style type="scss">
  .b-tabs .tab-content {
    padding:0;
  }
  .modal {
    z-index:999!important;

  }
  .modal .animation-content {
    overflow: visible;
  }
  @media (min-width:768px){
    .modal .animation-content {
      min-width: 600px;
    }
  }
</style>
