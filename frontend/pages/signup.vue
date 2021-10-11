<template>
<signup-form :next="$route.query.next" :query="next_query" />
</template>

<script>
import cloneDeep from 'lodash/cloneDeep'

export default {
    computed: {
        next_query() {
            let q = cloneDeep(this.$route.query);
            delete q.next;
            return q;
        }
    },
}
</script>

<style lang="scss" scoped>

</style>
