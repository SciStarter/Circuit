<template>
  <a @click="$emit('click')" :title="title" class="arrow right"><slot/></a>
</template>

<style lang="scss" scoped>
a {
  cursor: pointer;
  display: inline-block;
  height: 40px;
  margin-left: 40px;
  margin-right: 40px;
  position: relative;
  line-height: 2.5em;
  padding-left: 1em;
  padding-right: 2em;
  background: $snm-color-element-med;
   &::after {
    border-left: 20px solid $snm-color-element-med;
   }
   box-shadow: 0px 0px 6px 2px $snm-color-shadow;
}

a::after {
  content: "";
  position: absolute;
  border-bottom: 20px solid transparent;
  border-top: 20px solid transparent;
  height: 0px;
  width: 0px;
  margin-right: -20px;
  right: 0;
}

a:hover, a:active {
  background: $snm-color-background-meddark;
  &::after {
    border-left: 20px solid $snm-color-background-meddark;
  }
}
</style>

<script>
export default {
    props: {
        title: {
            type: String,
            required: false,
            default: "",
        },
    }
}

</script>
