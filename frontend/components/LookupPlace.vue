<template>
<b-field class="lookup-place">
  <b-field label="Near" :label-position="labelPosition">
    <b-autocomplete
      :loading="loading"
      :data="matches"
      field="near"
      :value="sanitized_value.near"
      placeholder="e.g. Iowa City, IA"
      @typing="completions"
      @select="change($event)"
      @input="change({near: $event})"
      />
  </b-field>
  <b-field label="Distance" :label-position="labelPosition">
    <b-select :value="sanitized_value.proximity" @input="change({proximity: $event})">
      <option :value="80467">
        50 miles
      </option>
      <option :value="40233">
        25 miles
      </option>
      <option :value="16093">
        10 miles
      </option>
      <option :value="8046">
        5 miles
      </option>
      <option :value="sanitized_value.proximity">
        {{ value_miles }} miles
      </option>
    </b-select>
  </b-field>
</b-field>
</template>

<script>
import debounce from 'lodash/debounce'

const MILES = 0.000621371

export default {
    props: {
        value: {
            type: Object,
            required: false,
            default: () => {
                return {
                    near: '',
                    longitude: 0,
                    latitude: 0,
                    proximity: 0
                }
            }
        },

        labelPosition: {
            type: String,
            required: false,
            default: 'on-border'
        }
    },

    data () {
        return {
            matches: [],
            num_loading: 0
        }
    },

    computed: {
        loading () {
            return this.num_loading > 0
        },

        sanitized_value () {
            const patch = {}

            if (!this.value.proximity || this.value.proximity < 1 || this.value.proximity > 100000) {
                patch.proximity = 80467
            }

            if (!this.value.longitude) {
                patch.longitude = 0
            }

            if (!this.value.latitude) {
                patch.latitude = 0
            }

            if (!this.value.near) {
                patch.near = ''
            }

            return Object.assign({}, this.value, patch)
        },

        value_miles () {
            return (this.value.proximity * MILES).toFixed(2)
        }
    },

    mounted () {
        if (this.value && this.value.near === '') {
            if (this.value.longitude !== 0 || this.value.latitude !== 0) {
                this.complete_near()
            } else if (this.$geolocation.checkSupport()) {
                this.num_loading += 1

                this.$geolocation.getCurrentPosition()
                    .then(({ coords: { latitude, longitude } }) => {
                        this.change({ longitude, latitude })
                        this.$nextTick(() => { this.complete_near() })
                    })
                    .finally(() => { this.num_loading -= 1 })
            }
        }

        this.change(this.sanitized_value)
    },

    methods: {
        completions: debounce(function (near) {
            if (near.length < 3) {
                this.matches = []
                return
            }

            this.num_loading += 1

            this.$axios.$post('/api/ui/finder/geo', { lookup: 'coords', place: this.sanitized_value })
                .then(({ places }) => { this.matches = places })
                .catch((error) => { this.matches = []; console.error(error) })
                .finally(() => { this.num_loading -= 1 })
        }, 500),

        complete_near () {
            this.num_loading += 1

      this.$axios.$post('/api/ui/finder/geo', { lookup: 'near', place: this.value })
        .then(({ places }) => {
          if (places.length > 0) {
            this.change({ near: places[0].near })
          }
        })
        .catch((error) => { console.error(error) })
        .finally(() => { this.num_loading -= 1 })
    },

    change (delta) {
      const result = Object.assign({}, this.value, delta)
      this.$emit('input', result)
    }
  }
}
</script>

<style lang="scss">
.lookup-place .autocomplete .dropdown-menu {
    width: 350px;
}
</style>
