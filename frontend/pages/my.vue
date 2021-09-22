<template>
  <div class="my">
    <nuxt-child />
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
        margin: 10px 5rem;
    }
}
</style>
