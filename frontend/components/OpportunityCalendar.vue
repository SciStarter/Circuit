<template>
<div class="opportunity-calendar">
  <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :exchange="exchange" previous-page="find" />
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
    },
};
</script>

<style lang="scss" scoped>
.opportunity-calendar {
    max-width: 900px;
}
</style>
