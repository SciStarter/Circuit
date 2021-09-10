<template>
<div class="read-more">
  <div ref="container" class="container" :class="{'closed': !value, 'togglable': closed_show_toggle}">
    <slot />
  </div>
  <a v-show="show_toggle" @click="$emit('input', !value)">read <span v-if="value">less</span><span v-else>more</span></a>
</div>
</template>

<script>
export default {
    props: {
        value: {
            type: Boolean,
            required: false,
            default: false,
        }
    },

    data() {
        return {
            closed_show_toggle: false,
        }
    },

    computed: {
        show_toggle() {
            return this.value || this.closed_show_toggle;
        }
    },

    mounted() {
        let container = this.$refs.container;
        this.closed_show_toggle = container.clientHeight >= parseFloat(window.getComputedStyle(container).maxHeight);
    },
}
</script>

<style lang="scss" scoped>
.read-more {
    > .container {
        position: relative;
        overflow: hidden;

        &.closed {
            max-height: 8rem;

            &.togglable:after {
                content  : "";
                position : absolute;
                z-index  : 1;
                bottom   : 0;
                left     : 0;
                pointer-events   : none;
                background-image : linear-gradient(to bottom, change-color($snm-color-background, $alpha: 0), $snm-color-background 90%);
                width    : 100%;
                height   : 2rem;
            }
        }
    }
}
</style>
