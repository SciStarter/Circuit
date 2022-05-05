<template>
<div class="opportunity-list">
  <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :exchange="exchange" previous-page="find" />

  <Pagination
    v-if="opportunities.matches.length > 0"
    :page-index="opportunities.pagination.page_index"
    :last-page="opportunities.pagination.last_page"
    @switch="$emit($event)" />
</div>
</template>

<script>
export default {
    name: "OpportunityList",

    props: {
        opportunities: {
            type: Object,
            required: true,
        },
    },
};
</script>
