<template>
  <div class="signup-form">
    <slot/>
    <b-field label="Email" :type="validate_email.type" :message="validate_email.message" label-position="on-border">
      <b-input type="email" v-model="signup.email" required />
    </b-field>
    <b-field label="Username" label-position="on-border">
      <b-input type="text" v-model="signup.username" required />
    </b-field>
    <b-field label="Password" :type="validate_password.type" :message="validate_password.message" label-position="on-border">
      <b-input type="password" v-model="signup.password" required />
    </b-field>
    <b-field label="Repeat password" :type="validate_password_repeat.type" :message="validate_password_repeat.message" label-position="on-border">
      <b-input type="password" v-model="password_repeat" required />
    </b-field>
    <b-field label="Zip / Postal Code" label-position="on-border">
      <b-input type="text" v-model="signup.zip_code"/>
    </b-field>
    <b-field label="Phone" label-position="on-border">
      <b-input type="tel" v-model="signup.phone"/>
    </b-field>
    <div>
      <b-button @click="cancel" type="is-primary is-light">Cancel</b-button>
      <b-button @click="sign_up" :loading="working" type="is-primary">Sign up</b-button>
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
    props: {},

    data() {
        return {
            working: false,

            password_repeat: "",

            signup: {
                email: "",
                username: "",
                password: "",
                zip_code: "",
                phone: "",
            },
        };
    },

    computed: {
        validate_email() {
            if(this.signup.email.length == 0) {
                return {type: "", message: "", valid: false};
            }

            let at_pos = this.signup.email.indexOf('@');

            if(at_pos < 1 || at_pos == this.signup.email.length - 1) {
                return {type: 'is-danger', message: 'Invalid email address', valid: false};
            }

            return {type: "is-success", message: "", valid: true};
        },

        validate_password() {
            if(this.signup.password.length == 0) {
                return {type: "", message: "", valid: false};
            }

            if(this.signup.password.length < 7) {
                return {type: "is-danger", message: "Password is too short", valid: false};
            }

            return {type: "is-success", message: "", valid: true};
        },

        validate_password_repeat() {
            if(this.signup.password.length == 0) {
                return {type: "", message: "", valid: false};
            }

            if(this.signup.password != this.password_repeat) {
                return {type: "is-danger", message: "Passwords do not match", valid: false};
            }

            return {type: "is-success", message: "", valid: true};
        },

        valid() {
            return this.validate_email.valid && this.validate_password.valid && this.validate_password_repeat.valid;
        }
    },

    methods: {
        cancel() {
            this.$emit('close');
            this.signup.email="";
            this.signup.password="";
            this.password_repeat="";
            this.signup.zip_code="";
            this.signup.phone="";
        },

        async sign_up() {
            if(!this.valid) {
                return;
            }

            this.working = true;

            let user = await this.$store.dispatch('signup', this.signup);

            this.working = false;

            if(user.authenticated) {
                this.$emit('close');
            }
            else {
                this.$buefy.dialog.alert("An account with that email already exists.");
            }
        }
    }
}
</script>