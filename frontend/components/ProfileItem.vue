<template>
<div class="profile-item">
  <div>
    <label>{{ label }}</label>
    <b-field v-if="editing">
      <b-numberinput v-if="value_type === Number" />
      <div v-else-if="value_type === Boolean" class="radio-stack">
        <b-radio v-model="model" :native-value="false" size="is-medium">
          {{ labelFalse }}
        </b-radio>
        <b-radio v-model="model" :native-value="true" size="is-medium">
          {{ labelTrue }}
        </b-radio>
      </div>
      <b-input v-else v-model="model" type="text" />
      <b-button @click="save">
        &check; Save
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
    <div v-else class="value">
      {{ value }}
    </div>
    <a v-if="!editing" @click="editing = true">(edit)</a>
  </div>
  <div>
    {{ hint }}
  </div>
</div>
</template>

<script>
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

        obscure: {
            type: Boolean,
            required: false,
            default: false,
        },

        hint: {
            type: String,
            required: false,
            default: "",
        }
    },

    data() {
        return {
            editing: false,
            overlay: null,
        }
    },

    computed: {
        model: {
            get() {
                if(this.overlay !== null) {
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
    },

    methods: {
        save() {
            this.editing = false;

            this.$emit('input', this.model);

            this.overlay = null;
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
            color: $snm-color-element-dark;
        }

        .value {
            font-family: $snm-font-content;
            font-size: $snm-font-medium-small;
            font-weight: bold;
            color: $snm-color-element-dark;
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
