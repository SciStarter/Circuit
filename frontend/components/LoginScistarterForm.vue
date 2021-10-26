<template>
  <div class="login-scistarter-form">
    <slot />
    <!-- <div class="form-header">
      <p>Don't have an account? <a href="/signup">Create one now</a>.</p>
    </div> -->
    <b-field label="Email" :type="validate_email.type" :message="validate_email.message" label-position="on-border">
      <b-input v-model="login.email" type="email" required />
    </b-field>
    <b-field label="Password" :type="validate_password.type" :message="validate_password.message" label-position="on-border">
      <b-input v-model="login.password" type="password" required />
    </b-field>
    <div class="flex flex-justify-sb">
      <b-button :loading="working" type="is-primary" @click="log_in">
        Log in
      </b-button>
      <b-button type="is-text" @click="cancel">
        Cancel
      </b-button>
    </div>
  </div>
</template>

<script>
export default {
    name: "LoginSciStarterForm",

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
                via: 'scistarter',
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

            return { type: 'is-success', message: '', valid: true }
        },

        valid () {
            return this.validate_email.valid && this.validate_password.valid
        }
    },

    methods: {
        cancel () {
            this.$emit('cancel')
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
                this.$buefy.dialog.alert(user.message)
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

</style>
