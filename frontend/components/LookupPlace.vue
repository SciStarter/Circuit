<template>
<b-field>
  <b-select v-model="query.radius">
    <option :value="80467">50 miles</option>
    <option :value="40233">25 miles</option>
    <option :value="16093">10 miles</option>
    <option :value="8046">5 miles</option>
    <option :value="value.radius">{{ value_miles }} miles</option>
  </b-select>
  <b-input v-model="query.near" placeholder="e.g. Iowa City, IA"/>
</b-field>
</template>

<style lang="scss" scoped>
>>> .select::after {
    border-color: $snm-color-hint !important;
}
</style>

<script>
const MILES = 0.000621371;

function clamp_radius(place) {
    if(place.radius > 0 && place.radius < 100000) {
        return place;
    }

    place.radius = 80467;

    return place;
}

export default {
    props: {
        value: {
            type: Object,
            required: false,
            default: {
                near: "",
                longitude: 0,
                latitude: 0,
                radius: 0
            }
        }
    },

    data() {
        return {
            query: clamp_radius(JSON.parse(JSON.stringify(this.value)))
       };
    },

    computed: {
        value_miles() {
            return (this.value.radius * MILES).toFixed(2);
        }
    },

    methods: {
        changed() {
            this.$emit('input', this.query);
        }
    }
}
</script>
