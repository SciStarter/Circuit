<template>
  <div>
    <h1>{{ entity.title }}</h1>
    <p>
      Here's where the opportunity page should be rendered.
    </p>
  </div>
</template>

<script>
export default {
  props: {
    entity: {
      type: Object,
      required: true
    }
  }
}
</script>

<style lang="scss" scoped>
</style>
