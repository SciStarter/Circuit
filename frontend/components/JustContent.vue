<template>
  <dynamic-block class="content" :group="entity.slug" item="content"/>
</template>

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
