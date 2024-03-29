<template>
<div class="signup-form" :class="{'evolveme':partner=='evolveme'}">
  <slot />
  <div class="form-header" v-if="!hideExtras">
    <p>Already have an account?
      <a v-if="inModal" @click="$emit('login')">Login here</a>
      <a v-else @click="$router.replace(partner ? {name: 'exchange-uid-login', params: {uid: partner}, query: $route.query} : {name: 'login', query: $route.query})">Login here</a>.
    </p>
    <p v-if="!partner" >Do you have a <img src="~/assets/img/scistarter-logo.svg" alt="SciStarter" /> account? <a href="/login-scistarter">Log in with your SciStarter account</a>.<b-tooltip label="SciStarter is a citizen science database."  position="is-left">
        <b-button label="?" />
    </b-tooltip></p>
  </div>
  <form @submit.prevent="0">
    <b-field :type="validate_email.type" :message="validate_email.message" label-position="on-border">
      <template #label>
        Email <span class="has-required">*</span>
      </template>
      <b-input v-model="signup.email" type="email" required />
    </b-field>
    <b-field label-position="on-border">
      <template #label>
        Username <span class="has-required">*</span>
      </template>
      <b-input v-model="signup.username" type="text" required />
    </b-field>
    <b-field :type="validate_password.type" :message="validate_password.message" label-position="on-border">
      <template #label>
        Password <span class="has-required">*</span>
      </template>
      <b-input v-model="signup.password" type="password" required />
    </b-field>
    <b-field :type="validate_password_repeat.type" :message="validate_password_repeat.message" label-position="on-border">
      <template #label>
        Repeat Password <span class="has-required">*</span>
      </template>
      <b-input v-model="password_repeat" type="password" required />
    </b-field>
    <b-field label-position="on-border">
      <template #label>
        Zip/Postal Code
      </template>
      <b-input v-model="signup.zip_code" type="text" />
    </b-field>
    <b-field label-position="on-border">
      <template #label>
        Phone <span class="has-optional">optional</span>
      </template>
      <b-input v-model="signup.phone" type="tel" />
    </b-field>
    <div class="form-push">
      <b-field>
        <b-checkbox v-model="signup.agree">
          I agree to the <a href="/terms" target="_blank">Terms of
            Service</a>, <a href="/privacy" target="_blank">Privacy
            Policy</a>, and <a href="/research-participant" target="_blank">Research Participation Agreement</a>.
        </b-checkbox>
      </b-field>
      <b-field>
        <b-checkbox v-model="signup.newsletter">
          Sign up for the Science Near Me Newsletter
        </b-checkbox>
      </b-field>
    </div>
    <div v-if="partner=='evolveme'" class="center">
      <action-button :loading="working" principal @click="sign_up">
        Create My Account
      </action-button>
    </div>
    <div v-else class="flex flex-justify-sb">
      <action-button :loading="working" type="is-primary" primary @click="sign_up">
        Sign up
      </action-button>
      <b-button v-if="!notCancelable" type="is-text" @click="inModal ? $emit('close') : $router.go(-1);">
        Cancel
      </b-button>
    </div>
  </form>
</div>
</template>

<script>

const LIKE_UUID = /^[a-z,0-9,-]{36,36}$/;

export default {
    name: "SignupForm",

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
            default: null,
        },

        hideExtras: {
            type: Boolean,
            required: false,
            default: false,
        },

        triggerSuccess: {
            type: Function,
            required: false,
            default: () => {}
        },
    },

    data() {
        return {
            working: false,

            password_repeat: '',

            signup: {
                email: '',
                username: '',
                password: '',
                zip_code: '',
                phone: '',
                agree: true,
                newsletter: true,
                next: this.next || '/',
                next_query: this.query,
                exchange: LIKE_UUID.test(this.partner) ? this.partner : undefined,
            }
        }
    },

    computed: {
        validate_email () {
            if (this.signup.email.length === 0) {
                return { type: '', message: '', valid: false }
            }

            const at = this.signup.email.indexOf('@')

            if (at < 1 || at === this.signup.email.length - 1) {
                return { type: 'is-danger', message: 'Invalid email address', valid: false }
            }

            return { type: 'is-success', message: '', valid: true }
        },

        validate_password () {
            if (this.signup.password.length === 0) {
                return { type: '', message: '', valid: false }
            }

            if (this.signup.password.length < 7) {
                return { type: 'is-danger', message: 'Password is too short', valid: false }
            }

            return { type: 'is-success', message: '', valid: true }
        },

        validate_password_repeat () {
            if (this.signup.password.length === 0) {
                return { type: '', message: '', valid: false }
            }

            if (this.signup.password !== this.password_repeat) {
                return { type: 'is-danger', message: 'Passwords do not match', valid: false }
            }

            return { type: 'is-success', message: '', valid: true }
        },

        valid () {
            return this.validate_email.valid && this.validate_password.valid && this.validate_password_repeat.valid
        }
    },

    methods: {
        async sign_up () {
            if (!this.valid) {
                return
            }

            this.working = true

            const user = await this.$store.dispatch('signup', this.signup)

            this.working = false

            if (user.authenticated) {
                this.$emit('close')
            } else {
                this.$buefy.dialog.alert(user.message || 'An account with that email already exists, or you did not agree to the terms.')
            }
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
  p {
    margin: 0.6rem 0;
    padding: 0.6rem 0;
    border-top:1px solid $snm-color-border;
    &:first-child {
      margin-bottom:0;
    }
    &:last-child {
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
.form-push {
  margin-bottom: 1rem;
}
.standalone-form {
  .field.is-floating-label, .form-push {
    margin-bottom: 2rem;
  }
  .flex {
    display: flex;
    justify-content: space-between;
  }
}
.has-required {
  color: $snm-color-info;
  font-size: 12px;
}
.has-optional {
  font-weight: 400;
  color: $snm-color-border;
  font-size: 12px;
}
.center {
  margin:0 auto;
}

.evolveme .form-header p {
  border-top:0;
  margin-bottom:3rem;
  padding-top: 0;
  padding-bottom: 1.5rem;
}

.evolveme .center {
  display: flex;
  justify-content: center;
  align-items: center;
  margin:2rem 0;
}

</style>
