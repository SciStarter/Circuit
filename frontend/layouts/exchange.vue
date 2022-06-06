<template>
    <nuxt />
</template>

<script>
  export default {
      async mounted() {
          this.$gtm.push({"user_id": window.localStorage.getItem("session")});
      }
  }
</script>
