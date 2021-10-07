<template>
  <div class="my">
    <div class="snm-container">
      <nuxt-child />
    </div>
  </div>
</template>

<script>
export default {
    name: "My",
}
</script>

<style lang="scss" scoped>
@media (min-width: $fullsize-screen) {
    div.my {
        padding: 2rem 2rem 0;
    }
}
</style>
