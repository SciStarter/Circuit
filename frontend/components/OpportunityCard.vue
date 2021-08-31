<template>
<article class="opportunity-card">
  <div class="primary">
    <img :src="opportunity.image_url">
    <h2>{{ subtitle }}</h2>
    <a :href="'/' + opportunity.slug"><h1>{{ opportunity.title }}</h1></a>
  </div>
  <div class="secondary">
    foo
  </div>
</article>
</template>

<script>
export default {
    props: {
        opportunity: {
            type: Object,
            required: true
        }
    },

    computed: {
        subtitle() {
            return this.opportunity.organization_name || this.opportunity.partner_name;
        }
    }
}
</script>

<style lang="scss" scoped>
.opportunity-card {

}
</style>
