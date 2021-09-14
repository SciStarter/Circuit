<template>
<div id="page" :class="{'authenticated': authenticated, 'not-authenticated': !authenticated}">
  <header>
    <button class="toggle-menu mobile-only" title="Toggle menu" :aria-pressed="String(menu)" data-context="header-menu" @click="menu = !menu">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data">
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <nuxt-link to="/" class="logo" title="Science Near Me logo">
      <img src="~assets/img/logo.svg?data">
    </nuxt-link>

    <button class="toggle-search" title="Toggle search box" :aria-pressed="String(search)" data-context="header-search" @click="toggle_search">
      <img src="~assets/img/search.svg?data">
    </button>

    <aside :class="{toggled: search}" class="search-box">
      <b-field>
        <b-input ref="search_keywords" v-model="query.keywords" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
      </b-field>
      <lookup-place v-model="query.place" />
      <div class="centered-row">
        <b-field>
          <b-checkbox v-model="query.include_online">
            Include Online Opportunities
          </b-checkbox>
        </b-field>
      </div>
      <div class="centered-row">
        <b-field label="From" label-position="on-border">
          <input v-model="query.date_from" class="control" type="date">
        </b-field>
        <b-field label="Until" label-position="on-border">
          <input v-model="query.date_until" class="control" type="date">
        </b-field>
      </div>
      <arrow-button @click="find">
        <search-icon class="button-icon" /> Search
      </arrow-button>
    </aside>

    <aside :class="{toggled: menu}" class="menu-box" @click="menu = !menu">
      <div v-if="authenticated" class="authenticated">
        <span class="no-mobile" data-context="header-username">{{ username }}</span>
        <ul>
          <li class="mobile-only">
            <nuxt-link to="/find">
              <find-icon class="menu-icon" /> Find Science Opportunities
            </nuxt-link>
          </li>
          <li class="mobile-only">
            <nuxt-link to="/my/saved">
              <saved-icon class="menu-icon" /> Saved Science Opportunities
            </nuxt-link>
          </li>
          <li class="mobile-only">
            <nuxt-link to="/my/science">
              <science-icon class="menu-icon" /> My Science
            </nuxt-link>
          </li>
          <li class="mobile-only">
            <nuxt-link to="/my/goals">
              <goals-icon class="menu-icon" /> My Goals
            </nuxt-link>
          </li>
          <li class="mobile-only">
            <nuxt-link to="/my/profile">
              <profile-icon class="menu-icon" /> My Profile
            </nuxt-link>
          </li>
          <li><span class="no-icon" /><a @click="logout">Log Out</a></li>
        </ul>
      </div>
      <div v-else class="not-authenticated">
        <action-button primary contrast-bg @click="show_login = true">
          Login
        </action-button>
        <action-button primary contrast-bg @click="show_signup = true">
          Create Account
        </action-button>
      </div>
    </aside>
  </header>

  <section id="main">
    <div id="content">
      <nuxt @login="show_login = true" @signup="show_signup = true" />
    </div>

    <nav>
      <strong v-if="owner">My Participation</strong>

      <nuxt-link to="/find">
        <find-icon /> Find Science Opportunities
      </nuxt-link>

      <nuxt-link to="/my/saved">
        <saved-icon /> Saved Science Opportunities
      </nuxt-link>

      <nuxt-link to="/my/science">
        <science-icon /> My Science
      </nuxt-link>

      <nuxt-link to="/my/goals">
        <goals-icon /> My Goals
      </nuxt-link>

      <nuxt-link to="/my/profile">
        <profile-icon /> My Profile &amp; Settings
      </nuxt-link>

      <strong v-if="owner" class="nav-separate">Manage Opportunities</strong>

      <nuxt-link v-if="owner" to="/my/opportunities">
        <my-opportunities-icon /> Current Opportunities
      </nuxt-link>

      <nuxt-link v-if="owner" to="/my/draft-or-closed">
        <my-past-opportunities-icon /> Draft &amp; Closed Opportunities
      </nuxt-link>

      <nuxt-link v-if="owner" to="/my/organization">
        <my-organization-icon /> Your Organization
      </nuxt-link>

      <nuxt-link v-if="owner" to="/my/submit-opportunity">
        <submit-opportunity-icon /> Submit an Opportunity
      </nuxt-link>
    </nav>

    <nuxt-link to="/" class="logo" title="Science Near Me logo">
      <img src="~assets/img/logo.svg?data">
    </nuxt-link>
  </section>

  <footer>
    <ul>
      <li><h1>For Everyone</h1></li>
      <li>
        <nuxt-link to="/about">
          About Us
        </nuxt-link>
      </li>
      <li>
        <nuxt-link to="/terms">
          Terms of Service
        </nuxt-link>
      </li>
      <li>
        <nuxt-link to="/privacy">
          Privacy Policy
        </nuxt-link>
      </li>
      <li>
        <nuxt-link to="/cookies">
          Cookies Policy
        </nuxt-link>
      </li>
      <li>
        <nuxt-link to="/contact">
          Contact Us
        </nuxt-link>
      </li>
    </ul>

    <ul>
      <li><h1>For Science Professionals</h1></li>
      <li>
        <nuxt-link to="/affiliate">
          Be Part of Science Near Me
        </nuxt-link>
      </li>
      <li>
        <external href="/api/docs/v1.html" content="footer-link">
          API documentation
        </external>
      </li>
      <li>
        <nuxt-link to="/contact">
          Display Science Opportunities
        </nuxt-link>
      </li>
    </ul>

    <div class="partner">
      <div class="logo">
        <img src="~assets/img/NSF-small.png">
      </div>
      <div class="description">
        This project is based upon work supported, in part, by the
        National Science Foundation under Grant DRL-1906998. Any
        opinions, findings, and conclusions or recommendations
        expressed in this material are those of the authors and do not
        necessarily reflect the view of the National Science
        Foundation.
      </div>
    </div>
  </footer>

  <b-modal v-model="show_login" :width="640" aria-role="dialog" aria-label="Log in" aria-modal>
    <div class="card">
      <login-form @close="show_login=false">
        <dynamic-block group="login-modal" item="standard" class="content" />
      </login-form>
    </div>
  </b-modal>

  <b-modal v-model="show_signup" :width="640" aria-role="dialog" aria-label="Log in" aria-modal>
    <div class="card">
      <signup-form @close="show_signup=false">
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
import External from '~/components/External'
import DynamicBlock from '~/components/DynamicBlock'
import ArrowButton from '~/components/ArrowButton'
import LookupPlace from '~/components/LookupPlace'
import ActionButton from '~/components/ActionButton'

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

