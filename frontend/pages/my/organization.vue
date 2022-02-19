<template>
  <div class="organizations snm-container">
      <h1>Your Organization</h1>

      <ul class="nav-tabs">
          <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Organizational Settings</a></li>
          <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Page Managers</a></li>
          <li><a class="tab-link":class="{'active':state==3}" @click="state=3">Your Contact Info</a></li>
      </ul>

      <div v-if="state==1" class="tab-panel">
        <profile-item v-model="organization.name" label="Organization" @input="save" />
        <profile-item v-model="organization.kind" label="Organization Type" @input="save" />
        <profile-item v-model="organization.address" label="Address" @input="save" />
        <profile-item v-model="organization.parent" label="Parent Organization" @input="save" />
        <profile-item v-model="organization.logo" label="Logo Link" @input="save" class="no-border" />
        <img v-if="organization.logo" :src="organization.logo" class="display-image" />
        <img v-else src="~/assets/img/no-image-thumb.jpg" class="display-image" />
      </div><!-- state 1 -->

      <div v-if="state==2" class="tab-panel">
        <div v-if="pending.length > 0" class="pending">
          <h2>Pending Organization Managers</h2>
          <div v-for="p in pending" class="flex managers">
              <div class="info">{{p.name}}</div>
              <div class="info"><a :href="'mailto:'+p.email">{{p.email}}</a></div>
              <div class="info">{{p.phone}}</div>
              <div class="actions">
                <action-button primary><div class="icon"><check-icon /></div>Approve</action-button>
                <action-button tertiary icon-only><div class="icon"><trash-icon /></div></action-button>
              </div>
          </div>
        </div>

          <h2>Current Organization Managers</h2>
          <div v-for="p in managers" class="flex managers">
              <div class="info">{{p.name}}</div>
              <div class="info"><a :href="'mailto:'+p.email">{{p.email}}</a></div>
              <div class="info">{{p.phone}}</div>
              <div class="actions">
                <action-button tertiary icon-only><div class="icon"><trash-icon /></div></action-button>
              </div>
          </div>
          <action-button primary @click="show_add=true">+ Add New Organization Manager(s)</action-button>



      </div><!-- state 2 -->

      <div v-if="state==3" class="tab-panel">
        <profile-item v-model="profile.first_name" label="First Name" @input="save" />
        <profile-item v-model="profile.last_name" label="Last Name" @input="save" />
        <profile-item v-model="profile.email" label="Email" @input="save" />
        <profile-item v-model="profile.phone" label="Phone" @input="save" />
        <action-button tertiary red>Leave organization</action-button>
      </div><!-- state 3 -->


      <b-modal
        v-model="show_add"
        has-modal-card
        trap-focus
        :destroy-on-hide="false"
        aria-role="dialog"
        aria-label="Show tooltip"
        aria-modal
        class="form-modal"
        >
        <div class="card">
          <h2>Add Organizational Managers <span class="close" @click="show_add = false">&times;</span></h2>
          <p>Organization managers will be able to edit organizational settings and opportunity records. Each email will receive an email link.</p>
          <p class="help">Add Emails of Additional Managers, One per Line</p>
          <b-field>
              <b-input type="textarea" />
          </b-field>


          <div>
              <action-button primary>Send Invitations</action-button>
              <action-button tertiary @click="show_add = false">Cancel</action-button>
          </div>

        </div>
      </b-modal>

  </div>
</template>

<script>
import CheckIcon from "~/assets/img/check.svg?inline"
import TrashIcon from '~/assets/img/trash.svg?inline'
export default {
  components:{
    CheckIcon,
    TrashIcon
  },
  data(){
    return {
      state: 1,
      show_add: false,
      profile: {"birth_year":null,"education_level":null,"email":"kevripka@gmail.com","ethnicities":[],"ethnicity_other":null,"family_income":null,"first_name":null,"gender_other":null,"genders":["male"],"image_url":null,"last_name":null,"opt_in_research":null,"opt_in_volunteer":null,"password":null,"phone":null,"private":false,"username":"System","whatsapp":null,"zip_code":null},
      organization:{
        name: 'Nerd Nite',
        kind: 'Club',
        address1: '123 Main St.<br />Iowa City, IA 52245',
        parent: null,
        logo: 'https://picsum.photos/300/200'
      },
      pending:[
        {name:'Lisa Turtle', email: 'turts@bayside.edu',phone:'213.123.1223'},
        {name:'Zach Morris', email: 'hair@bayside.edu',phone:'213.123.1223'},
        {name:'Samuel Powers', email: 'screech@bayside.edu',phone:'213.123.1223'}
      ],
      managers:[
        {name:'A.C. Slater', email: 'jock@bayside.edu',phone:'213.123.1223'},
        {name:'Kelly Kapowski', email: 'cheer@bayside.edu',phone:'213.123.1223'},
        {name:'Jesse Spano', email: 'pillpopper@bayside.edu',phone:'213.123.1223'}
      ]
    }
  }
}

// export default {
//     httpHeaders() {
//         return {
//             'X-XSS-Protection': '1; mode=block',
//             'X-Frame-Options': 'DENY',
//             'X-Content-Type-Options': 'nosniff',
//             'Referrer-Policy': 'same-origin',
//         };
//     },
//
//     async asyncData(context) {
//         const user = await context.store.dispatch('get_user');
//
//         if(!user.authenticated) {
//             context.error({
//                 statusCode: 401,
//                 message: "Authentication required"
//             });
//
//             return;
//         }
//
//         let partners = [];
//
//         try {
//             partners = await context.$axios.$get('/api/ui/profile/partners', this.$store.state.auth);
//         }
//         catch(err) {
//             context.error({
//                 statusCode: err.response.status,
//                 message: err.response.data
//             });
//         }
//
//         return {
//             partners,
//         }
//     },
//
//     data() {
//         return {
//             partner_index: 0,
//         }
//     },
//
//     computed: {
//         user() {
//             return this.$store.state.user;
//         },
//
//         choose_partner() {
//             return this.partners.length > 1;
//         },
//
//         selected_partner() {
//             return this.partners[this.partner_index] || null;
//         },
//     },
// }
</script>

<style lang="scss" scoped>
.organizations{
  h1 {
    font-family: $snm-font-heading;
    font-size: 1.8rem;
    font-weight:bold;
    color: $snm-color-element-med;
    margin-bottom:2rem;
  }
  .display-image {
    height: auto;
    margin: 0 1rem 1rem 0;
    border: 1px solid #d9d9d9;
    max-width: 200px;
    border-radius: 6px;
    -o-object-fit: contain;
    object-fit: contain;
    max-height: 180px;
    min-width:300px;
    margin-left:175px;
  }
  .managers {
      align-items:center;
      border-bottom:1px solid $snm-color-border;

    .info {
      width: calc(33.33% - 100px);
    }
    .actions {
      margin-left:auto;
      display:flex;
      align-items:center;
    }
  }
  h2 {
      text-transform: uppercase;
      color: $snm-color-element-med;
      font-weight:bold;
      font-size:14px;
  }
  .trash.icon {
    padding-right:0;
  }
  .pending {
    margin-bottom:2rem;
  }

  .form-modal {
    h2 {
      color: $snm-color-background-meddark;
      font-weight:bold;
      font-size:18px;
      font-family: $snm-font-heading;
      display:flex;
      justify-content:space-between;
      align-items:center;
      span {
        font-size: 44px;
        display: block;
        line-height: 1;
        font-weight: normal;
        cursor:pointer;
      }
    }
  }

}
</style>
