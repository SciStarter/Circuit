<template>
<div id="page" class="container">
  <header>
    <button class="toggle-menu" title="Toggle menu" :aria-pressed="String(menu)" data-context="header-menu" @click="menu = !menu">
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
        <ul>
          <li>
            <nuxt-link to="/find">
              <find-icon class="menu-icon" /> Find Science Opportunities
            </nuxt-link>
          </li>
          <li>
            <nuxt-link to="/my/saved">
              <saved-icon class="menu-icon" /> Saved Science Opportunities
            </nuxt-link>
          </li>
          <li>
            <nuxt-link to="/my/science">
              <science-icon class="menu-icon" /> My Science
            </nuxt-link>
          </li>
          <li>
            <nuxt-link to="/my/goals">
              <goals-icon class="menu-icon" /> My Goals
            </nuxt-link>
          </li>
          <li>
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

  <nuxt @login="show_login = true" @signup="show_signup = true" />

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
        SearchIcon
    },

    data () {
        const now = new Date();
        return {
            alert: false,
            menu: false,
            search: false,
            show_login: false,
            show_signup: false,

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
            return this.$store.state.user.authenticated;
        },

        username() {
            return this.$store.state.user.username;
        }
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

    .logo img {
        position: absolute;
        top: 3px;
        left: 75px;
        left: calc(50% - 36px);
        height: 39px;
        width: auto;
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
            text-align: center;
            background-color: $snm-color-element-med;

            button {
                min-width: 9rem;
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

@media (min-width: $fullsize-screen) {

}
</style>

<style lang="scss">
.modal .card {
    margin: 1rem;
    padding: 1rem;
}
</style>
