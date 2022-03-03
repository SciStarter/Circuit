<template>
<div id="edit-opportunity">
  <div class="flex">
    <h1>Edit Opportunity</h1>
    <action-button class="round-btn" principal>
      <div class="icon">
        <eye-icon />
      </div>
      Return to opportunity
    </action-button>
  </div>
  <opportunity-form v-model="opp" :partner="partner" :timezones="timezones" :descriptors="descriptors" :topics="topics" edit-mode />
</div>
</template>

<script>
import EyeIcon from '~/assets/img/eye.svg?inline'

export default {
    name: "EditOpportunity",

    components: {
        EyeIcon
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
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });
        }

        let timezones = [];
        let descriptors = [];
        let topics = [];
        let partners = [];
        let opp = null;

        try {
            timezones = await context.$axios.$get('/api/ui/timezone');
            descriptors = await context.$axios.$get('/api/ui/finder/descriptors');
            topics = await context.$axios.$get('/api/ui/finder/topics');
            partners = await context.$axios.$get('/api/ui/profile/partners', context.store.state.auth);
            opp = await context.$axios.$get('/api/ui/opportunity/' + context.params.uid, context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        return {
            timezones,
            descriptors,
            topics,
            partners,
            opp,
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },

        partner() {
            for(let p of this.partners) {
                if(p.uid == this.opp.partner) {
                    return p;
                }
            }

            return null;
        }
    },
}
</script>

<style lang="scss" scoped>
#edit-opportunity {
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
    margin-bottom: 2rem;
  }
  .cancel {
    color: $snm-color-info;
    text-decoration:underline;
  }

  @media (max-width:959px) {
    #edit-opportunity {
      padding:0 20px;
    }
  }
</style>
