<template>
<div class="my-profile">
  <component :is="tabs" v-model="current_tab" type="is-boxed">
    <component :is="tab" label="Profile & Settings">
      <h1>My Profile &amp; Settings</h1>
      <profile-item v-model="profile.email" label="Email (login)" @input="save" />
      <profile-item v-model="profile.password" label="Password" obscure @input="save" />
      <profile-item v-model="profile.username" label="Display Name" @input="save" />
      <profile-item v-model="profile.zip_code" label="Zip/Postal Code" @input="save" />
      <profile-item v-model="profile.phone_number" label="Phone Number" @input="save" />
      <profile-item v-model="profile.first_name" label="First Name" @input="save" />
      <profile-item v-model="profile.last_name" label="Last Name" @input="save" />
      <profile-item v-model="profile.private" label="Privacy Setting" label-true="Private" label-false="Public" @input="save" />
      <div id="allow-comm">
        <label class="label">Accept messages from all users, opportunity providers, and researchers.</label>
        <b-field>
           <b-switch v-model="profile.allow_emails"
               true-value="Yes"
               false-value="No"
               type="is-success"
               @input="save">
               {{ profile.allow_emails }}
           </b-switch>
       </b-field>
       <small>If you turn this off, only Science Near Me staff and Opportunity Providers can email you.</small>
      </div>
    </component>
    <component :is="tab" label="Research Questions" class="research-questions">
      <h2>The following fields are used for scientific research.</h2>
      <em>Consider adding them to help us study public engagement in science and informal science learning.</em>

      <div class="research-item">
        <div>
          <label>Gender</label>
          <small>Check one or more options</small>
        </div>
        <div>
          <b-checkbox v-model="gender_female">
            Female
          </b-checkbox>
          <b-checkbox v-model="gender_male">
            Male
          </b-checkbox>
          <b-checkbox v-model="gender_nonbinary">
            Non-binary
          </b-checkbox>
          <b-checkbox v-model="gender_trans">
            Transgender
          </b-checkbox>
          <b-checkbox v-model="gender_inter">
            Intersex
          </b-checkbox>
          <div>
            <b-checkbox v-model="gender_other">
              Other
            </b-checkbox>
            <b-input v-model="gender_other_text" type="text" />
          </div>
        </div>
      </div>

      <div class="research-item">
        <div>
          <label>Birth Year</label>
        </div>
        <div>
          <b-input :value="birth_year" type="text" @input="birth_year = parseInt($event) || null" />
        </div>
      </div>

      <div class="research-item">
        <div>
          <label>Race/Ethnicity</label>
          <small>Check one or more options</small>
        </div>
        <div>
          <b-checkbox v-model="eth_asian">
            Asian
          </b-checkbox>
          <b-checkbox v-model="eth_black">
            Black or African
          </b-checkbox>
          <b-checkbox v-model="eth_latin">
            Latinx or Hispanic
          </b-checkbox>
          <b-checkbox v-model="eth_mideast">
            Middle Eastern or North African
          </b-checkbox>
          <b-checkbox v-model="eth_americas">
            Native American or Native Alaskan
          </b-checkbox>
          <b-checkbox v-model="eth_pacific">
            Native Hawaiian or Pacific Islander
          </b-checkbox>
          <b-checkbox v-model="eth_europe">
            White or Caucasian
          </b-checkbox>
          <div>
            <b-checkbox v-model="eth_other">
              Other
            </b-checkbox>
            <b-input v-model="eth_other_text" type="text" />
          </div>
        </div>
      </div>

      <div class="research-item">
        <div>
          <label>Family Income</label>
          <small>Select the range that best reflects your household yearly income</small>
        </div>
        <div>
          <b-radio v-model="family_income" native-value="zero-to-twenty-five">
            $0&mdash;$25,000
          </b-radio>
          <b-radio v-model="family_income" native-value="twenty-five-to-fifty">
            $25,001&mdash;$50,000
          </b-radio>
          <b-radio v-model="family_income" native-value="fifty-to-hundred">
            $50,001&mdash;$100,000
          </b-radio>
          <b-radio v-model="family_income" native-value="hundred-plus">
            $100,001+
          </b-radio>
        </div>
      </div>

      <div class="research-item">
        <div>
          <label>Your Education</label>
          <small>Select your highest level of attainment</small>
        </div>
        <div>
          <b-radio v-model="education_level" native-value="grade-school">
            Elementary Graduate
          </b-radio>
          <b-radio v-model="education_level" native-value="high-school">
            High School Graduate or Equivalent
          </b-radio>
          <b-radio v-model="education_level" native-value="trade-school">
            Some College
          </b-radio>
          <b-radio v-model="education_level" native-value="university">
            Bachelor's Degree
          </b-radio>
          <b-radio v-model="education_level" native-value="graduate">
            Master's Degree or Advanced Degree
          </b-radio>
          <b-radio v-model="education_level" native-value="doctorate">
            Ph. D. or M. D.
          </b-radio>
        </div>
      </div>
    </component>
  </component>
  <div class="account-ops">
    <b-button type="is-success" @click="$buefy.toast.open({message: 'Saved', type: 'is-success'})">
      Save
    </b-button>
  </div>
  <div class="delete-account">
    <button @click="confirm_delete_account">
      Delete Account
    </button>
  </div>
