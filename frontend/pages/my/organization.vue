<template>
<div v-if="selected_partner.uid" class="organizations snm-container">
  <p v-if="choose_partner">
    <b-select v-model="partner_index" size="is-large" aria-role="list">
      <option v-for="(partner, idx) in partners" :key="partner.uid" :value="idx" aria-role="listitem">{{partner.name}}</option>
    </b-select>
    <br>
  </p>
  <h1 v-else>Your Organization</h1>

  <ul class="nav-tabs">
    <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Organizational Settings</a></li>
    <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Page Managers</a></li>
    <li><a class="tab-link":class="{'active':state==3}" @click="state=3">Contact Info</a></li>
  </ul>

  <div v-if="state==1" class="tab-panel">
    <profile-item v-model="selected_partner.name" label="Organization" @input="save" />
    <profile-item v-model="selected_partner.organization_type" label="Organization Type" :choices="org_types" @input="save" />
    <profile-item v-model="selected_partner.under" label="Parent Organization" :auto="{url: '/api/ui/organization/exists', 'search_field': 'name', label: 'name', value: 'uid'}" :display="parent_org_name" @input="save" />
    <profile-item v-model="selected_partner.url" label="Link" @input="save" />
    <profile-item v-model="selected_partner.image_url" label="Logo Link" @input="save" class="no-border" />
    <img v-if="selected_partner.image_url" :src="selected_partner.image_url" class="display-image">
    <img v-else src="~/assets/img/no-image-thumb.jpg" class="display-image">
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
    <profile-item v-model="selected_partner.manager.name" label="Name" @input="save" />
    <profile-item v-model="selected_partner.manager.email" label="Email" @input="save" />
    <profile-item v-model="selected_partner.manager.phone" label="Phone" @input="save" />
    <profile-item v-model="selected_partner.manager.mailing" label="Address" @input="save" />
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

</div>
<div v-else class="organizations snm-container">
  <h1>No Organization</h1>
  <p class="paragraph">
    You have permission to manage opportunities, but you are not a member of any organization, so this page is blank.
  </p>
</div>
</template>

<script>
import CheckIcon from "~/assets/img/check.svg?inline"
import TrashIcon from '~/assets/img/trash.svg?inline'

export default {
    name: "MyOrganization",

    components:{
        CheckIcon,
        TrashIcon
    },

    httpHeaders() {
        // This will need to be adjusted if we decide the page should be embedded
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });

            return;
        }

        let partners = [];
        let org_types = [];
        let managers = [];
        let pending = [];

        try {
            org_types = await context.$axios.$get('/api/ui/organization/types', context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        try {
            partners = await context.$axios.$get('/api/ui/organization/all', context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        if(partners.length) {
            try {
                managers = await context.$axios.$get('/api/ui/organization/' + partners[0].uid + '/managers', context.store.state.auth);
                pending = await context.$axios.$get('/api/ui/organization/' + partners[0].uid + '/pending-managers', context.store.state.auth);
            }
            catch(err) {
                context.error({
                    statusCode: err.response.status,
                    message: err.response.data
                });
            }
        }

        return {
            partners,
            partner_index: 0,
            org_types,
            managers,
            pending,
        }
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

        choose_partner() {
            return this.partners.length > 1;
        },

        selected_partner() {
            return this.partners[this.partner_index] || {};
        },

        can_leave() {
            return this.user.uid != this.selected_partner.prime;
        },
    },

    watch: {
        'selected_partner.under': async function(val, old) {
            let ref = await this.$axios.$post('/api/ui/organization/exists', {uid: val});
            if(ref.length) {
                this.parent_org_name = ref[0].name;
            }
            else {
                this.parent_org_name = '';
            }
        },

        'selected_partner.uid': async function(val, old) {
            this.managers = await this.$axios.$get('/api/ui/organization/' + val + '/managers', this.$store.state.auth);
            this.pending = await this.$axios.$get('/api/ui/organization/' + val + '/pending-managers', this.$store.state.auth);
        },
    },

    methods: {
        async invite() {
            let emails = this.emails.split(/[ \t\r\n,]+/g);

            this.show_add = false;
            this.emails = '';

            await this.$axios.$post('/api/ui/organization/' + this.selected_partner.uid + '/invite', {emails}, this.$store.state.auth);

            this.$buefy.toast.open('Invitations sent');
        },

        async approve_pending(idx) {
            let entry = this.pending[idx];
            this.pending.splice(idx, 1);
            this.managers.push(entry);
            this.selected_partner.pending = this.selected_partner.pending.filter(x => x != entry.uid);
            this.selected_partner.authorized.push(entry.uid);
            await this.save();
        },

        async discard_pending(idx) {
            let entry = this.pending[idx];
            this.pending.splice(idx, 1);
            this.selected_partner.pending = this.selected_partner.pending.filter(x => x != entry.uid);
            await this.save();
        },

        async discard_authorized(idx) {
            let entry = this.managers[idx];
            this.managers.splice(idx, 1);
            this.selected_partner.authorized = this.selected_partner.authorized.filter(x => x != entry.uid);
            await this.save();
        },

        async leave_org() {
            let {result} = await this.$buefy.dialog.confirm({
                message: 'Are you sure you want to give up your authority to manage ' + this.selected_partner.name + ' on Science Near Me?',
            });

            if(result) {
                this.selected_partner.authorized = this.selected_partner.authorized.filter(x => x != this.user.uid);
                await this.save();
                this.$router.replace('/my/profile');
                this.$buefy.toast.open('Exited organization');
            }
        },

        async save() {
            await this.$axios.$put('/api/ui/organization/' + this.selected_partner.uid, this.selected_partner, this.$store.state.auth);
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
