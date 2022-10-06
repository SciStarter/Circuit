<template>
<div class="snm-wrapper">
  <div class="snm-container base-typography snm-container-first">
      <header>
        <dynamic-block group="add-opportunities" item="header">
          <h1><span><add-icon /></span>Add Your Opportunities to Science Near Me</h1>
          <p>Do you want to draw a larger or more targeted audience for your STEM events, programs, and projects?</p>
          <p class="i">Add your opportunities to Science Near Me!</p>
        </dynamic-block>
      </header>

      <div class="step-header">
          <step-one />
          <dynamic-block group="add-opportunities" item="step-one">
            <h2 class="h3">Check to see if your opportunities fit one of our partner organizations.</h2>
          </dynamic-block>
      </div>

      <div class="flex-row">
        <div>
          <p class="small-t">If your opportunities are</p>
          <h3>Citizen Science<br />Projects &amp; Events</h3>
          <img src="~/assets/img/add-opps/scistarter.jpg" alt="scistarter logo" />
          <dynamic-block group="add-opportunities" item="scistarter">
            <p>If you provide citizen science projects and events, or crowdsourcing opportunities, visit <a href="https://scistarter.org/">SciStarter</a> to add your opportunities there.</p>
            <p>Your opportunities will automatically show up in Science Near Me through our partnership with SciStarter.</p>
          </dynamic-block>
        </div>
        <div>
          <p class="small-t">If your opportunities are</p>
          <h3>Girl-Serving<br>Out-of-School Time STEM</h3>
          <img src="~/assets/img/add-opps/connectory.jpg" alt="connectory logo" />
          <dynamic-block group="add-opportunities" item="connectory">
            <p>If you provide opportunities for children and teens to explore STEM in out-of-school time settings, visit <a href="https://theconnectory.org/">The Connectory</a> to add your opportunities there.</p>
            <p>Your opportunities will automatically show up in Science Near Me through our partnership with The Connectory.</p>
          </dynamic-block>
        </div>
        <div>
          <p class="small-t">If your organization is</p>
          <h3>Part of a<br />STEM Network</h3>
          <img src="~/assets/img/add-opps/stem.jpg" alt="STEM" />
          <dynamic-block group="add-opportunities" item="stem-network">
            <p>If you are connected with an existing STEM Network like Nation of Makers or STEM Ecosystem, check our <a href="https://sciencenearme.org/about#our-partners">list of partners</a> to see if you have a more local way of sharing data that will ultimately feed into Science Near Me through their involvement.</p>
          </dynamic-block>
        </div>
      </div>

      <div class="step-header">
          <step-two />
          <dynamic-block group="add-opportunities" item="step-two">
            <h2 class="h3">Choose an option for sharing your opportunity data with Science Near Me.</h2>
            <p>If there is no partner organization that works for you, use one of the options below.</p>
          </dynamic-block>
      </div>

      <div class="flex-row flex-row2">
        <div>
          <div>
            <p class="small-t">Option 1</p>
            <h3>Connect with an API</h3>
            <img src="~/assets/img/add-opps/api.svg" alt="API icon" />
            <dynamic-block group="add-opportunities" item="connect-api">
              <p>If you have your opportunities stored in a database, connecting to our API is the optimal option to maintain ongoing sharing of data with Science Near Me. Every change you make in your database will show up automatically in Science Near Me.</p>
              <p><span class="step">Step 1:</span> Request a partner account using the API option.</p>
              <p><span class="step">Step 2:</span> Refer to our <a href="/api/docs/v1.html">API documentation</a> to connect your opportunity data to Science Near Me using an API.</p>
              <p><span class="step">Step 3:</span> Within a week of connecting your opportunity data to Science Near Me using the API, you should receive an email from us confirming that we've received your opportunities and they are accurately showing up in our system and the site.</p>
            </dynamic-block>
          </div>
          <a class="action-btn" @click="request.access='API';show_form();">Request a Partner Account</a>
        </div>
        <div>
          <div>
            <p class="small-t">Option 2</p>
            <h3>Use Wordpress Plug-Ins</h3>
            <img src="~/assets/img/add-opps/wordpress.jpg" alt="wordpress logo" />
            <dynamic-block group="add-opportunities" item="connect-eventscalendar">
              <p>If your website is powered by self-hosted WordPress, you’ll need to install a few specific plug-ins that will share your opportunities automatically with Science Near Me.</p>
              <p><span class="step">Step 1:</span> Request a partner account using WordPress Plug-In Option.</p>
              <p><span class="step">Step 2:</span> Follow the steps outlined in the <a href="https://sciencenearme.s3.amazonaws.com/SNM-wordpress-integration.pdf" download>WordPress Guide</a> to install specific plug-ins that are compatible with Science Near Me.</p>
              <p><span class="step">Step 3:</span> Within a week of installing the recommended plug-ins, you should receive an email from us confirming that we’ve received your opportunities and they are accurately showing up in our system and the site.</p>
            </dynamic-block>
          </div>
          <a class="action-btn" @click="request.access='Wordpress';show_form();">Request a Partner Account</a>
        </div>
        <div>
          <div>
            <p class="small-t">Option 3</p>
            <h3>Use Our Opportunity Form</h3>
            <img src="~/assets/img/add-opps/form-icon.svg" alt="form icon" />
            <dynamic-block group="add-opportunities" item="connect-form">
              <p>If you do not yet have a system for organizing your event data, you can enter the opportunities directly into our system one by one.</p>
              <p><span class="step">Step 1:</span> Request a partner account using our Opportunity Form Option.</p>
              <p><span class="step">Step 2:</span> Log-in using your partner account information. You’ll see additional options in the left menu once your account is converted by Science Near Me staff.</p>
              <p><span class="step">Step 3:</span> Select “Submit Opportunity” to add each opportunity. You’ll see your opportunities on Science Near Me immediately.</p>
            </dynamic-block>
          </div>
          <a class="action-btn" @click="request.access='Form';show_form();">Request a Partner Account</a>
        </div>
      </div>

      <div class="exchange">
        <dynamic-block group="add-opportunities" item="exchange">
          <h3>Do you lead a network where you would like partners to add opportunities through your site?</h3>
          <p>Learn about options for creating an “exchange” version of our opportunity form that you can host directly on your site <a href="/add-opportunities">here</a>. <a href="mailto:info@sciencenearme.org?subject=Exchange Info Request">Contact us</a> for more information.</p>
        </dynamic-block>
      </div>


      <b-modal
        v-model="show_request"
        has-modal-card
        trap-focus
        :destroy-on-hide="false"
        aria-role="dialog"
        aria-label="Request Partner Account"
        aria-modal
        >
        <div class="card flex-col">
          <div v-if="submitted">
              <h2>Thank you for your request! <span class="close" @click="show_request = false">&times;</span></h2>
              <p>The Science Near Me team will contact you regarding your request within 72 hours.</p>
          </div>
          <template v-else>
            <h2>Request Partner Account <span class="close" @click="show_request = false">&times;</span></h2>
            <p>Please provide the following information:</p>
            <b-field label="Your Organization">
              <b-input v-model="request.partner"></b-input>
            </b-field>
            <b-field label="Type of Access Requested">
              <b-select placeholder="Select" v-model="request.access">
                <option value="API">API</option>
                <option value="Wordpress">Wordpress</option>
                <option value="Form">Opportunity Form</option>
              </b-select>
            </b-field>
            <b-field label="Contact Name">
              <b-input v-model="request.name"></b-input>
            </b-field>
            <b-field label="Contact Email">
              <b-input v-model="request.email"></b-input>
            </b-field>
            <b-field label="Contact Phone Number">
              <b-input v-model="request.phone"></b-input>
            </b-field>
            <b-field label="Contact Address">
            </b-field>

            <label style="display:block;margin-top:-10px;">Street Address</label>
            <b-input v-model="request.address"></b-input>

            <div class="flex">
              <div>
                <label style="display:block;">City</label>
                <b-input v-model="request.city"></b-input>
              </div>
              <div class="states">
                <label style="display:block;">State</label>
                <b-select v-model="request.state">
                  <option v-for="s in states" :value="s" :key="s">{{s}}</option>
                </b-select>
              </div>
            </div>
            <b-field label="Organization Website">
              <b-input v-model="request.website"></b-input>
            </b-field>
            <b-field label="Tell us about the types of opportunities you offer.">
              <b-input type="textarea" v-model="request.about"></b-input>
            </b-field>
            <b-field label="How many opportunities do you expect to enter on an average year?">
              <b-numberinput controls-position="compact" :min="1" v-model="request.number"></b-numberinput>
            </b-field>
            <action-button principal @click="submit" type="submit">Submit</action-button>
          </template>
        </div>
      </b-modal>

  </div>
