<template>
  <div class="snm-wrapper">
    <div class="snm-container standalone-form">
      <h1>Log In with Your SciStarter Account</h1>
      <login-scistarter-form :next="$route.query.next" :query="next_query" @cancel="$router.back()" />
      <p class="legal">
        If you do not remember your SciStarter password, you
        can <a href="https://scistarter.org/login"
        target="_blank">reset it on SciStarter</a>.
      </p>
      <p class="legal">
        You are granting SciStarter permission to share your profile
        information and user data with Science Near Me (SNM) and you
        agree to the
        <a href="/terms" target="_blank">Terms of Service</a>,
        <a href="/privacy" target="_blank">Privacy Policy</a>, and
        <a href="/research-participant" target="_blank">Participant Research Agreement</a>.
      </p>
    </div>
  </div>
</template>

<script>
import cloneDeep from 'lodash/cloneDeep'

export default {
    name: "LoginSciStarter",

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    computed: {
        next_query() {
            let q = cloneDeep(this.$route.query);
            delete q.next;
            return q;
        }
    },
}
</script>

<style lang="scss" scoped>
  .standalone-form {
    padding: 2rem 3rem;
    border: 1px solid $snm-color-border;
    max-width: 800px;
    border-radius: 6px;

    h1 {
      font-size: 1.8rem;
      color: $snm-color-background-meddark;
      text-align: center;
      font-weight: bold;
      font-family: $snm-font-heading;
      margin-bottom:2rem;
    }
    .field.is-floating-label {
      margin-bottom: 2rem;
    }
    .legal {
        margin-top: 2rem;
        font-size: 90%;
    }
  }
</style>
