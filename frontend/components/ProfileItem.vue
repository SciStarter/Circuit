<template>
<div class="profile-item">
  <div>
    <label>{{ label }}</label>
    <b-field v-if="editing">
      <div v-if="choices !== null">
        <b-select v-model="model" >
          <option v-for="choice in choices" :key="choices[0]" :value="choice[0]">{{choice[1]}}</option>
        </b-select>
      </div>
      <div v-else-if="auto !== null">
        <b-autocomplete
          :data="completions"
          :field="auto.label"
          :loading="completing"
          keep-first
          @typing="load_completions"
          @select="model = $event ? (auto.value ? $event[auto.value] : $event) : null"
          />
      </div>
      <b-numberinput v-model="model" v-else-if="value_type === Number" />
      <div v-else-if="value_type === Boolean" class="radio-stack">
        <b-radio v-model="model" :native-value="false" size="is-medium">
          {{ labelFalse }}
        </b-radio>
        <b-radio v-model="model" :native-value="true" size="is-medium">
          {{ labelTrue }}
        </b-radio>
      </div>
      <div v-else-if="color">
        <b-colorpicker ref="cpicker" v-model="color_model"/>
      </div>
      <b-input v-else v-model="model" type="text" />
      <b-button @click="save">
        &check; Save
      </b-button>
      <b-button @click="clear">
        Clear
      </b-button>
    </b-field>
    <div v-else-if="obscure" class="value">
      **********
    </div>
    <div v-else-if="value === null" class="not-set">
      not set
    </div>
    <div v-else-if="value_type === Boolean" class="value">
      <span v-if="value">{{ labelTrue }}</span>
      <span v-else>{{ labelFalse }}</span>
    </div>
    <div v-else-if="choices !== null" class="value">
      {{ choice_label }}
    </div>
    <div v-else-if="color">
      <div :style="{'width': '2rem', 'height': '2rem', 'background-color': value}"></div>
    </div>
    <div v-else class="value">
      {{ display ? display : value }}
    </div>
    <a v-if="!editing" @click="editing = true">(edit)</a>
  </div>
  <div>
    {{ hint }}
  </div>
</div>
</template>

<script>

const NoValue = {};

export default {
    name: "ProfileItem",

    props: {
        label: {
            type: String,
            required: true,
        },

        labelTrue: {
            type: String,
            required: false,
            default: "",
        },

        labelFalse: {
            type: String,
            required: false,
            default: "",
        },

        value: {
            type: [String, Boolean, Number],
            required: false,
            default: "",
        },

        choices: {
            type: [Array, null],
            required: false,
            default: null,
        },

        auto: {
            type: [Object, null],
            required: false,
            default: null,
        },

        obscure: {
            type: Boolean,
            required: false,
            default: false,
        },

        hint: {
            type: String,
            required: false,
            default: "",
        },

        display: {
            type: String,
            required: false,
            default: null,
        },

        color: {
            type: Boolean,
            required: false,
            default: false,
        },
    },

    data() {
        return {
            editing: false,
            overlay: NoValue,
            completions: [],
            completing: false,
        }
    },

    computed: {
        model: {
            get() {
                if(this.overlay !== NoValue) {
                    return this.overlay;
                }
                else {
                    return this.value;
                }
            },

            set(val) {
                this.overlay = val;
            }
        },

        color_model: {
            get() {
                if(!this.model) {
                    return "#ccddee";
                }

                return this.model;
            },

            set(val) {
                this.model = val.toString('hex');
            },
        },

        value_type() {
            switch(typeof(this.value)) {
            case 'string':
                return String;
            case 'boolean':
                return Boolean;
            case 'number':
                return Number;
            default:
                return null;
            }
        },

        choice_label() {
            for(let choice of this.choices) {
                if(choice[0] == this.value) {
                    return choice[1];
                }
            }

            return "Unknown Value";
        },
    },

    methods: {
        async load_completions(text) {
            this.completing = true;
            let query = {};
            query[this.auto.search_field] = text;
            this.completions = await this.$axios.$post(this.auto.url, query);
            this.completing = false;
        },

        save() {
            this.editing = false;

            if(this.color) {
                this.model = this.$refs.cpicker.colorSelected.toString('hex');
            }

            this.$emit('input', this.model);

            this.overlay = NoValue;
        },

        clear() {
            this.editing = false;

            this.$emit('input', null);

            this.overlay = NoValue;
        },
    },
}
</script>

<style lang="scss" scoped>
div.profile-item {
    display: flex;
    flex-direction: column;
    border-bottom: 1px solid $snm-color-border;
    padding: 1rem;

    &.no-border {
      border-bottom:0;
    }

    > div:first-child {
        display: flex;
        flex-direction: column;

        > label {
            font-family: $snm-font-content;
            font-size: $snm-font-small;
            font-weight: normal;
            color: var(--primary-color, $snm-color-element-dark);
        }

        .value {
            font-family: $snm-font-content;
            font-size: $snm-font-medium-small;
            font-weight: bold;
            color: var(--primary-color, $snm-color-element-dark);
        }

        .not-set {
            font-family: $snm-font-content;
            font-size: $snm-font-medium-small;
            font-style: italic;
            color: $snm-color-info;
        }

        .radio-stack {
            display: flex;
            flex-direction: column;

            .b-radio {
                margin-bottom: 0.75rem;
            }
        }
    }

    > div:last-child {
        font-family: $snm-font-content;
        font-style: italic;
        font-size: $snm-font-small;
        color: $snm-color-hint;
    }
}

@media (min-width: $fullsize-screen) {
    div.profile-item {
        > div:first-child {
            flex-direction: row;

            >label {
                text-align: left;
                min-width: 10vw;
            }

            >.value,>.not-set  {
                text-align: left;
                min-width: 20vw;
            }
        }
    }
}
</style>
