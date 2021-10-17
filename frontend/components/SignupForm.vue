<template>
  <div class="signup-form">
    <slot />
    <div class="form-header">
      <p>Already have an account? <a href="/login">Login here</a>.</p>
      <p>Do you have a <img src="~/assets/img/scistarter-logo.svg" alt="SciStarter" /> account? <a href="/login-scistarter">Log in with your SciStarter account</a>.<b-tooltip label="SciStarter is a citizen science database.">
          <b-button label="?" />
      </b-tooltip></p>
    </div>
    <b-field label="Email" :type="validate_email.type" :message="validate_email.message" label-position="on-border">
      <b-input v-model="signup.email" type="email" required />
    </b-field>
    <b-field label="Username" label-position="on-border">
      <b-input v-model="signup.username" type="text" required />
    </b-field>
    <b-field label="Password" :type="validate_password.type" :message="validate_password.message" label-position="on-border">
      <b-input v-model="signup.password" type="password" required />
    </b-field>
    <b-field label="Repeat password" :type="validate_password_repeat.type" :message="validate_password_repeat.message" label-position="on-border">
      <b-input v-model="password_repeat" type="password" required />
    </b-field>
    <b-field label="Zip / Postal Code" label-position="on-border">
      <b-input v-model="signup.zip_code" type="text" />
    </b-field>
    <b-field label="Phone" label-position="on-border">
      <b-input v-model="signup.phone" type="tel" />
    </b-field>
    <div class="flex flex-justify-sb">
    <b-field>
      <b-checkbox v-model="signup.agree">
        I agree to the <a href="/terms" target="_blank">Terms of
        Service</a> and <a href="/privacy" target="_blank">Privacy
        Policy</a>.
      </b-checkbox>
    </b-field>
    <b-field>
      <b-checkbox v-model="signup.newsletter">
        Sign up for the Science Near Me Newsletter
      </b-checkbox>
    </b-field>
    <div>
      <b-button type="is-primary is-light" @click="cancel">
        Cancel
      </b-button>
      <b-button :loading="working" type="is-primary" @click="sign_up">
        Sign up
      </b-button>
      <b-button type="is-primary is-light" @click="cancel">
        Cancel
      </b-button>
    </div>
  </div>
</template>

<script>
/*

  This component presents a form allowing the user to sign up. Once
  the user has successfully signed up, or clicked the cancel button,
  it emits a 'close' event.

*/
export default {
    name: "SignupForm",

    props: {
        next: {
            type: String,
            required: false,
            default: '',
        },

        query: {
            type: Object,
            required: false,
            default: () => ({}),
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
                next: this.next,
                next_query: this.query,
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
        cancel () {
            this.$emit('close')
            this.signup.email = ''
            this.signup.password = ''
            this.password_repeat = ''
            this.signup.zip_code = ''
            this.signup.phone = ''
        },

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
                this.$buefy.dialog.alert('An account with that email already exists, or you did not agree to the terms.')
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
.standalone-form {
  .field.is-floating-label {
    margin-bottom: 2rem;
  }
  .flex {
    display: flex;
    justify-content: space-between;
  }
}

</style>
