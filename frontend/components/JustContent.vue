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

<style>
.not-authenticated  .just-content {
  padding-top: 2rem;
}
</style>
