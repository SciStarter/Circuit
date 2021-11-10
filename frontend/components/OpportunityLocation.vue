<template>
<div class="opportunity-location">
  <p v-if="!isOpportunity && opportunity.location_polygon && opportunity.location_polygon.type === 'MultiPolygon'">
    <em>See map on opportunity page</em>
  </p>
  <p v-else-if="!isOpportunity && opportunity.location_point && opportunity.location_point.type === 'Point'">
    <span v-if="opportunity.location_name">
      {{ opportunity.location_name }}
    </span>
    <span v-else-if="(opportunity.address_city || opportunity.address_state) && !short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </span>
    <em v-else>See map on opportunity page</em>
  </p>
  <p v-else-if="isOpportunity && opportunity.location_polygon && opportunity.location_polygon.type === 'MultiPolygon'">
    In a specific area
  </p>
  <ul v-else-if="isOpportunity && opportunity.location_point && opportunity.location_point.type === 'Point'">
    <li v-if="opportunity.location_name">
      {{ opportunity.location_name }}
    </li>
    <li v-if="short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_street && (!short || !opportunity.location_name)">
      {{ opportunity.address_street }}
    </li>
    <li v-if="(opportunity.address_city || opportunity.address_state) && !short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_country && !short">
      {{ opportunity.address_country }}
    </li>
  </ul>
  <span v-else-if="opportunity.location_type == 'online'">Online</span>
  <span v-else-if="opportunity.location_type == 'any'">Anywhere</span>
  <ul v-else-if="opportunity.location_type == 'at'">
    <li v-if="opportunity.location_name">
      {{ opportunity.location_name }}
    </li>
    <li v-if="short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_street && (!short || !opportunity.location_name)">
      {{ opportunity.address_street }}
    </li>
    <li v-if="(opportunity.address_city || opportunity.address_state) && !short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_country && !short">
      {{ opportunity.address_country }}
    </li>
  </ul>
  <span v-else>Unrecognized location type {{ opportunity.location_type }}</span>
</div>
</template>

<script>
export default {
    props: {
        opportunity: {
            type: Object,
            required: true
        },

        short: {
            type: Boolean,
            required: false,
            default: false
        },

        isOpportunity: {
            type: Boolean,
            required: false,
            default: false
        },
    }
}
</script>

<style lang="scss" scoped>
ul {
    display: inline-block;
}

li {
    font-family: $snm-font-content;
    letter-spacing: 0px;
    color: $snm-color-element-dark;
    font-size: $snm-font-smaller;
}
</style>
