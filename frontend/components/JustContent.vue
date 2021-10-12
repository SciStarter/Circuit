<template>
  <div class="snm-wrapper">
    <div class="snm-container">
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