</div>
</template>

<script>
import AddIcon from '~/assets/img/add-opps/add-opp-icon.svg?inline'
import StepOne from '~/assets/img/add-opps/step-1.svg?inline'
import StepTwo from '~/assets/img/add-opps/step-2.svg?inline'

export default {
    name: "AddOpportunities",

    components: {
        AddIcon,
        StepOne,
        StepTwo
    },

    data() {
        return {
            show_request: false,
            submitted:false,
            request: {
                partner: "",
                name: "",
                email: "",
                phone: "",
                address: "",
                city: "",
                state: "",
                about: "",
                number: 10,
                access: "",
                website: "",
            },
            states: ['Alabama','Alaska','American Samoa','Arizona','Arkansas','California','Colorado','Connecticut','Delaware','District of Columbia','Federated States of Micronesia','Florida','Georgia','Guam','Hawaii','Idaho','Illinois','Indiana','Iowa','Kansas','Kentucky','Louisiana','Maine','Marshall Islands','Maryland','Massachusetts','Michigan','Minnesota','Mississippi','Missouri','Montana','Nebraska','Nevada','New Hampshire','New Jersey','New Mexico','New York','North Carolina','North Dakota','Northern Mariana Islands','Ohio','Oklahoma','Oregon','Palau','Pennsylvania','Puerto Rico','Rhode Island','South Carolina','South Dakota','Tennessee','Texas','Utah','Vermont','Virgin Island','Virginia','Washington','West Virginia','Wisconsin','Wyoming']
        }
    },

    methods: {
        show_form() {
            if(this.$store.state.user.authenticated) {
                this.show_request = true;
            }
            else {
                this.$emit('signup');
                setTimeout(this.logged_in_yet, 3000);
            }
        },

        logged_in_yet() {
            if(this.$store.state.user.authenticated) {
                this.show_request = true;
            }
            else {
                setTimeout(this.logged_in_yet, 1000);
            }
        },

        async submit() {
            try {
                await this.$axios.$post("/api/ui/organization/", this.request, this.$store.state.auth);
                this.submitted = true;
            }
            catch(x) {
                alert("Error while submitting request: " + x);
            }
        }
    },
}
</script>

