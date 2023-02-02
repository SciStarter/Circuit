<template>
  <div class="login-form">
    <slot />
    <div class="form-header" v-if="!hideExtras">
      <p>
        Or
        <a v-if="inModal" @click="$emit('signup')">Create a Science Near Me account</a>
        <a v-else @click="$router.replace(partner ? {name: 'exchange-uid-signup', params: {uid: partner}, query: $route.query} : {name: 'signup', query: $route.query})">Create a Science Near Me account</a>.
      </p>
      <div class="was-p" v-if="!partner">
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
    </div>
    <form @submit.prevent="0">
      <b-field label="Email" :type="validate_email.type" :message="validate_email.message" label-position="on-border">
        <b-input v-model="login.email" type="email" required />
      </b-field>
      <b-field label="Password" :type="validate_password.type" :message="validate_password.message" label-position="on-border">
        <b-input v-model="login.password" type="password" required />
      </b-field>
      <div class="forgot">
        <a @click="forgot">Send a one-time login link to my email</a>
      </div>
      <div class="flex flex-justify-sb">
        <action-button :loading="working" type="is-primary" primary @click="log_in">
          Log in
        </action-button>
        <b-button v-if="!notCancelable" type="is-text" @click="inModal ? $emit('close') : $router.go(-1);">
          Cancel
        </b-button>
      </div>
    </form>
  </div>
</template>

<script>
export default {
    name: "LoginForm",

    props: {
        next: {
            type: String,
            required: false,
            default: '/',
        },

        notCancelable: {
            type: Boolean,
            required: false,
            default: false,
        },

        query: {
            type: Object,
            required: false,
            default: () => ({}),
        },

        inModal: {
            type: Boolean,
            required: false,
            default: false,
        },

        partner: {
            type: String,
            required: false,
            default: false,
        },

        hideExtras: {
            type: Boolean,
            required: false,
            default: false,
        }
    },

    data() {
        return {
            working: false,

            login: {
                email: '',
                password: '',
                next: this.next || '/',
                next_query: this.query,
            }
        }
    },

    computed: {
        validate_email () {
            if (this.login.email.length === 0) {
                return { type: '', message: '', valid: false }
            }

            const at = this.login.email.indexOf('@')

            if (at < 1 || at === this.login.email.length - 1) {
                return { type: 'is-danger', message: 'Invalid email address', valid: false }
            }

            return { type: 'is-success', message: '', valid: true }
        },

        validate_password () {
            if (this.login.password.length === 0) {
                return { type: '', message: '', valid: false }
            }

            if (this.login.password.length < 7) {
                return { type: 'is-danger', message: 'Password is too short', valid: false }
            }

            return { type: 'is-success', message: '', valid: true }
        },

        valid () {
            return this.validate_email.valid && this.validate_password.valid
        }
    },

    methods: {
        async forgot() {
            let {result} = await this.$buefy.dialog.prompt({
                message: `Email Address`,
                inputAttrs: {
                    placeholder: 'e.g. person@example.com',
                },
                trapFocus: true,
            });

            try {
                await this.$axios.$post('/api/ui/auth/reset', {email: result});
                this.$buefy.dialog.alert('Login message sent');
            }
            catch(err) {
                this.$buefy.dialog.alert({
                    title: "Error",
                    message: "One time login request failed",
                    type: 'is-danger',
                    hasIcon: true,
                    icon: 'times-circle',
                    iconPack: 'fa',
                    ariaRole: 'alertdialog',
                    ariaModal: true
                });
            }
        },

        async log_in () {
            if (!this.valid) {
                return
            }

            this.working = true

            const user = await this.$store.dispatch('login', this.login);

            this.working = false

            if (user.authenticated) {
                this.$emit('close')
            } else {
                this.$buefy.dialog.alert('Invalid email or password.')
            }
        },

        async via_scistarter() {
            this.$emit('close');
            this.$router.push({path: '/login-scistarter', query: {next: this.$route.fullPath}});
        }
    }
}
</script>

<style lang="scss" scoped>
.help {
  font-size:1rem;
}
.form-header {
  text-align:center;
  div.was-p {
    margin: 0.6rem 0;
    padding: 0.6rem 0;
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
}
.standalone-form {
  p:first-child {
    border-top:1px solid $snm-color-border;
  }
  .field.is-floating-label {
    margin-bottom: 2rem;
  }
  .flex {
    display: flex;
    justify-content: space-between;
  }
}
.forgot {
  text-align: right;
}

</style>
