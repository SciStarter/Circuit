<template>
<div id="submit-opportunity">
  <div class="flex">
    <h1 class="h2">Add an Opportunity
      <span v-if="choose_partner">
        to
        <b-select v-model="partner_index" size="is-large" aria-role="list">
          <option v-for="(partner, idx) in partners" :key="partner.uid" :value="idx" aria-role="listitem">{{partner.name}}</option>
        </b-select>
      </span>
    </h1>
    <a class="cancel">cancel</a>
  </div>

  <opportunity-form :partner="selected_partner" :opportunity="opp"/>

</div>
</template>

<script>
export default {
    httpHeaders() {
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
        }

        let partners = [];
        let opp = null;

        try {
            opp = await context.$axios.$get('/api/ui/opportunity/');
            partners = await context.$axios.$get('/api/ui/profile/partners', context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        return {
            partners,
            opp,
        }
    },

    data() {
        return {
            partner_index: 0,
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
            return this.partners[this.partner_index] || null;
        },
    },
}
</script>

<style lang="scss" scoped>

#submit-opportunity {
  max-width:1000px;
  margin:0 auto;
}
  .flex {
    display:flex;
    align-items:center;
    justify-content:space-between;
  }
  footer {
    display: none;
  }
  h1 {
    font-family: $snm-font-heading;
    font-size: 1.8rem;
    font-weight:bold;
    color: $snm-color-element-med;
  }
  .cancel {
    color: $snm-color-info;
    text-decoration:underline;
  }

  @media (max-width:959px) {
    #submit-opportunity {
      padding:0 20px;
    }
  }
</style>
