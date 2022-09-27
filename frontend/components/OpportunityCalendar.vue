<template>
<div class="opportunity-calendar">
  <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :exchange="exchange" previous-page="find" />
  <div class="nav">
    <a v-if="hasPrevious" @click="$emit('prev')">&laquo;&nbsp;previous</a><span v-else>&bull;</span><a @click="$emit('next')">next&nbsp;&raquo;</a>
  </div>
</div>
</template>

<script>
export default {
    name: "OpportunityCalendar",

    props: {
        opportunities: {
            type: Object,
            required: true,
        },

        exchange: {
            type: Object,
            required: false,
            default: null,
        },

        hasPrevious: {
            type: Boolean,
            required: true,
            default: true,
        },
    },
};
</script>

<style lang="scss" scoped>
.opportunity-calendar {
    max-width: 900px;
}
.nav {
    display: flex;
    justify-content: space-between;
}
</style>
