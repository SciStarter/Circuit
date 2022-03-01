<template>
<b-field label="Name" :label-position="labelPosition" class="location-input" autocomplete="off">
  <b-autocomplete
    :loading="loading"
    :data="matches"
    :value="value"
    :name="'new-' + Math.random()"
    :clearable="true"
    placeholder="e.g. Olympia, WA"
    @typing="completions"
    @select="select"
    />
</b-field>
</template>

<script>
import debounce from 'lodash/debounce'

export default {
    name: "LookupGeometry",

    props: {
        value: {
            type: String,
            required: false,
            default: ''
        },

        labelPosition: {
            type: String,
            required: false,
            default: 'on-border'
        },
    },

    data () {
        return {
            search: "",
            matches: [],
            num_loading: 0
        }
    },

    computed: {
        loading () {
            return this.num_loading > 0
        },
    },

    methods: {
        completions: debounce(function (near) {
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
            if(evt === undefined || evt === null || evt === '') {
                return;
            }

            this.num_loading += 1

            this.$axios.$post('/api/ui/finder/geom', { q: evt })
                .then((result) => {
                    this.$emit('input', evt);

                    this.$emit('license', result.licence);

                    if(result.class == "boundary") {
                        this.$emit('polygon', result.geojson);
                    }
                    else {
                        this.$emit('point', {type: "Point", coordinates: [parseFloat(result.lon), parseFloat(result.lat)]});
                    }
                })
                .catch((error) => { console.error(error) })
                .finally(() => { this.num_loading -= 1 })
        },
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
