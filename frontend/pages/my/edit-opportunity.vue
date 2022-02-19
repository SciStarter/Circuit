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
  <opportunity-form edit-mode />
</div>
</template>

<script>
import EyeIcon from '~/assets/img/eye.svg?inline'

export default {
  components: {
    EyeIcon
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
