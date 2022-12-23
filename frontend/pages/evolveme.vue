<template>
    <div id="evolve">
        
        <div class="snm-wrapper">
            <div class="snm-container">

                <div class="logo-lockup">
                    <img src="~assets/img/evolve-me/evolveme-logo.svg" id="evolve-logo" />
                    <nuxt-link to="/" class="logo" data-context="Science Near Me logo">
                        <img src="~assets/img/logo.svg" title="return to home page" />
                    </nuxt-link>
                </div>


                <div v-if="state > 1">
                    <ul class="steps">
                        <li class="step" :class="{'current':state==2,'past':state>2}">
                          <div v-if="state>2" class="checkmark"><img src="~assets/img/evolve-me/checkmark.svg" /></div>
                          <div>
                            <strong>Step One</strong>
                            <small>Create a Science Near Me Account</small>
                          </div>
                        </li>
                        <li class="step" :class="{'current':state==3,'past':state>3}">
                          <div v-if="state>3" class="checkmark"><img src="~assets/img/evolve-me/checkmark.svg" /></div>
                          <div>
                            <strong>Step Two</strong>
                            <small>Search for a science opportunity near you</small>
                          </div>
                        </li>
                        <li class="step" :class="{'current':state==4}">
                          <div>
                            <strong>Step Three</strong>
                            <small>View the science opportunities near you!</small>
                          </div>
                        </li>
                    </ul>
                </div>


                <div class="content-card" v-if="state==1">
                    <div class="center">
                        <h1>Welcome to Science Near Me</h1>
                        <p>Science Near Me has partnered with EvolveMe to help you complete tasks, earn points, and build skills in discovering and connecting with opportunities to engage with science near you!</p>
                        <action-button principal @click="begin()">Begin by Creating Your Account</action-button>
                    </div>
                </div>

                <div class="content-card" v-if="state==2">
                    <div class="center">
                        <h1>Create Your Account</h1>
                        <div id="evolve-signup"><signup-form partner="evolveme" :triggerSuccess="completeAccountCreation" /></div>
                    </div>
                </div>

                <div class="content-card" v-if="state==3">
                    <div class="center">
                        <h1>Find Science Near You!</h1>
                        <p>Use the search bar below. You may want to allow your browser to find your location.</p>
                        <general-filters
                            :text="search_text"
                            :place="search_place"
                            :beginning="search_beginning"
                            :ending="search_ending"
                            :include-online="search_online"
                            search-button
                            @text="search_text=$event"
                            @place="search_place=$event"
                            @beginning="search_beginning=$event"
                            @ending="search_ending=$event"
                            @include-online="search_online=$event"
                            />
                        <button @click="state=4">go to step 4</button>
                    </div>
                </div>

                <div class="content-card" v-if="state==4">
                    <div class="center">
                        <h1>Congratulations &mdash; You did it! You've earned points in EvolveMe</h1>
                        <h2>You've Found Science Near You and Completed Your EvolveMe Task!</h2>
                        <p>You can look through the list of science near you below or you can visit your <a href="/">Science Near Me Dashboard</a>to save and find more projects.</p>
                    </div>
                </div>


            </div>
        </div>



        <Footer />
        <SubFooter />

    </div>
</template>

<script>
import Footer from "~/components/Footer"
import SubFooter from "~/components/SubFooter"
import ActionButton from "~/components/ActionButton"
import SignupForm from "~/components/SignupForm"
import GeneralFilters from "~/components/GeneralFilters"
export default {
  layout: "empty",
  components: {
    Footer,
    SubFooter,
    ActionButton,
    SignupForm,
    GeneralFilters
  },
  data(){
    return {
        state:1,
    }
  },
  methods: {

    begin(){
        this.state = 2;
    },
    completeStepOne(){

    },
    completeStepTwo(){

    },
    completeStepThree(){

    },
    completeAccountCreation(){
        this.state = 3;
    }

  }
};
</script>

<style lang="scss" scoped>

@import url("https://use.typekit.net/tew6huf.css");

@mixin proxima {
    font-family: proxima-nova,sans-serif;
    font-weight: 400;
    font-style: normal;
}

@mixin proxima-italic {
    font-family: proxima-nova,sans-serif;
    font-weight: 400;
    font-style: italic;
}

@mixin proxima-bold {
    font-family: proxima-nova,sans-serif;
    font-weight: 700;
    font-style: normal;
}

@mixin proxima-extrabold {
    font-family: proxima-nova,sans-serif;
    font-weight: 800;
    font-style: normal;
}

$purple: #8E51F0;
$dark_purple: #6630BC;
$light_purple: #DFD1F7;
$dark: #09163B;
$yellow: #F4B033;
$tan: #F2F1EF;


