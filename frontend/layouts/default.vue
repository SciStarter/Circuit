<template>
<div>

  <header>
    <button class="toggle-menu" @click="menu = !menu" title="Toggle menu" :aria-pressed="String(menu)">
      <img v-if="alert" src="~assets/img/hamburger-alert.svg?data" >
      <img v-else src="~assets/img/hamburger.svg?data">
    </button>

    <nuxt-link to="/" class="logo" title="Science Near Me logo">
      <img src="~assets/img/logo.svg?data">
    </nuxt-link>

    <button class="toggle-search" @click="search = !search" title="Toggle search box" :aria-pressed="String(search)"><img src="~assets/img/search.svg?data"></button>

    <aside :class="{toggled: search}" class="search-box">
      <b-field>
        <b-input v-model="query.keywords" placeholder="e.g. astronomy, bar crawl" icon="magnify"/>
      </b-field>
      <lookup-place v-model="query.place"/>
      <b-field>
        <b-checkbox v-model="query.include_online">Include Online Opportunities</b-checkbox>
      </b-field>
      <b-field>
        <b-field label="From">
          <b-datepicker v-model="query.date_from"/>
        </b-field>
        <b-field label="Until">
          <b-datepicker v-model="query.date_until"/>
        </b-field>
      </b-field>
      <arrow-button @click="find" style="color: #fff"><search-icon class="button-icon"/> Search</arrow-button>
    </aside>

    <aside :class="{toggled: menu}" class="menu-box">
      <div v-if="authenticated" class="authenticated">
        <ul>
          <li><nuxt-link to="/find"><find-icon class="menu-icon"/> Find Science Opportunities</nuxt-link></li>
          <li><nuxt-link to="/my/saved"><saved-icon class="menu-icon"/> Saved Science Opportunities</nuxt-link></li>
          <li><nuxt-link to="/my/science"><science-icon class="menu-icon"/> My Science</nuxt-link></li>
          <li><nuxt-link to="/my/goals"><goals-icon class="menu-icon"/> My Goals</nuxt-link></li>
          <li><nuxt-link to="/my/profile"><profile-icon class="menu-icon"/> My Profile</nuxt-link></li>
          <li><span class="no-icon"/><a @click="logout">Log Out</a></li>
        </ul>
      </div>
      <div v-else class="not-authenticated">
        <button @click="show_login = true">Login</button>
        <button @click="show_signup = true">Create Account</button>
      </div>
    </aside>
  </header>

  <nuxt/>

  <footer>
    <ul>
      <li><h1>For Everyone</h1></li>
      <li><nuxt-link to="/about">About Us</nuxt-link></li>
      <li><nuxt-link to="/terms">Terms of Service</nuxt-link></li>
      <li><nuxt-link to="/privacy">Privacy Policy</nuxt-link></li>
      <li><nuxt-link to="/cookies">Cookies Policy</nuxt-link></li>
      <li><nuxt-link to="/contact">Contact Us</nuxt-link></li>
    </ul>

    <ul>
      <li><h1>For Science Professionals</h1></li>
      <li><nuxt-link to="/affiliate">Be Part of Science Near Me</nuxt-link></li>
      <li><external href="/api/docs/v1.html" content="footer-link">API documentation</external></li>
      <li><nuxt-link to="/contact">Display Science Opportunities</nuxt-link></li>
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
      <div class="card-content">
        <login-form @close="show_login=false">
          <dynamic-block group="login-modal" item="standard" class="content"/>
        </login-form>
      </div>
    </div>
  </b-modal>

  <b-modal v-model="show_signup" :width="640" aria-role="dialog" aria-label="Log in" aria-modal>
    <div class="card">
      <div class="card-content">
        <signup-form @close="show_signup=false">
          <dynamic-block group="signup-modal" item="standard" class="content"/>
        </signup-form>
      </div>
    </div>
  </b-modal>

</div>
</template>

