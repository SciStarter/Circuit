<template>
  <div class="login-form">
    <slot/>
    <b-field label="Email" :type="validate_email.type" :message="validate_email.message" label-position="on-border">
      <b-input type="email" v-model="login.email" required />
    </b-field>
    <b-field label="Password" :type="validate_password.type" :message="validate_password.message" label-position="on-border">
      <b-input type="password" v-model="login.password" required />
    </b-field>
    <div>
      <b-button @click="cancel" type="is-primary is-light">Cancel</b-button>
      <b-button @click="log_in" :loading="working" type="is-primary">Log in</b-button>
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
    props: {},

    data() {
        return {
            working: false,

            login: {
                email: "",
                password: "",
            },
        };
    },

    computed: {
        validate_email() {
            if(this.login.email.length == 0) {
                return {type: "", message: "", valid: false};
            }

            let at_pos = this.login.email.indexOf('@');

            if(at_pos < 1 || at_pos == this.login.email.length - 1) {
                return {type: 'is-danger', message: 'Invalid email address', valid: false};
            }

            return {type: "is-success", message: "", valid: true};
        },

        validate_password() {
            if(this.login.password.length == 0) {
                return {type: "", message: "", valid: false};
            }

            if(this.login.password.length < 7) {
                return {type: "is-danger", message: "Password is too short", valid: false};
            }

            return {type: "is-success", message: "", valid: true};
        },

        valid() {
            return this.validate_email.valid && this.validate_password.valid;
        }
    },

    methods: {
        cancel() {
            this.$emit('close');
            this.login.email="";
            this.login.password="";
        },

        async log_in() {
            if(!this.valid) {
                return;
            }

            this.working = true;

            let user = await this.$store.dispatch('login', this.login);

            this.working = false;

            if(user.authenticated) {
                this.$emit('close');
            }
            else {
                this.$buefy.dialog.alert('Invalid email or password.');
            }
        }
    }
}
</script>
