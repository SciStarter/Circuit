<template>
  <div class="snm-wrapper">
    <div class="snm-container">
      <div class="was-p">
        Do you have a
        <img src="~/assets/img/scistarter-logo.svg" alt="SciStarter">
        account?
        <a @click="via_scistarter">
          Log in with your SciStarter account
        </a>.
        <b-tooltip label="SciStarter is a citizen science database." position="is-left">
          <b-button label="?" />
        </b-tooltip>
      </div>
      <div class="flex">
        <div class="standalone-form">
          <h1>Log In</h1>
          <login-form :next="$route.query.next" :query="next_query" @cancel="$router.back()" hide-extras />
        </div>
          <div class="standalone-form">
            <h1>Create an Account</h1>
            <signup-form :next="$route.query.next" :query="next_query" @cancel="$router.back()" hide-extras />
          </div>
      </div>
    </div>
    <PageView/>
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
.flex {
  justify-content:space-between;
  margin-top:2rem;
}
  .standalone-form {
    padding: 2rem 3rem;
    border: 1px solid $snm-color-border;
    max-width: 800px;
    width:48%;
    border-radius: 6px;

    h1 {
      font-size: 1.8rem;
      color: $snm-color-background-meddark;
      text-align: center;
      font-weight: bold;
      font-family: $snm-font-heading;
      margin-bottom:1rem;
    }
    .field.is-floating-label {
      margin-bottom: 2rem;
    }
  }
  div.was-p {
    margin: 0.6rem auto;
    padding: 0.6rem 0;
    text-align:center;
    &:first-child {
      margin-bottom:0;
    }
    &:last-child {
      border-top:1px solid $snm-color-border;
      border-bottom:1px solid $snm-color-border;
      margin-bottom: 2rem;
      margin-top:0;
    }
    img {
      width: 72px;
      vertical-align: middle;
      position: relative;
      top: 2px;
    }
  }
  .tooltip-trigger button {
    height: 1rem;
    width: 1rem;
    border-radius: 100%;
    padding: 0.5rem;
    font-size: 14px;
    margin-left: 6px;
    background-color: $snm-color-action;
  }
  @media (max-width:767px){
    .flex {
      flex-direction:column;
    }
    .standalone-form {
      width:100%;
    }
    .flex > div:first-child {
      margin-bottom:2rem;
    }
  }
</style>