<style lang="scss" scoped>
header {
    height: 45px;
    background-color: $snm-color-background-medlight;
    border-top: 2px solid $snm-color-background-dark;

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
        z-index: 1;
        background-color: $snm-color-element-dark;
        text-align: left;
        box-shadow: 0px 3px 6px $snm-color-shadow;

        .not-authenticated {
            text-align: center;

            button {
                font-family: $snm-font-content;
                font-size: 16px;
                color: $snm-color-element-light;
                background-color: $snm-color-background-meddark;
                border: 1px solid $snm-color-element-light;
                border-radius: 6px;
                margin: 10px 1rem;
                padding: 10px;
                box-shadow: 0px 3px 6px $snm-color-shadow;
                min-width: 9rem;
                font-weight: bold;
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
        z-index: 1;
        box-sizing: border-box;
        padding: 1rem;
        text-align: center;
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
</style>

<script>
import Card from '~/components/Card'
import LoginForm from '~/components/LoginForm'
import SignupForm from '~/components/SignupForm'
import External from '~/components/External'
import DynamicBlock from '~/components/DynamicBlock'
import ArrowButton from '~/components/ArrowButton'
import LookupPlace from '~/components/LookupPlace'

import FindIcon from '~/assets/img/find-science-opportunities.svg?inline'
import SavedIcon from '~/assets/img/saved-science-opportunities.svg?inline'
import ScienceIcon from '~/assets/img/my-science.svg?inline'
import GoalsIcon from '~/assets/img/my-goals.svg?inline'
import ProfileIcon from '~/assets/img/my-profile-and-settings.svg?inline'
import SearchIcon from '~/assets/img/search.svg?inline'

export default {
    components: {
        Card,
        LoginForm,
        SignupForm,
        External,
        DynamicBlock,
        ArrowButton,
        LookupPlace,

        FindIcon,
        SavedIcon,
        ScienceIcon,
        GoalsIcon,
        ProfileIcon,
        SearchIcon,
    },

    data() {
        return {
            alert: false,
            menu: false,
            search: false,
            show_login: false,
            show_signup: false,

            query: {
                keywords: "",
                place: {
                    near: "",
                    lon: 0,
                    lat: 0,
                    radius: 0
                },
                include_online: true,
                date_from: null,
                date_until: null,
            },
        }
    },

    computed: {
        search_query() {
            let joint = "?";
            let ret = "";

            if(this.query.keywords) {
                ret += joint + "text=" + encodeURIComponent(this.query.keywords);
                joint = "&";
            }

            if(this.query.place.near) {
                ret += joint + "near=" + encodeURIComponent(this.query.place.near);
                joint = "&";
            }

            if(this.query.place.lon) {
                ret += joint + "longitude=" + encodeURIComponent(this.query.place.lon);
                joint = "&";
            }

            if(this.query.place.lat) {
                ret += joint + "latitude=" + encodeURIComponent(this.query.place.lat);
                joint = "&";
            }

            if(this.query.place.radius) {
                ret += joint + "proximity=" + encodeURIComponent(this.query.place.radius);
                joint = "&";
            }

            if(!this.query.include_online) {
                ret += joint + "online=false";
                joint = "&";
            }

            if(this.query.date_from !== null) {
                ret += joint + "beginning=" + encodeURIComponent(this.query.date_from.toISOString());
                joint = "&";
            }

            if(this.query.date_until !== null) {
                ret += joint + "ending=" + encodeURIComponent(this.query.date_until.toISOString());
                joint = "&";
            }

            return ret;
        },

        authenticated() {
            return this.$store.state.user.authenticated;
        },

        username() {
            return this.$store.state.user.username;
        },
    },

    async fetch() {
        await this.$store.dispatch('get_user');
    },

    methods: {
        find() {
            this.search = false;
            this.$router.push("/find" + this.search_query);
        },

        logout() {
            // !!!
        }
    }
}
</script>
