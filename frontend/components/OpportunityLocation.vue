<template>
<div class="opportunity-location">
  <span v-if="opportunity.location_type == 'online'">Online</span>
  <span v-else-if="opportunity.location_type == 'any'">Anywhere</span>
  <ul v-else-if="opportunity.location_type == 'at'">
    <li v-if="opportunity.location_name">
      {{ opportunity.location_name }}<template v-if="short"></template>
    </li>
    <li v-if="short">{{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_street && (!short || !opportunity.location_name)">
      {{ opportunity.address_street }}
    </li>
    <li v-if="opportunity.address_city && !short">
      {{ opportunity.address_city }}
    </li>
    <li v-if="(opportunity.address_state || opportunity.address_zip) && !short">
      {{ opportunity.address_state }} {{ opportunity.address_zip }}
    </li>
    <li v-if="opportunity.address_country && !short">
      {{ opportunity.address_country }}
    </li>
  </ul>
  <div v-else-if="opportunity.location_type == 'near'">
    <p v-if="opportunity.location_polygon && opportunity.location_polygon.type === 'MultiPolygon'">
    <!--   !!! TODO -->
    </p>
    <p v-else-if="opportunity.location_point && opportunity.location_point.type === 'Point'">
    <!--   !!! TODO -->
    </p>
    <p v-else>
      Unknown location
    </p>
  </div>
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
        }
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
