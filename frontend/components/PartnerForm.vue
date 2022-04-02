<template>
<div class="organizations snm-container">
  <div class="nav-tab-wrapper">
    <ul class="nav-tabs">
      <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Organizational Settings</a></li>
      <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Page Managers</a></li>
      <li><a class="tab-link":class="{'active':state==3}" @click="state=3">Contact Info</a></li>
    </ul>
  </div>

  <div v-if="state==1" class="tab-panel">
    <profile-item v-model="partner.name" label="Organization" @input="save" />
    <profile-item v-model="partner.organization_type" label="Organization Type" :choices="org_types" @input="save" />
    <profile-item v-model="partner.under" label="Parent Organization" :auto="{url: '/api/ui/organization/exists', 'search_field': 'name', label: 'name', value: 'uid'}" :display="parent_org_name" @input="save" />
    <profile-item v-model="partner.url" label="Link" @input="save" />
    <profile-item v-model="partner.image_url" label="Logo Link" @input="save" class="no-border" />
    <img v-if="partner.image_url" :src="partner.image_url" class="display-image">
    <img v-else src="~/assets/img/no-image-thumb.jpg" class="display-image">
    <profile-item v-model="partner.background_color" color label="Background Color" @input="save" class="no-border" />
    <profile-item v-model="partner.primary_color" color label="Primary Color" @input="save" class="no-border" />
    <profile-item v-model="partner.secondary_color" color label="Secondary Color" @input="save" class="no-border" />
    <profile-item v-model="partner.tertiary_color" color label="Tertiary Color" @input="save" class="no-border" />
    <profile-item v-if="inExchange" :value="!!partner.open_submission" @input="partner.open_submission=$event; save()" label-true="Anyone can submit opportunities for approval" label-false="Only organization members can add opportunities" label="Opportunity creation" class="no-border" />
  </div><!-- state 1 -->

  <div v-if="state==2" class="tab-panel">
    <div v-if="pending.length > 0" class="pending">
      <h2>Pending Organization Page Managers</h2>
      <div v-for="(p, i) in pending" class="flex managers">
        <div class="info"><span v-if="p.first_name && p.last_name">{{p.first_name}} {{p.last_name}}</span><span v-else>{{p.username}}</span></div>
        <div class="info"><a :href="'mailto:'+p.email">{{p.email}}</a></div>
        <div class="info">{{p.phone}}</div>
        <div class="actions">
          <action-button primary @click="approve_pending(i)"><div class="icon"><check-icon /></div>Approve</action-button>
          <action-button tertiary icon-only @click="discard_pending(i)"><div class="icon"><trash-icon /></div></action-button>
        </div>
      </div>
    </div>

    <h2>Current Organization Page Managers</h2>
    <div v-for="(p, i) in managers" class="flex managers">
      <div class="info"><span v-if="p.first_name && p.last_name">{{p.first_name}} {{p.last_name}}</span><span v-else>{{p.username}}</span></div>
      <div class="info"><a :href="'mailto:'+p.email">{{p.email}}</a></div>
      <div class="info">{{p.phone}}</div>
      <div class="actions">
        <action-button v-if="p.uid != user.uid" tertiary icon-only @click="discard_authorized(i)"><div class="icon"><trash-icon /></div></action-button>
      </div>
    </div>
    <action-button primary @click="show_add=true">+ Add New Organization Manager(s)</action-button>



  </div><!-- state 2 -->

  <div v-if="state==3" class="tab-panel">
    <profile-item v-model="partner.manager.name" label="Name" @input="save" />
    <profile-item v-model="partner.manager.email" label="Email" @input="save" />
    <profile-item v-model="partner.manager.phone" label="Phone" @input="save" />
    <profile-item v-model="partner.manager.mailing" label="Address" @input="save" />
  </div><!-- state 3 -->


  <div class="global-actions">
    <action-button v-if="can_leave" tertiary red @click="leave_org">Leave organization</action-button>
  </div>

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
        <b-input v-model="emails" type="textarea" />
      </b-field>


      <div>
        <action-button primary @click="invite">Send Invitations</action-button>
        <action-button tertiary @click="show_add = false">Cancel</action-button>
      </div>

    </div>
  </b-modal>
