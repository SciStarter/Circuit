<template>
  <div class="snm-wrapper">
    <div class="snm-container standalone-form">
      <h1>Log In with Your SciStarter Account</h1>
        <login-scistarter-form :next="$route.query.next" :query="next_query" @cancel="$router.go(1)" />
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
  }
</style>
