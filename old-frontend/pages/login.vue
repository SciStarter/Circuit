<template>
  <div class="snm-wrapper">
    <div class="forgot-alert snm-container">
      <b-message 
            type="is-warning" 
            aria-close-label="Close message">
            If your email address exists in our database, you should receive a password recovery link at your email address in a few minutes.
        </b-message>
    </div>
    <div class="snm-container standalone-form">
      <h1>Log In</h1>

      <login-form :next="$route.query.next" :query="next_query" @cancel="$router.back()" />
    </div>
  </div>
</template>

<script>
import cloneDeep from 'lodash/cloneDeep'

export default {
    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(user.authenticated) {
            let query = context.route.query;
            let next = query.next || "/";
            delete query.next;

            if(next.startsWith("/")) {
                context.redirect({path: next, query: query});
            }
            else {
                context.redirect({name: next, query: query});
            }
        }
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
    }
    .field.is-floating-label {
      margin-bottom: 2rem;
    }
  }
  .forgot-alert {
    max-width: 800px;
    margin:2rem auto;
  }
</style>
