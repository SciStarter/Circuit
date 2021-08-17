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
      <arrow-button @click="$buefy.toast.open('moo')" style="color: #fff"><search-icon class="button-icon"/> Search</arrow-button>
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
    }

    .toggled {
        display: block;
    }
}

footer {
    background-color: $snm-color-background-dark;
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
                    name: "",
                    longitude: 0,
                    latitude: 0,
                    radius: 0
                }
            },
        }
    },

    computed: {
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
        logout() {
            // !!!
        }
    }
}
</script>
