<template>
  <div class="snm-wrapper">
    <div class="snm-container just-content">
      <dynamic-block class="content" :group="entity.slug" item="content" />
    </div>
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
.not-authenticated  .just-content {
  padding-top: 2rem;
}
.content {
  max-width: 800px;
  margin: 0 auto;
}
a, a span {
  color: var(--secondary-color, $snm-color-element-med) !important;
  font-family: $snm-font-content!important;
}
a:hover, a:hover span {
  color: var(--primary-color, $snm-color-element-dark) !important;
}

</style>