export default {
    components: {
        // Card,
        LoginForm,
        SignupForm,
        External,
        DynamicBlock,
        ArrowButton,
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
    },

    data () {
        const now = new Date();
        return {
            alert: false,
            menu: false,
            search: false,
            show_login: false,
            show_signup: false,
            show_person_dropdown: false,

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
    },

    async mounted() {
        await this.$store.dispatch('sync_local_to_server');
    },

    methods: {
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
        }
    }
}
</script>

<style lang="scss" scoped>

/* Increase if wider menu items are added */
$user-menu-width: 10rem;

/* Increase if more menu items are added */
$user-menu-height: 2rem;

#page {
    width: 100vw;
}

header {
    height: 45px;
    background-color: $snm-color-background-medlight;
    border-top: 2px solid $snm-color-background-dark;

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
    }

    .no-icon {
        margin-left: 1rem;
    }

    .logo {
        position: absolute;
        top: 3px;
        left: 75px;
        left: calc(50% - 36px);
        height: 39px;
        width: auto;

        img {
            width: 100%;
            height: 100%;
        }
    }

    .toggle-menu {
        background-color: transparent;
        border: 0px;
        position: absolute;
        top: 10px;
        left: 10px;
    }

    .toggle-search {
        background-color: $snm-color-action;
        border: 1px solid $snm-color-action-border;
        border-radius: 5px;
        position: absolute;
        top: 10px;
        right: 10px;
        padding: 5px;
        width: 24px;
        height: 24px;
        box-shadow: 0px 3px 6px $snm-color-shadow;

        img {
            display: block;
            width: 14px;
            height: 14px;
        }
    }

    .menu-box {
        display: none;
        position: absolute;
        top: 45px;
        left: 0px;
        right: 0px;
        z-index: 20;
        background-color: $snm-color-element-dark;
        text-align: left;
        box-shadow: 0px 3px 6px $snm-color-shadow;

        .not-authenticated {
            display: flex;
            justify-content: space-evenly;
            background-color: $snm-color-element-med;

            button {
                display: inline-flex;
                align-items: center;
                justify-content: center;
                width: 9rem;
                box-shadow: 0px 3px 6px $snm-color-shadow;

                &:hover,&:active {
                    color: $snm-color-element-med;
                    background-color: $snm-color-element-light;
                }
            }
        }

        .authenticated {
            li {
                border: 1px solid $snm-color-border-ondark;
                line-height: 48px;

                a {
                    color: $snm-color-element-ondark;

                    img {
                        color: $snm-color-background-medium;
                    }
                }
            }
        }
    }

    .search-box {
        display: none;
        position: absolute;
        top: 45px;
        left: 0px;
        right: 0px;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        background-color: $snm-color-background;
        min-height: 4rem;
        z-index: 10;
        box-sizing: border-box;
        padding: 1rem;

        input[type="date"] {
            padding: 0.75rem;
            border-radius: 10px;
            border: 1px solid #B4B4B4;
        }

        .arrow-button {
            color: #fff;
            display: block;
            margin: 1rem;
            text-align: center;
        }
    }

    .toggled {
        display: block;
    }
}

