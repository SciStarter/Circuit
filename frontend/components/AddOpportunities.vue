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
          <h3>Out-of-School Time<br />STEM for Youth</h3>
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
              <p><span class="step">Step 1:</span> Request a provider account using the API option.</p>
              <p><span class="step">Step 2:</span> Refer to our <a href="/api/docs/v1.html">API documentation</a> to connect your opportunity data to Science Near Me using an API.</p>
              <p><span class="step">Step 3:</span> Within a week of connecting your opportunity data to Science Near Me using the API, you should receive an email from us confirming that we’ve received your opportunities and they are accurately showing up in our system and the site.</p>
            </dynamic-block>
          </div>
          <a class="action-btn" href="mailto:info@sciencenearme.org?subject=Partner Request - API">Request a Provider Account</a>
        </div>
        <div>
          <div>
            <p class="small-t">Option 2</p>
            <h3>Use Wordpress Plug-Ins</h3>
            <img src="~/assets/img/add-opps/wordpress.jpg" alt="wordpress logo" />
            <dynamic-block group="add-opportunities" item="connect-eventscalendar">
              <p>If your website is powered by self-hosted WordPress, you’ll need to install a few specific plug-ins that will share your opportunities automatically with Science Near Me.</p>
              <p><span class="step">Step 1:</span> Request a provider account using WordPress Plug-In Option.</p>
              <p><span class="step">Step 2:</span> Follow the steps outlined in the <a href="https://sciencenearme.s3.amazonaws.com/SNM-wordpress-integration.pdf" download>WordPress Guide</a> to install specific plug-ins that are compatible with Science Near Me.</p>
              <p><span class="step">Step 3:</span> Within a week of installing the recommended plug-ins, you should receive an email from us confirming that we’ve received your opportunities and they are accurately showing up in our system and the site.</p>
            </dynamic-block>
          </div>
          <a class="action-btn" href="mailto:info@sciencenearme.org?subject=Partner Request - Wordpress">Request a Provider Account</a>
        </div>
        <div>
          <div>
            <p class="small-t">Option 3</p>
            <h3>Use Our Opportunity Form</h3>
            <img src="~/assets/img/add-opps/form-icon.svg" alt="form icon" />
            <dynamic-block group="add-opportunities" item="connect-form">
              <p>If you do not yet have a system for organizing your event data, you can enter the opportunities directly into our system one by one.</p>
              <p><span class="step">Step 1:</span> Request a provider account using our Opportunity Form Option.</p>
              <p><span class="step">Step 2:</span> Log-in using your provider account information. You’ll see additional options in the left menu once your account is converted by Science Near Me staff.</p>
              <p><span class="step">Step 3:</span> Select “Submit Opportunity” to add each opportunity. You’ll see your opportunities on Science Near Me immediately.</p>
            </dynamic-block>
          </div>
          <a class="action-btn" href="mailto:info@sciencenearme.org?subject=Partner Request - Opportunity Form">Request a Provider Account</a>
        </div>
      </div>

      <div class="exchange">
        <dynamic-block group="add-opportunities" item="exchange">
          <h3>Do you lead a network where you would like partners to add opportunities through your site?</h3>
          <p>Learn about options for creating an “exchange” version of our opportunity form that you can host directly on your site. <a href="mailto:info@sciencenearme.org?subject=Exchange Info Request">Contact us</a> for more information.</p>
        </dynamic-block>
      </div>

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
  .step {
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

</style>
