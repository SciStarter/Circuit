<template>
<section class="section">
  <div class="columns is-mobile">
    <card title="Free" icon="github">
      Open source on <external href="https://github.com/buefy/buefy">GitHub</external>
    </card>

    <card title="Responsive" icon="cellphone-link">
      <b class="has-text-grey">Every</b> component is responsive
    </card>

    <card title="Modern" icon="alert-decagram">
      Built with <external href="https://vuejs.org/">Vue.js</external> and <external href="http://bulma.io/">Bulma</external>
    </card>

    <card title="Lightweight" icon="arrange-bring-to-front">
      No other internal dependency
    </card>
  </div>
  <h1><dynamic-block group="homepage" item="demo-header" remove-paragraphs>The contents of a &lt;dynamic-block&gt; tag are displayed if the dynamic content can not be retrieved. If the dynamic content can't be retrieved <em>and</em> the tag has no contents, lorem ipsum is displayed.</dynamic-block></h1>
  <!-- Dynamic blocks are rendered as div elements, but you can use classes to change how they are displayed. -->
  <!-- The "content" class is provided by Bulma, and often makes WYSIWYG-created content look better-->
  <dynamic-block group="homepage" item="demo-dynamic" class="content demo"></dynamic-block>

  <p>
    Logged in user: {{username}}
  </p>
  <p>
    <a @click="show_login=true">login</a>
  </p>
  <p>
    <a @click="show_signup=true">signup</a>
  </p>

  <b-modal v-model="show_login" :width="640" aria-role="dialog" aria-label="Log in" aria-modal>
    <div class="card">
      <div class="card-content">
        <login-form @close="show_login=false">
          <div class="content">
            <h1>Basic login form</h1>
            <p>
              This modal contains this bit of introductory text, and a login form.
            </p>
          </div>
        </login-form>
      </div>
    </div>
  </b-modal>

  <b-modal v-model="show_signup" :width="640" aria-role="dialog" aria-label="Log in" aria-modal>
    <div class="card">
      <div class="card-content">
        <signup-form @close="show_signup=false">
          <div class="content">
            <h1>Basic signup form</h1>
            <p>
              This modal contains this bit of introductory text, and a signup form.
            </p>
          </div>
        </signup-form>
      </div>
    </div>
  </b-modal>
</section>
</template>

<style>
h1 {
    font-weight: bold;
    font-size: 120%;
}

.demo {
    color: #999;
}
</style>

<script>
import Card from '~/components/Card'
import External from '~/components/External'
import DynamicBlock from '~/components/DynamicBlock'
import LoginForm from '~/components/LoginForm'
import SignupForm from '~/components/SignupForm'

export default {
    name: 'HomePage',

    components: {
        Card,
        External,
        DynamicBlock,
        LoginForm,
        SignupForm,
    },

    data() {
        return {
            show_login: false,
            show_signup: false,
        };
    },

    computed: {
        username() {
            return this.$store.state.user.username;
        }
    },

    async fetch() {
        await this.$store.dispatch('get_user');
    },
}
</script>
