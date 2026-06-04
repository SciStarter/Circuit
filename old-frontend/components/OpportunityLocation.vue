<template>
<div class="opportunity-location">
  <span v-if="opportunity.location_type == 'online'">Online</span>
  <span v-else-if="opportunity.location_type == 'any'">Anywhere</span>
  <p v-else-if="!isOpportunity && opportunity.location_polygon && opportunity.location_polygon.type === 'MultiPolygon'">
    <em>Multiple locations, including yours!</em>
  </p>
  <p v-else-if="!isOpportunity && opportunity.location_point && opportunity.location_point.type === 'Point'">
    <span v-if="location_name">
      <template v-if="shortstacked"><template v-if="location_name">{{ location_name }}, </template>{{ opportunity.address_state }}</template>
      <template v-else>{{ location_name }} {{ opportunity.address_state }}</template>
    </span>
    <span v-else-if="(opportunity.address_city || opportunity.address_state)">
      <template v-if="opportunity.address_city">{{ opportunity.address_city }}, </template>{{ opportunity.address_state }}
    </span>
    <em v-else>Near your search location!</em>
  </p>
  <p v-else-if="isOpportunity && opportunity.location_polygon && opportunity.location_polygon.type === 'MultiPolygon'">
    In a specific area
  </p>
  <ul v-else-if="isOpportunity && opportunity.location_point && opportunity.location_point.type === 'Point'">
    <li v-if="location_name">
      {{ location_name }} {{ opportunity.address_state }}
    </li>
    <li v-else-if="short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_street && (!short || !location_name)">
      {{ opportunity.address_street }}
    </li>
    <li v-if="(opportunity.address_city || opportunity.address_state) && !short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_country && !short">
      {{ opportunity.address_country }}
    </li>
  </ul>
  <ul v-else-if="opportunity.location_type == 'at'">
    <li v-if="location_name">
      {{ location_name }} {{ opportunity.address_state }}
    </li>
    <li v-if="short">
      {{ opportunity.address_city }}, {{ opportunity.address_state }}
    </li>
    <li v-if="opportunity.address_street && (!short || !location_name)">
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
import {decode} from 'html-entities';

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
        shortstacked: {
            type: Boolean,
            required: false,
            default: false
        },

        isOpportunity: {
            type: Boolean,
            required: false,
            default: false
        },
    },

    computed: {
        location_name() {
            return decode(this.opportunity.location_name);
        }
    },
}
</script>

<style lang="scss" scoped>
ul {
    display: inline-block;
}

li {
    font-family: $snm-font-content;
    letter-spacing: 0px;
    color: var(--primary-color, $snm-color-element-dark);
    font-size: $snm-font-smaller;
    line-height: 1.2;
}
</style>
