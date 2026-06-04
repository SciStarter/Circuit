<template>
<div class="opportunity-list">
    <Pagination
    v-if="opportunities.matches.length > 0 && showTopPagination"
    :page-index="opportunities.pagination.page_index"
    :last-page="opportunities.pagination.last_page"
    @switch="$emit('switch', $event)" 
    small />

  <opportunity-card v-for="opp in opportunities.matches" :key="opp.uid" :opportunity="opp" :exchange="exchange" previous-page="find" />

  <Pagination
    v-if="opportunities.matches.length > 0"
    :page-index="opportunities.pagination.page_index"
    :last-page="opportunities.pagination.last_page"
    @switch="$emit('switch', $event)" />
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

        exchange: {
            type: Object,
            required: false,
            default: null,
        },
        showTopPagination: {
            type: Boolean,
            required: false,
            default: false
        }
    },
};
</script>
