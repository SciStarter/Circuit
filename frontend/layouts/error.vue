<template>
  <div class="snm-wrapper">
    <div class="snm-container">
    <div v-if="error.statusCode === 404">

      <div class="error">
        <div class="error-left">
          <img src="~/assets/img/404.svg" alt="error" />
        </div>
        <div class="error-right">
          <div>
          <h1>404</h1>
          <h2>We've searched the universe, but we can't find what you're looking for.
</h2>
          <p>Perhaps try the <nuxt-link to="/find">science opportunity search page</nuxt-link>.</p>
          <p>Or maybe <nuxt-link to="/">our homepage</nuxt-link>.</p>
        </div>
        </div>
      </div>
    </div>
    <div v-else-if="error.statusCode === 401">
      <h1>You need to log in</h1>
    </div>
    <div v-else>
      <div class="error">
        <div class="error-left">
          <img src="~/assets/img/404.svg" alt="error" />
        </div>
        <div class="error-right">
          <div>
          <h1>Error</h1>
          <h2>We've searched the universe, but we can't find what you're looking for.
</h2>
          <p>Perhaps try the <nuxt-link to="/find">science opportunity search page</nuxt-link>.</p>
          <p>Or maybe <nuxt-link to="/">our homepage</nuxt-link>.</p>
        </div>
      </div>
      </div>
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

.error {
  display: flex;

  .error-left {
    max-width: 33%;
    margin-right: 2rem;
  }
  .error-right {
    display: flex;
    align-content: center;
    flex-wrap: wrap;

    h1 {
      color: $snm-color-background-dark;
    }

    h2 {
      font-weight: bold;
    font-style: italic;
    font-size: 1.2rem;
    margin-bottom: 1rem;
    }
    p {
      margin-bottom: 0.5rem;
    }
  }

}
</style>
