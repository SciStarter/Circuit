<template>
  <div>
    <h1>{{ entity.title }}</h1>
    <p>
      Here's where the opportunity page should be rendered.
    </p>
  </div>
</template>

<style lang="scss" scoped>
</style>

<script>
export default {
  props: {
    entity: {
      type: Object,
      required: true
    }
  },
}
</script>