<style lang="scss" scoped>
.base-typography header {
  text-align:center;
  h1 {
    display: flex;
    justify-content:center;
    align-items:center;
    margin-bottom:-30px;
    span {
      margin-right:10px;
    }
  }
  p {
    font-weight: bold;
    &.i {
      font-style:italic;
      margin-bottom:2rem;
    }
  }
}
.base-typography .step-header {
  text-align:center;
  background-color: $snm-color-background-medlight;
  border-radius:6px;
  margin-top:80px;
  padding-bottom: 1rem;
  margin-bottom:1rem;
  svg {
    width:200px;
    margin-top:-50px
  }
  h2 {
    color: #1D1D1D;
  }
}

.flex-row {
  display:flex;
  gap: 20px;
  > div {
    border:1px solid $snm-color-border;
    border-radius:6px;
    width: 33.33%;
    padding:1rem;
  }
  h3, .small-t {
    text-align:center;
  }
  .small-t {
    text-transform: uppercase;
    color: $snm-color-background-medium;
    font-size:12px;
    margin-bottom:0;
    font-weight:bold;
  }
  img {
    max-width:80%;
    object-fit:contain;
    border:1px solid $snm-color-border;
    margin:0 auto;
    display: block;
    height:80px;
    padding:10px 20px;
    margin-bottom:1rem;
  }
  .small-t {
    color: $snm-color-info;
  }
}

.flex-row2 {
  > div {
    display:flex;
    flex-direction:column;
    > div {
      flex-grow:1;
    }
  }
  img {
    border:0;
    height: 120px;
    padding:0;
  }
  :deep(.step) {
    font-weight:bold;
  }
  .action-btn {
    background-color: $snm-color-background-meddark;
    color: #fff;
    font-weight: bold;
    padding: .5rem 1rem;
    border-radius:6px;
    text-align:center;
    display:inline-block;
    margin:0 auto;
    line-height:1;
    border:1px solid $snm-color-background-dark;
    &:hover {
      background-color: $snm-color-background-dark;
    }
  }
}

.exchange {
  text-align:center;
  padding:4rem 2rem;
}

@media (max-width:1399px) {
  .base-typography header {
    h1 {
      font-size: 2rem;
      svg {
        width:50px;
      }
    }
  }
}

@media (max-width:767px) {
  .base-typography header {
    h1 {
      font-size: 1.6rem;
    }
  }
}

@media (max-width:699px){
  .flex-row {
    flex-direction:column;
    > div {
      width: 100%;
    }
  }
}

@media (max-width:599px) {
  .base-typography header {
    h1 {
      flex-direction:column;
      margin-bottom:.25rem;
      svg {
        height:60px;
      }
    }
  }
}

.modal {
    .card {
      max-height:90vh;
      overflow:auto;
      .flex {
        margin-top:10px;
        margin-bottom:1rem;
        >div {
          width:100%;
        }
          .states {
              max-width:33%;
              margin-left:20px;
          }
      }
        form button {
        margin-left:0;
      }
        h2 {
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-medium;
            line-height: 26px;
            letter-spacing: 0px;
            color: $snm-color-background-dark;

            .close {
                float: right;
                font-size: $snm-font-larger;
                position: relative;
                top: -10px;
                cursor: pointer;
            }
        }

        p {
            font-family: $snm-font-content;
            font-weight: normal;
            font-size: $snm-font-small;
            line-height: 22px;
            letter-spacing: 0px;
            color: $snm-color-tldr;
            margin: 1rem 0px;
        }


    }
}

</style>