footer {
    background-color: $snm-color-background-dark;

    li {
        line-height: 40px;
        padding-left: 1rem;
        border-bottom: 1px solid $snm-color-border-ondark;

        h1 {
            color: $snm-color-heading-ondark;
            font-family: $snm-font-heading;
            font-size: 14px;
            font-weight: bold;
            letter-spacing: 0.7px;
        }

        a {
            color: $snm-color-element-ondark;
            font-family: $snm-font-content;
            font-size: 16px;
            letter-spacing: 0px;
        }
    }

    .partner {
        display: flex;
        flex-direction: row;

        .logo {
            margin: 16px;
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
            font-size: 14px;
            line-height: 22px;
            letter-spacing: 0px;
            margin: 16px 16px 16px 0px;
        }
    }
}

#main {
    >nav,>.logo {
        display: none;
    }
}

@media only screen and (min-width: $fullsize-screen) {
    header {
        height: 100px;
        border-top: 10px solid $snm-color-prehead;

        .logo {
            top: 25px;
            left: 30px;
            width: 180px;
            height: auto;
        }

        .toggle-search {
            top: 30px;
            right: 10px;
            padding: 10px;
            width: 40px;
            height: 40px;

            img {
                width: 20px;
                height: 20px;
            }
        }

        .menu-box {
            display: inline-block;
            background-color: transparent;
            box-shadow: none;
            left: unset;
            right: 60px;
            top: 18px;

            .not-authenticated {
                display: inline-block;
                background-color: transparent;

                button {
                    width: auto;
                    box-shadow: 0px 3px 6px $snm-color-shadow;

                    &:hover,&:active {
                        color: $snm-color-element-med;
                        background-color: $snm-color-element-light;
                    }
                }
            }

            .authenticated {
                display: flex;
                flex-wrap: wrap;
                align-items: center;
                justify-content: right;
                margin-top: 12px;
                height: 40px;
                border-radius: 6px;
                background-color: $snm-color-background-meddark;
                color: $snm-color-element-light;
                box-sizing: border-box;
                border: 1px solid $snm-color-background-dark;
                transition: border-bottom-right-radius 0.5s, botder-bottom-left-radius 0.5s;
                padding: 0px 1rem;
                min-width: $user-menu-width;

                > span {
                    font-family: $snm-font-content;
                    font-weight: bold;
                    font-size: 16px;
                    cursor: pointer;

                    &::after {
                        content: ">";
                        display: inline-flex;
                        align-items: center;
                        justify-content: center;
                        margin-left: 1rem;
                        width: 1.5em;
                        height: 1.5em;
                        border-radius: 1em;
                        color: $snm-color-element-dark;
                        background-color: $snm-color-element-light;
                        transform: rotate(0.25turn);
                        transition: transform;
                    }
                }

                > ul {
                    position: absolute;
                    top: 50px;
                    right: 5px;
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
                        transform: rotate(-0.25turn);
                        transition: transform;
                    }
                }

                ul {
                    max-width: $user-menu-width;
                    max-height: $user-menu-height;

                    right: 0px;
                    transition: max-height 0.5s, max-width 0.5s, right 0.5s;
                }
            }
        }
    }

    #content {
        margin: 34px 170px 10px 190px;
    }

    .authenticated #main {
        display: flex;
        flex-direction: row-reverse;

        a.logo {
            display: block;
            position: absolute;
            top: 0px;
            left: 0px;
            width: 280px;
            height: 120px;
            padding: 1rem;
            background-color: $snm-color-background-medium;
            box-shadow: 0px 0px 6px $snm-color-shadow;
            clip-path: inset(0px -15px 0px 0px);

            img {
                width: 100%;
                height: 100%;
                object-position: center center;
                object-fit: contain;
            }
        }

        >:not(nav) {
            flex-grow: 1;
        }

        nav {
            margin-top: 20px;
            display: flex;
            flex-direction: column;
            width: 280px;
            background-color: $snm-color-background-medium;
            flex-grow: 0;
            flex-shrink: 0;
            box-shadow: 0px 0px 6px $snm-color-shadow;
            clip-path: inset(0px -2rem 0px 0px);

            strong {
                font-family: $snm-font-content;
                font-weight: bold;
                font-size: 13px;
                line-height: 15px;
                letter-spacing: 0.39px;
                text-transform: uppercase;
                padding: 2rem 0px 0px 1rem;

                &.nav-separate {
                    margin-top: 2rem;
                    border-top: 1px solid $snm-color-background-light;
                }
            }

            a,a:hover,a:active {
                display: flex;
                color: $snm-color-element-dark;
                background-color: transparent;
                align-items: center;
                font-family: $snm-font-content;
                font-weight: normal;
                font-size: 16px;
                line-height: 52px;
                padding-left: 1rem;
                position: relative;
                box-sizing: border-box;
                height: 52px;

                &.nuxt-link-active {
                    background-color: $snm-color-background-meddark;
                    color: $snm-color-element-light;

                    &::after {
                        position: absolute;
                        right: -26px;
                        content: "";
                        width: 0px;
                        height: 52px;
                        border-left: 26px solid $snm-color-background-meddark;
                        border-top: 26px solid transparent;
                        border-bottom: 26px solid transparent;
                    }
                }

                svg {
                    margin-right: 1rem;

                    * {
                        fill: currentColor;
                    }
                }
            }
        }
    }

    footer {
        display: flex;
        justify-content: space-evenly;
        padding: 60px 0px;

        > ul {
            li {
                border: 0px;

                h1 {
                    text-decoration: underline;
                }
            }
        }

        > div {
            max-width: 30vw;
        }
    }

    .authenticated footer {
        border-left: 280px solid $snm-color-background-medium;
    }
}
</style>

<style lang="scss">
.modal .card {
    margin: 1rem;
    padding: 1rem;
}

@media (max-width: $mobile-screen) {
    .no-mobile {
        display: none !important;
    }
}

@media (min-width: $fullsize-screen) {
    .mobile-only {
        display: none !important;
    }
}
</style>
