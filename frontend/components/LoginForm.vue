<template>
  <div class="login-form">
    <slot />
    <b-field label="Email" :type="validate_email.type" :message="validate_email.message" label-position="on-border">
      <b-input v-model="login.email" type="email" required />
    </b-field>
    <b-field label="Password" :type="validate_password.type" :message="validate_password.message" label-position="on-border">
      <b-input v-model="login.password" type="password" required />
    </b-field>
    <div>
      <b-button type="is-primary is-light" @click="cancel">
        Cancel
      </b-button>
      <b-button :loading="working" type="is-primary" @click="log_in">
        Log in
      </b-button>
    </div>
  </div>
</template>

<script>
/*

    This component presents a form allowing the user to log in. Once
    the user has successfully logged in, or clicked the cancel button,
    it emits a 'close' event.

  */
export default {
    name: "LoginForm",

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

            login: {
                email: '',
                password: '',
                next: this.next,
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
        cancel () {
            this.$emit('close')
            this.login.email = ''
            this.login.password = ''
        },

        async log_in () {
            if (!this.valid) {
                return
            }

            this.working = true

            const user = await this.$store.dispatch('login', this.login)

            this.working = false

            if (user.authenticated) {
                this.$emit('close')
            } else {
                this.$buefy.dialog.alert('Invalid email or password.')
            }
        }
    }
}
</script>