#evolve {
    border-top: 10px solid $purple;
    background-color: $tan;
}

  ::v-deep(footer) {
    background-color: $dark_purple;
  }
  ::v-deep(.subfooter) {
    background-color:$dark;
  }

  .logo-lockup {
    margin: 2rem auto 4rem;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  #evolve-logo {
    width: 180px;
  }
  .logo {
    width: 140px;
    padding-left: 1rem;
    border-left: 3px solid $dark;
    margin-left: 1rem;
    position: relative;
    top:7px;
    img {
          position: relative;
          top:3px;
          display: block;
        }
  }

  @media (max-width:600px) {
    #evolve-logo {
        width: 100px;
    }
    .logo {
        width: 90px;
        img {
          top:0;
        }
    }
    .logo-lockup {
      margin-top: 1rem;
      margin-bottom: 2.5rem;
    }
  }

  .content-card {
    background-color: #fff;
    box-shadow: 0 1px 15px rgba(0,0,0,.1);
    border-radius: 8px;
    max-width: 1075px;
    margin:2rem auto;
    padding: 1rem;

    h1 {
        color: $purple;
        @include proxima-extrabold;
        font-size: 2rem;
    }

    p {
        @include proxima;
        max-width: 780px;
        margin:0 auto 1rem;
    }
  }
  .center {
    text-align: center;
  }

  #evolve-signup {
    max-width: 600px;
    text-align: left;
    margin: 0 auto;

    ::v-deep(a) {
      color: $purple;
      text-decoration: underline;
    }
  }

    .steps {
        display:flex;
        max-width: 1075px;
        margin:4rem auto 2rem;
        height: 64px;

        div {
          z-index: 99;
        }

      strong, small {
        display: block;
        line-height: 1.2;
        z-index: 99;
        color: #fff;
      }

      small {
        font-size: 12px;
      }

      .step {
        width:33.3%;
        padding:.5rem;
        position: relative;
        background-color: $light_purple;
        z-index: 2;
        margin-left: 10px;
        padding-left: 36px;
        display: flex;
        align-items: center;

        &.current {
          background-color: $purple;
          &:after {
            background-color: $purple;
          }
        }
        &.past {
          background-color: $dark;
          &:after {
            background-color: $dark;
          }
        }

        &:nth-child(1) {
          z-index: 99;
          margin-left: 0;
          border-top-left-radius: 8px;
          border-bottom-left-radius: 8px;
          padding-left: 12px;
          &:before {
            display: none;
          }
        }
        &:nth-child(2) {
          z-index: 98;
        }
        &:nth-child(3) {
          border-top-right-radius: 8px;
          border-bottom-right-radius: 8px;
          &:after {
            display: none;
          }
        }

        &:before {
          content: " ";
          // position: absolute;
          // top: 0;
          // width: 0;
          // height: 0;
          // border-top: 26px solid transparent;
          // border-bottom: 26px solid transparent;
          // border-left: 26px solid $light_purple;
          // z-index: 2;
          // right: auto;
          // left: 0;
          // border-left: 17px solid $tan;
          // z-index: 0;
          content: " ";
          position: absolute;
          top: 9px;
          left: -28px;
          width: 48px;
          height: 48px;
          transform: rotate(47deg);
          z-index: 2;
          background: $tan;
          border-radius: 4px;
          z-index: 1;
        }

        &:after {
          // content: " ";
          // position: absolute;
          // top: 0;
          // right: -26px;
          // width:52px;
          // height: 52px;
          // transform: rotate(90deg);
          // width: 0;
          // height: 0;
          // border-top: 26px solid transparent;
          // border-bottom: 26px solid transparent;
          // border-left: 26px solid $light_purple;
          // z-index: 2;
          content: " ";
          position: absolute;
          top: 8.5px;
          right: -23px;
          width: 48px;
          height: 47px;
          transform: rotate(47deg);
          z-index: 2;
          background: $light_purple;
          border-radius: 4px;
          z-index: 1;
        }

      }

    }

    .checkmark {
      display: block;
      width:30px;
      height: 30px;
      background-color: $yellow;
      border-radius: 100%;
      margin-right: 10px;
      flex-shrink: 0;
      img {
        width: 20px;
        position: relative;
        top: 6px;
        left: 4px;
      }
    }

    @media (max-width:600px) {
      .steps {
        height: 32px;
        margin-top:3rem;
        .step {
          height: 32px;
          padding-left: 20px;
          strong {
            font-size: 14px;
          }
          small {
            display: none;
          }
          &:before {
            top: 4px;
            left: -16px;
            width: 24px;
            height: 26px;
          }
          &:after {
            top: 4px;
            right: -12px;
            width: 25px;
            height: 24px;
          }
        }
      }
      .checkmark {
        width:20px;
        height: 20px;
        margin-right: 5px;
        img {
          width: 12px;
           top: -2px;
        }
      }
    }


::v-deep(.general-filters) {
  background-color: transparent;
  padding-top:0;
  .basic-filter-backdrop form {
    background-color: $purple;
  }
}

@media (max-width:959px) {
  ::v-deep(.general-filters) {
    padding:1rem;
    background-color: $purple;
    border-radius: 8px;
    .centered-row {
      justify-content: flex-start;
      > div {
        width:100%;
      }
      > div:first-child {
        margin-right: 1rem;
      }
    }
  }
}

@media (max-width:488px) {
  ::v-deep(.general-filters) {
    .centered-row {
      flex-direction: column;
    }
  }
}
  
</style>