<!--   <ul class="nav-tabs"> -->
<!--     <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Organizational Settings</a></li> -->
<!--     <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Page Managers</a></li> -->
<!--     <li><a class="tab-link":class="{'active':state==3}" @click="state=3">Contact Info</a></li> -->
<!--   </ul> -->

<!--   <div v-if="state==1" class="tab-panel"> -->
<!--     <profile-item v-model="partner.name" label="Organization" @input="save" /> -->
<!--     <profile-item v-model="partner.organization_type" label="Organization Type" :choices="org_types" @input="save" /> -->
<!--     <profile-item v-model="partner.under" label="Parent Organization" :auto="{url: '/api/ui/organization/exists', 'search_field': 'name', label: 'name', value: 'uid'}" :display="parent_org_name" @input="save" /> -->
<!--     <profile-item v-model="partner.url" label="Link" @input="save" /> -->
<!--     <profile-item v-model="partner.image_url" label="Logo Link" @input="save" class="no-border" /> -->
<!--     <img v-if="partner.image_url" :src="partner.image_url" class="display-image"> -->
<!--     <img v-else src="~/assets/img/no-image-thumb.jpg" class="display-image"> -->
<!--   </div><\!-- state 1 -\-> -->

<!--   <div v-if="state==2" class="tab-panel"> -->
<!--     <div v-if="pending.length > 0" class="pending"> -->
<!--       <h2>Pending Organization Page Managers</h2> -->
<!--       <div v-for="(p, i) in pending" class="flex managers"> -->
<!--         <div class="info"><span v-if="p.first_name && p.last_name">{{p.first_name}} {{p.last_name}}</span><span v-else>{{p.username}}</span></div> -->
<!--         <div class="info"><a :href="'mailto:'+p.email">{{p.email}}</a></div> -->
<!--         <div class="info">{{p.phone}}</div> -->
<!--         <div class="actions"> -->
<!--           <action-button primary @click="approve_pending(i)"><div class="icon"><check-icon /></div>Approve</action-button> -->
<!--           <action-button tertiary icon-only @click="discard_pending(i)"><div class="icon"><trash-icon /></div></action-button> -->
<!--         </div> -->
<!--       </div> -->
<!--     </div> -->

<!--     <h2>Current Organization Page Managers</h2> -->
<!--     <div v-for="(p, i) in managers" class="flex managers"> -->
<!--       <div class="info"><span v-if="p.first_name && p.last_name">{{p.first_name}} {{p.last_name}}</span><span v-else>{{p.username}}</span></div> -->
<!--       <div class="info"><a :href="'mailto:'+p.email">{{p.email}}</a></div> -->
<!--       <div class="info">{{p.phone}}</div> -->
<!--       <div class="actions"> -->
<!--         <action-button v-if="p.uid != user.uid" tertiary icon-only @click="discard_authorized(i)"><div class="icon"><trash-icon /></div></action-button> -->
<!--       </div> -->
<!--     </div> -->
<!--     <action-button primary @click="show_add=true">+ Add New Organization Manager(s)</action-button> -->



<!--   </div><\!-- state 2 -\-> -->

<!--   <div v-if="state==3" class="tab-panel"> -->
<!--     <profile-item v-model="partner.manager.name" label="Name" @input="save" /> -->
<!--     <profile-item v-model="partner.manager.email" label="Email" @input="save" /> -->
<!--     <profile-item v-model="partner.manager.phone" label="Phone" @input="save" /> -->
<!--     <profile-item v-model="partner.manager.mailing" label="Address" @input="save" /> -->
<!--   </div><\!-- state 3 -\-> -->


<!--   <div class="global-actions"> -->
<!--     <action-button v-if="can_leave" tertiary red @click="leave_org">Leave organization</action-button> -->
<!--   </div> -->

<!--   <b-modal -->
<!--     v-model="show_add" -->
<!--     has-modal-card -->
<!--     trap-focus -->
<!--     :destroy-on-hide="false" -->
<!--     aria-role="dialog" -->
<!--     aria-label="Show tooltip" -->
<!--     aria-modal -->
<!--     class="form-modal" -->
<!--     > -->
<!--     <div class="card"> -->
<!--       <h2>Add Organizational Managers <span class="close" @click="show_add = false">&times;</span></h2> -->
<!--       <p>Organization managers will be able to edit organizational settings and opportunity records. Each email will receive an email link.</p> -->
<!--       <p class="help">Add Emails of Additional Managers, One per Line</p> -->
<!--       <b-field> -->
<!--         <b-input v-model="emails" type="textarea" /> -->
<!--       </b-field> -->


