<template>
<div class="mini-select">
  <label v-if="label">
    {{ label }}
  </label>
  <select v-model="selection">
    <slot />
  </select>
</div>
</template>

<script>
export default {
    name: 'MiniSelect',

    props: {
        value: {
            type: [String, Number, Boolean, Array, Object, Date],
            required: false,
            default: null
        },

        label: {
            type: String,
            required: false,
            default: ''
        }
    },

    data() {
        return {
            active: this.value
        };
    },

    computed: {
        // We use this computed intermediary so we can take advantage
        // of Vue's special-case support for v-model on select
        // elements.
        selection: {
            get() {
                return this.active;
            },

            set(val) {
                this.active = val;
                this.$emit('input', val);
            }
        }
    },

    watch: {
        value(val) {
            this.active = val;
        }
    },

    methods: {
        changed() {

        }
    }
}
</script>

<style lang="scss" scoped>
.mini-select {
    display: inline-block;
}

label {
    font-family: $snm-font-meta;
    font-weight: normal;
    letter-spacing: 0px;
    color: $snm-color-element-dark;
    font-size: $snm-font-small;
    line-height: 19px;
}

select {
    display: inline-block;
    -moz-appearance: none;
    -webkit-appearance: none;
    appearance: none;
    outline: none;
    border: none;
    font-family: $snm-font-meta;
    font-weight: bold;
    letter-spacing: 0px;
    color: $snm-color-element-dark;
    font-size: $snm-font-small;
    line-height: 19px;
    cursor: pointer;
    background-image: url('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAoAAAAGCAYAAAD68A/GAAABhGlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9TpUUqCnaQ4pChOlmQKuKoVShChVArtOpgcukXNGlIUlwcBdeCgx+LVQcXZ10dXAVB8APEzc1J0UVK/F9SaBHrwXE/3t173L0DhEaFaVbPBKDptplOJsRsblUMvCKICAYRhyAzy5iTpBS6jq97+Ph6F+NZ3c/9OfrVvMUAn0g8ywzTJt4gnt60Dc77xGFWklXic+Jxky5I/Mh1xeM3zkWXBZ4ZNjPpeeIwsVjsYKWDWcnUiKeIo6qmU76Q9VjlvMVZq9RY6578haG8vrLMdZojSGIRS5AgQkENZVRgI0arToqFNO0nuvgjrl8il0KuMhg5FlCFBtn1g//B726twmTcSwolgN4Xx/kYBQK7QLPuON/HjtM8AfzPwJXe9lcbwMwn6fW2Fj0CBraBi+u2puwBlzvA8JMhm7Ir+WkKhQLwfkbflAOGboG+Na+31j5OH4AMdZW6AQ4OgbEiZa93eXews7d/z7T6+wFr9HKkFSZPwQAAAAZiS0dEAP8A/wD/oL2nkwAAAAlwSFlzAAAOJgAADiYBou8l/AAAAAd0SU1FB+UIGhA2EA0E/soAAAAZdEVYdENvbW1lbnQAQ3JlYXRlZCB3aXRoIEdJTVBXgQ4XAAAAd0lEQVQI143NsQnCUACE4e/lKUKwE9cQx3IkqyzgADZO4ASCLmBSmEJ8aJBnEQJBLPzL/467sGT+ZpNZ+M29ZBsfvKacClaYjRuBLlDVXCM8SQWXyDowGYqZ3Y0zxEF2/UWtXw6ZfctxyOP4KtGUJDQtB38QvsUH4n4dJXy3Ty0AAAAASUVORK5CYII=');
    background-repeat: no-repeat, repeat;
    background-position: right .2em top 50%, 0 0;
    background-size: .65em auto, 100%;
    padding-right: 1em;
}

select::-ms-expand {
  display: none;
}
</style>
