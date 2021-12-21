<template>
  <div class="snm-wrapper">
    <div class="snm-container">
    <div v-if="error.statusCode === 404">
      <h1>Page not found</h1>
      <nuxt-link to="/">
        Return to the home page
      </nuxt-link>
    </div>
    <div v-else-if="error.statusCode === 401">
      <h1>You need to log in</h1>
    </div>
    <div v-else>
      <h1>An error occurred</h1>
      <nuxt-link to="/">
        Return to the home page
      </nuxt-link>
    </div>
  </div>
  </div>
</template>

<script>
  export default {
      layout: 'default',

      props: {
          'error': {
              type: Object,
              required: true,
              default: { statusCode: 500, message: "undefined error occurred" }
          },
      },
  }
</script>

<style lang="scss" scoped>
h1 {
  font-size: 3rem;
  font-weight: bold;
}
.snm-container {
  padding: 4rem 0;
}
</style>
