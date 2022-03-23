<template>
<div v-if="selected_partner.uid" class="organizations snm-container">
  <p v-if="choose_partner">
    <b-select v-model="partner_index" size="is-large" aria-role="list">
      <option v-for="(partner, idx) in partners" :key="partner.uid" :value="idx" aria-role="listitem">{{partner.name}}</option>
    </b-select>
    <br>
  </p>
  <h1 v-else>Your Partner Organization</h1>

  <partner-form
    :partner="selected_partner"
    :org_types="org_types"
    :managers="managers"
    :pending="pending"/>

</div>
<div v-else class="organizations snm-container">
  <h1>No Organization</h1>
  <p class="paragraph">
    You have permission to manage opportunities, but you are not a member of any organization, so this page is blank.
  </p>
</div>
</template>

<script>
export default {
    name: "MyOrganization",

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

    computed: {
        choose_partner() {
            return this.partners.length > 1;
        },

        selected_partner() {
            return this.partners[this.partner_index] || {};
        },
    },
}
</script>

<style lang="scss" scoped>
h1 {
    font-family: $snm-font-heading;
    font-size: 1.8rem;
    font-weight:bold;
    color: $snm-color-element-med;
    margin-bottom:2rem;
}

@media (max-width:1159px) {
  .snm-container {
    padding:1rem;
  }
}

.nav-tab-wrapper {
  width:100%;
  overflow:auto;
  .nav-tabs {
    min-width: 500px
  }
}
.nav-tab-wrapper::-webkit-scrollbar {
  display: none;
}

@media (max-width:600px){
  .flex.managers {
    flex-direction:column;
    align-items:flex-start;
  }
}
</style>
