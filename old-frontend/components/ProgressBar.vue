<template>
<div class="progress-bar">
  <label>{{ label }}</label>
  <label v-if="countdown" :class="{'alert': alert}">{{ max - value }} {{ units }}</label>
  <label v-else :class="{'alert': alert}">{{ value - min }} / {{ max - min }} ({{ percentage }}%)</label>
  <div ref="bar" :style="{'--pixels': pixels}" />
</div>
</template>

<script>
export default {
    props: {
        label: {
            type: String,
            required: false,
            default: "",
        },

        min: {
            type: Number,
            required: false,
            default: 0,
        },

        max: {
            type: Number,
            required: false,
            default: 100,
        },

        value: {
            type: Number,
            required: false,
            default: 0,
        },

        units: {
            type: String,
            required: false,
            default: "",
        },

        countdown: {
            type: Boolean,
            required: false,
            default: false,
        },

        alertThreshold: {
            type: [Number, null],
            required: false,
            default: null,
        },
    },

    data() {
        return {
            observer: null,
            width: 0,
        };
    },

    computed: {
        pixels() {
            return (this.scale * this.width) + 'px';
        },

        scale() {
            return (this.value - this.min) / (this.max - this.min);
        },

        percentage() {
            return Math.round(this.scale * 100);
        },

        alert() {
            if(this.alertThreshold === null) {
                return false;
            }

            if(this.countdown) {
                return this.value >= this.alertThreshold;
            }
            else {
                return this.value <= alertThreshold;
            }
        },
    },

    mounted() {
        this.observer = new ResizeObserver(entries => {
            this.width = entries[0].borderBoxSize[0].inlineSize;
        });

        this.observer.observe(this.$refs.bar, { box: 'border-box' });
    },
}
</script>

<style lang="scss" scoped>
div.progress-bar {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    writing-mode: horizontal-tb;

    > label {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-small;
        color: var(--secondary-color, $snm-color-element-med);
        margin-right: 2rem;

        &:first-child {
            color: var(--primary-color, $snm-color-element-dark);
        }

        &.alert {
            color: $snm-color-info;
        }
    }

    > div {
        border-radius: 0px;
        box-sizing: border-box;
        width: 100%;
        height: $snm-font-medium;
        background-color: $snm-color-border;
        border-left-style: solid;
        border-left-color: var(--secondary-color, $snm-color-element-med);
        border-left-width: var(--pixels);
    }
}

@media (min-width: $fullsize-screen) {
    div.progress-bar {
        justify-content: start;
    }
}
</style>
