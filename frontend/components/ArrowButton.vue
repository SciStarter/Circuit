<template>
<span><a @click="$emit('click')" :title="title"><slot/></a></span>
</template>

<style lang="scss" scoped>
a {
    display: inline-block;
    position: relative;
    box-sizing: border-box;
    height: 40px;
    line-height: 40px;
    margin: 0px 32px 0px 12px;
    padding: 0px 1rem;
    /* Not using the Sass variables since the SVG colors are fixed */
    color: currentcolor;
    background-color: #087A91;
    border-top: 1px solid #1B4A54;
    border-bottom: 1px solid #1B4A54;
}

a::before {
    position: absolute;
    top: -1px;
    left: -8px;
    content: url(~/assets/img/button-arrow-rear.svg);
}

a::after {
    position: absolute;
    top: -1px;
    right: -27px;
    content: url(~/assets/img/button-arrow-point.svg);
}
</style>

<script>
export default {
    props: {
        title: {
            type: String,
            required: false,
            default: "",
        },
    }
}

</script>
