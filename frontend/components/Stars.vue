<template>
<div class="stars" @mouseleave="unhover">
  <svg v-for="i in filled" :viewBox="FilledStar.viewBox" @click.stop="click(i)" @mouseover="hover(i)">
    <use :xlink:href="'#' + FilledStar.id" />
  </svg>
  <svg v-for="i in empty" :viewBox="EmptyStar.viewBox" @click.stop="click(i + filled)" @mouseover="hover(i + filled)">
    <use :xlink:href="'#' + EmptyStar.id" />
  </svg>
</div>
</template>

<script>
import FilledStar from '~/assets/img/star-on.svg?sprite'
import EmptyStar from '~/assets/img/star-off.svg?sprite'

export default {
    components: {
    },

    props: {
        value: {
            type: Number,
            required: false,
            default: 0
        },

        editable: {
            type: Boolean,
            required: false,
            default: false
        }
    },

    data() {
        return {
            EmptyStar,
            FilledStar,

            selecting: null,
        }
    },

    computed: {
        filled() {
            if(this.selecting !== null) {
                return this.selecting;
            }
            else {
                return Math.round(this.value);
            }
        },

        empty() {
            return 5 - this.filled;
        }
    },

    methods: {
        click(val) {
            if(this.editable) {
                this.$emit('input', val)
            }
        },

        hover(val) {
            if(this.editable) {
                this.selecting = val;
            }
        },

        unhover() {
            this.selecting = null;
        }
    }
}
</script>

<style lang="scss" scoped>
.stars {
    display: inline-block;
    vertical-align: middle;

    svg {
        display: inline-block;
        width: 20px;
        height: 20px;
    }
}
</style>
