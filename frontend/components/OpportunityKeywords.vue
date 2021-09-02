<template>
<div class="opportunity-keywords">
  <span v-for="kw in keywords" :key="kw">
    {{ kw }}
  </span>
</div>
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
        keywords() {
            let ret = [];

            if(this.opportunity.opp_descriptor) {
                for(let desc of this.opportunity.opp_descriptor) {
                    if(desc) {
                        ret.push(desc);
                    }
                }
            }

            if(this.opportunity.opp_topics) {
                for(let topic of this.opportunity.opp_topics) {
                    if(topic) {
                        ret.push(topic);
                    }
                }
            }

            if(this.opportunity.tags) {
                for(let tag of this.opportunity.tags) {
                    if(tag) {
                        ret.push(tag);
                    }
                }
            }

            return ret;
        }
    }
}
</script>

<style lang="scss" scoped>
span:not(:first-of-type)::before {
    content: ", ";
}
</style>