</div>
</template>

<script>
import debounce from 'lodash/debounce'
import Responsive from '~/assets/vars/responsive'
import ProfileItem from '~/components/ProfileItem'

function member_boolean(array_name, item_name) {
    return {
        get() {
            return this.profile[array_name].indexOf(item_name) >= 0;
        },

        set(val) {
            if(val) {
                this.profile[array_name].push(item_name);
            }
            else {
                this.profile[array_name] = this.profile[array_name].filter(x => x != item_name);
            }
            this.debounced_save();
        },
    }
}

export default {
    name: "MyProfile",

    components: {
        ProfileItem,
    },

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        let profile;

        try {
            profile = await context.$axios.$get('/api/ui/profile/', context.store.state.auth);
        }
        catch(_) {
            context.redirect({name: 'login', query: {next: 'my-profile'}});
            return;
        }

        return {
            profile
        };
    },

    data() {
        return {
            mobile: true,
            media_query: null,
            current_tab: 0,
        };
    },

    computed: {
        tabs() {
            if(this.mobile) {
                return 'b-tabs';
            }
            return 'div';
        },

        tab() {
            if(this.mobile) {
                return 'b-tab-item';
            }
            return 'section';
        },

        gender_female: member_boolean('genders', 'female'),
        gender_male: member_boolean('genders', 'male'),
        gender_nonbinary: member_boolean('genders', 'non-binary'),
        gender_trans: member_boolean('genders', 'transgender'),
        gender_inter: member_boolean('genders', 'intersex'),
        gender_other: member_boolean('genders', 'other'),

        gender_other_text: {
            get() {
                return this.profile.gender_other || '';
            },

            set(val) {
                this.profile.gender_other = val;
                this.debounced_save();
            }
        },

        birth_year: {
            get() {
                return this.profile.birth_year || '';
            },

            set(val) {
                this.profile.birth_year = val;
                this.debounced_save();
            }
        },

        eth_asian: member_boolean('ethnicities', 'asian'),
        eth_black: member_boolean('ethnicities', 'black-or-african'),
        eth_latin: member_boolean('ethnicities', 'latinx-or-hispanic'),
        eth_mideast: member_boolean('ethnicities', 'middle-eastern-or-north-african'),
        eth_americas: member_boolean('ethnicities', 'native-american-or-native-alaskan'),
        eth_pacific: member_boolean('ethnicities', 'native-hawaiian-or-pacific-islander'),
        eth_europe: member_boolean('ethnicities', 'white-or-caucasian'),
        eth_other: member_boolean('ethnicities', 'other'),

        eth_other_text: {
            get() {
                return this.profile.ethnicity_other || '';
            },

            set(val) {
                this.profile.ethnicity_other = val;
                this.debounced_save();
            }
        },

        family_income: {
            get() {
                return this.profile.family_income;
            },

            set(val) {
                this.profile.family_income = val;
                this.debounced_save();
            }
        },

        education_level: {
            get() {
                return this.profile.education_level || '';
            },

            set(val) {
                this.profile.education_level = val;
                this.debounced_save();
            }
        }
    },

    mounted() {
        this.media_query = window.matchMedia('(min-width: ' + Responsive.fullsize_min + ')');
        this.media_query.addEventListener("change", (evt) => {
            this.mobile = !evt.matches;
        });
        this.mobile = !this.media_query.matches;
    },

    methods: {
        log() {
            console.log(arguments)
        },

        debounced_save: debounce(function() { this.save(); }),

        async save() {
            await this.$axios.$put('/api/ui/profile/', this.profile, this.$store.state.auth);
        },

        confirm_delete_account() {
            this.$buefy.dialog.confirm({
                title: 'Deleting account',
                message: 'Are you sure you want to <b>delete</b> your account? This action cannot be undone.',
                confirmText: 'Delete Account',
                type: 'is-danger',
                hasIcon: true,
                onConfirm: this.delete_account
            });
        },

        async delete_account() {
            await this.$axios.$delete('/api/ui/profile/', this.$store.state.auth);
            this.$router.replace('/');
            this.$buefy.toast.open('Account deleted!');
        },
    },
}
</script>

