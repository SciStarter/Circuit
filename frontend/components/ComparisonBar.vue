<template>
        <div class="outer" :style="{width: width, height: height, 'background-color': background}"><div class="inner" :style="{width: bar_width, height: height, 'background-color': color}"></div></div>
</template>

<script>
export default {
    name: "ComparisonBar",

    props: {
        value: {
            type: Number,
            required: true,
            default: 0,
        },

        min: {
            type: Number,
            required: false,
            default: 0,
        },

        max: {
            type: Number,
            required: true,
            default: 100,
        },

        width: {
            type: String,
            required: false,
            default: "100px"
        },

        height: {
            type: String,
            required: false,
            default: "10px"
        },

        color: {
            type: String,
            required: false,
            default: "#000"
        },

        background: {
            type: String,
            required: false,
            default: "transparent"
        },
        tooltip: {
            type: Boolean,
            required: false,
            default: false
        },
    },

    computed: {
        bar_width() {
            let span = this.max - this.min;
            let val = Math.min(this.value, this.max) - this.min;
            return `${100 * (val / span)}%`;
        },
    },
}
</script>

<style type="scss" scoped>
div.outer {
    position: relative;
    padding: 0px;
}

div.inner {
    position: absolute;
    top: 0px;
    left: 0px;
    padding: 0px;
    margin: 0px;
    border: 0px;
}
</style>
