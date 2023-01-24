<template>
  <div class="my snm-wrapper">
    <div class="snm-container">
      <nuxt-child />
    </div>
  </div>
</template>

<script>
export default {
    name: "My",

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
        return {};
    },
}
</script>

<style lang="scss" scoped>
// @media (min-width: 1280px) {
//     div.my {
//         padding: 2rem 2rem 0;
//     }
// }

@media (max-width:1159px) {
  .snm-wrapper {
    padding:0;
  }

  .snm-container {
    padding:1rem;
  }
}

</style>