<style lang="scss" scoped>
.my-profile {
    h1 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-largest;
        color: $snm-color-caption;
        margin: 0;
        padding: 2rem 0 1rem;
        display: none;
    }

    div.research-questions {
        h2 {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-medium-small;
            color: $snm-color-element-dark;
            margin-top:2rem;
        }

        em {
            font-family: $snm-font-content;
            font-weight: normal;
            font-size: $snm-font-small;
            color: $snm-color-tldr;
            font-style: normal;
            padding-bottom: 1rem;
            border-bottom: 1px solid $snm-color-border;
        }
        h2, em {
          padding-left: 1rem;
          padding-right: 1rem;
          display: block;
        }

        label {
            font-family: $snm-font-content;
            font-weight: normal;
            font-size: $snm-font-small;
            color: $snm-color-element-dark;
        }
    }

    .research-item {
        display: flex;
        flex-direction: column;
        border-bottom: 1px solid $snm-color-border;
        padding: 1rem;

        >div:first-child {
            display: flex;
            flex-direction: column;

            small {
                font-family: $snm-font-content;
                font-style: italic;
                font-size: $snm-font-small;
                color: $snm-color-glance;
            }
        }

        >div:last-child {
            display: flex;
            flex-direction: column;

            >* {
                margin-bottom: 1rem;
            }

            >div:last-child {
                display: flex;
            }
        }
    }

    .account-ops {
        margin: 2rem;
    }

    .delete-account {
        text-align: right;

        >button {
            margin: 2rem;
            padding: 1rem;
            color: $snm-color-info;
            border: 1px solid $snm-color-info;
            border-radius: 5px;
            cursor: pointer;
        }
    }
}

@media (min-width: $fullsize-screen) {
    .my-profile {
      h1 {
        display: block;
      }
        section.research-questions {
            border-top: 3px solid $snm-color-element-med;
            padding-top: 3rem;
        }

        .research-item {
            flex-direction: row;

            >div:first-child {
                max-width: 20rem;
                width: 10vw;
            }

            >div:last-child {
                margin-left: 2rem;
            }
        }
    }
}

#allow-comm {
  padding: 1rem;
  .label {
    margin-bottom: 0.3rem;
    display:block;
  }
  .field {
    margin-bottom: 0.3rem;
  }
  small {
    font-style: italic;
  }
}
</style>
