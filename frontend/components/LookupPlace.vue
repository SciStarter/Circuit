<template>
<b-field class="lookup-place" :class="{'stacked':stacked,'widget':widget}">
  <b-field label="Near" :label-position="labelPosition" class="location-input" autocomplete="off">
    <b-autocomplete
      :loading="loading"
      :data="matches"
      field="near"
      :value="sanitized_value.near"
      :name="'new-' + Math.random()"
      :clearable="true"
      placeholder="e.g. Iowa City, IA"
      @typing="completions"
      @select="select"
      @input="check_valid"
      />
  </b-field>
  <b-field label="Distance" :label-position="labelPosition" class="distance">
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
      <option :value="0">
        Anywhere
      </option>
      <!-- <option v-if="value_miles" :value="sanitized_value.proximity"> -->
      <!--   {{ value_miles }} miles -->
      <!-- </option> -->
    </b-select>
  </b-field>
</b-field>
</template>

<script>
import debounce from 'lodash/debounce'

const MILES = 0.000621371

export default {
    name: "LookupPlace",

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
        },
        stacked: {
            type: Boolean,
            required: false,
            default: false
        },
        widget: {
            type: Boolean,
            required: false,
            default: false
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

        sanitized_value: {
            get() {
                const patch = {}

                if (this.value.proximity === undefined || this.value.proximity < 0 || this.value.proximity > 100000) {
                    patch.proximity = 40233
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

                const ret = Object.assign({}, this.value, patch);

                return ret
            },

            set(val) {
                this.$emit('input', val);
            }
        },

        value_miles () {
            switch(this.value.proximity) {
            case 80467:
            case 40233:
            case 16093:
            case 8046:
                return "";
            default:
                const miles = this.value.proximity * MILES;

                if(miles > 5) {
                    return Math.ceil(miles).toString();
                }

                return miles.toFixed(2)
            }
        }
    },

    methods: {
        completions: debounce(function (near) {
            this.$emit('valid', false);

            this.matches = []

            if (near.length < 3) {
                return
            }

            this.num_loading += 1

            this.$axios.$get('https://geocode.arcgis.com/arcgis/rest/services/World/GeocodeServer/suggest?f=json&text=' + encodeURIComponent(near))
                .then(({ suggestions }) => { this.matches = suggestions.map(x => x.text); })
                .catch((error) => { this.matches = []; console.error(error) })
                .finally(() => { this.num_loading -= 1 })
        }, 500),

        select(evt) {
            if(evt === undefined) {
                return;
            }

            if(evt === null || evt === '') {
                this.change({near: '', longitude: 0, latitude: 0, proximity: 0});
            }

            this.num_loading += 1

            this.$axios.$post('/api/ui/finder/geo', { lookup: 'coords', place: { near: evt, longitude: 0, latitude: 0, proximity: this.sanitized_value.proximity }})
                .then(({ places }) => {
                    if (places.length > 0) {
                        places[0].near = evt;
                        this.change(places[0])
                    }
                })
                .catch((error) => { console.error(error) })
                .finally(() => { this.num_loading -= 1 })
        },

        change(delta) {
            if(delta['near'] === "") {
                delta.latitude = 0;
                delta.longitude = 0;
            }

            this.sanitized_value = Object.assign({}, this.sanitized_value, delta);

            this.check_valid()
        },

        check_valid(val) {
            if(val === undefined) {
                val = this.sanitized_value;
            }

            const valid = !val.near || !!val.latitude || !!val.longitude;

            this.$emit('valid', valid);

            return valid;
        }
    }
}
</script>

<style lang="scss">
.lookup-place .autocomplete .dropdown-menu {
    width: 350px;
    text-align:left;
}
.lookup-place {
  margin-top: 1.2rem;
}
.location-input {
  width: 100%!important;
  input {
    border-radius: 6px 0 0 6px!important;
  }
}
.field input.input {
  border-radius: 6px;
}
.lookup-place .distance {
  width: 115px;
}

.stacked {
  .field,.field-body, .field.has-addons{
    display: block!important;
  }
  .field-body .field .field {
    margin-bottom:8px;
  }
  input,select {
    border-radius: 6px!important;
  }
  .distance,.control, .select, select {
    width:100%!important;
  }
  .autocomplete .dropdown-menu,.autocomplete .dropdown-content {
      width: 180px;
      text-align:left;
  }
  a.dropdown-item {
    padding-right:1rem;
  }
}

.widget {
  .autocomplete .dropdown-content {
    max-height:110px;
  }
}

</style>
