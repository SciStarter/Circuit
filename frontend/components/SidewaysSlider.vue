<template>
<div class="sideways-slider">
  <div ref="scroll" class="slider-scroll">
    <div ref="items" class="slider-items">
      <slot />
    </div>
  </div>
  <button class="slide left no-mobile show-tablet" @click="scroll(-items_scroll(),$event)" ref="leftBtn" :class="{'hidden':leftBtnHidden}">
    <span>&lsaquo;</span>
  </button>
  <button class="slide right no-mobile show-tablet" @click="scroll(items_scroll(),$event)" ref="rightBtn" :class="{'hidden':rightBtnHidden}">
    <span>&rsaquo;</span>
  </button>
</div>
</template>

<script>
import { calc } from 'csscalc';

export default {
    name: "SidewaysSlider",
    data(){
      return {
        leftBtnHidden: true,
        rightBtnHidden: false
      }
    },

    methods: {
        items_width() {
            return this.$refs.items.clientWidth;
        },

        scroll_width() {
            return this.$refs.scroll.scrollWidth;
        },

        items_scroll() {
            return calc(this.items_width() + 'px + 1rem', this.$refs.items);
        },

        max_scroll() {
            return this.scroll_width() - this.items_width();
        },

        scroll(by, $event) {
            const to = Math.max(0, Math.min(this.$refs.scroll.scrollLeft + by, this.max_scroll()));

            const do_scroll = () => {
                const diff = to - this.$refs.scroll.scrollLeft;
                const amount = 0.1 * diff;

                if(-1 < amount && 1 > amount) {
                    this.$refs.scroll.scrollLeft = to;
                }
                else {
                    this.$refs.scroll.scrollLeft += amount
                    window.requestAnimationFrame(do_scroll);
                }
            }

            // if (this.$refs.)
            // this.$refs.leftBtn.classList.remove('no-more');
            // this.$refs.rightBtn.classList.remove('no-more');
            // this.target.classList.add('no-more');
            if (this.leftBtnHidden) {
              this.leftBtnHidden = false;
              this.rightBtnHidden = true;
            } else {
              this.leftBtnHidden = true;
              this.rightBtnHidden = false;
            }

            do_scroll();
        },
    },
}
</script>

<style lang="scss" scoped>
.sideways-slider {
    position: relative;

    .slider-scroll {
        max-width: 100%;
        overflow-x: scroll;
        scrollbar-width: none;
        padding-left: 1rem;

        &::-webkit-scrollbar {
            height: 0px;
        }

        .slider-items {
            display: flex;
            flex-direction: row;

            >* {
                flex-shrink: 0;
                flex-grow: 0;
                max-width: 70vw;
            }
        }
    }

    .slide {
        position: absolute;
        top: calc(50% - 1rem);
        font-family: $snm-font-content;
        font-size: 2rem;
        font-weight: bold;
        border: 0px;
        width: 3rem;
        color: $snm-color-element-dark;
        background-color: $snm-color-action;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        border-radius: 4px;
        cursor: pointer;

        >span {
            position: relative;
            top: -0.1rem;
        }

        &.left {
            left: -1rem;
        }

        &.right {
            right: -1rem;
        }
    }
}

@media (min-width:$tablet-screen) {

  .sideways-slider {
    margin: 1rem auto;
    width: calc(100% - 4rem);
  }
  .sideways-slider .slider-scroll{
    padding:0;
  }
  .show-tablet {
    display: block!important;
  }
  .hidden {
    display: none!important;
  }
}
</style>