<!--       <div> -->
<!--         <action-button primary @click="invite">Send Invitations</action-button> -->
<!--         <action-button tertiary @click="show_add = false">Cancel</action-button> -->
<!--       </div> -->

<!--     </div> -->
  <!-- </b-modal> -->

</div>
</template>

<script>
import CheckIcon from "~/assets/img/check.svg?inline"
import TrashIcon from '~/assets/img/trash.svg?inline'

export default {
    name: "PartnerForm",

    components:{
        CheckIcon,
        TrashIcon
    },

    props: {
        partner: {
            type: Object,
            required: true,
        },

        org_types: {
            type: Array,
            required: true,
        },

        managers: {
            type: Array,
            required: true,
        },

        pending: {
            type: Array,
            required: true,
        },

        inExchange: {
            type: Boolean,
            required: false,
            default: false,
        },
    },

    data() {
        return {
            state: 1,
            show_add: false,
            parent_org_name: '',
            emails: '',
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },

        can_leave() {
            return this.user.uid != this.partner.prime;
        },
    },

    watch: {
        'partner.under': async function(val, old) {
            let ref = await this.$axios.$post('/api/ui/organization/exists', {uid: val});
            if(ref.length) {
                this.parent_org_name = ref[0].name;
            }
            else {
                this.parent_org_name = '';
            }
        },

        'partner.uid': async function(val, old) {
            this.managers = await this.$axios.$get('/api/ui/organization/' + val + '/managers', this.$store.state.auth);
            this.pending = await this.$axios.$get('/api/ui/organization/' + val + '/pending-managers', this.$store.state.auth);
        },
    },

    methods: {
        async invite() {
            let emails = this.emails.split(/[ \t\r\n,]+/g);

            this.show_add = false;
            this.emails = '';

            await this.$axios.$post('/api/ui/organization/' + this.partner.uid + '/invite', {emails}, this.$store.state.auth);

            this.$buefy.toast.open('Invitations sent');
        },

        async approve_pending(idx) {
            let entry = this.pending[idx];
            this.pending.splice(idx, 1);
            this.managers.push(entry);
            this.partner.pending = this.partner.pending.filter(x => x != entry.uid);
            this.partner.authorized.push(entry.uid);
            await this.save();
        },

        async discard_pending(idx) {
            let entry = this.pending[idx];
            this.pending.splice(idx, 1);
            this.partner.pending = this.partner.pending.filter(x => x != entry.uid);
            await this.save();
        },

        async discard_authorized(idx) {
            let entry = this.managers[idx];
            this.managers.splice(idx, 1);
            this.partner.authorized = this.partner.authorized.filter(x => x != entry.uid);
            await this.save();
        },

        async leave_org() {
            let {result} = await this.$buefy.dialog.confirm({
                message: 'Are you sure you want to give up your authority to manage ' + this.partner.name + ' on Science Near Me?',
            });

            if(result) {
                this.partner.authorized = this.partner.authorized.filter(x => x != this.user.uid);
                await this.save();
                this.$router.replace('/my/profile');
                this.$buefy.toast.open('Exited organization');
            }
        },

        async save() {
            await this.$axios.$put('/api/ui/organization/' + this.partner.uid, this.partner, this.$store.state.auth);
        },
    }
}
</script>

<style lang="scss" scoped>
.organizations{
    position: relative;

  .paragraph {
    padding: 2rem;
  }

  h1 {
    font-family: $snm-font-heading;
    font-size: 1.8rem;
    font-weight:bold;
    color: var(--secondary-color, $snm-color-element-med);
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
    margin-left:5px;
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
      color: var(--secondary-color, $snm-color-element-med);
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

  .global-actions {
      display: flex;
      flex-direction: column;
  }

  @media (min-width: 600px) {
      .global-actions {
          position: absolute;
          width: 15rem;
          top: 0px;
          right: 0px;
      }

      .display-image {
          margin-left:175px;
      }
  }
}
</style>
