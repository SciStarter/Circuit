<template>
<div v-if="keywords.length > 0" class="opportunity-keywords">
  <span v-for="kw in keywords" :key="kw">
    {{ kw }}
  </span>
</div>
<div v-else class="opportunity-keywords">
  <em>no keywords assigned</em>
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
