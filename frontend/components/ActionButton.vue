<template>
  <button ref="actionBtn" type="button" class="action-button" :class="{'principal': principal, 'primary': primary, 'secondary': secondary, 'tertiary': tertiary, 'contrast-bg': contrastBg, 'contrast-fg': contrastFg, 'arrow': arrow, 'large': large, 'text':text, 'gray':gray, 'tight':tight, 'text2':text2, 'red':red, 'icon-only':iconOnly}" :disabled="disabled" @click="$emit('click')">
    <slot />
  </button>
</template>

<script>
export default {
    name: "ActionButton",

    props: {
        principal: {
            type: Boolean,
            required: false,
            default: false
        },

        primary: {
            type: Boolean,
            required: false,
            default: false
        },

        secondary: {
            type: Boolean,
            required: false,
            default: false
        },

        tertiary: {
            type: Boolean,
            required: false,
            default: false,
        },

        contrastBg: {
            type: Boolean,
            required: false,
            default: false
        },

        contrastFg: {
            type: Boolean,
            required: false,
            default: false
        },

        disabled: {
            type: Boolean,
            required: false,
            default: false,
        },

        arrow: {
            type: Boolean,
            required: false,
            default: false,
        },
        large: {
            type: Boolean,
            required: false,
            default: false,
        },
        text: {
            type: Boolean,
            required: false,
            default: false,
        },
        gray: {
            type: Boolean,
            required: false,
            default: false,
        },
        tight: {
            type: Boolean,
            required: false,
            default: false,
        },
        text2: {
            type: Boolean,
            required: false,
            default: false,
        },
        red: {
            type: Boolean,
            required: false,
            default: false,
        },
        iconOnly: {
            type: Boolean,
            required: false,
            default: false,
        },
    },

    methods: {
        blur() {
            this.$refs.actionBtn.blur();
        }
    },
}
</script>

<style lang="scss" scoped>
button.action-button {
    --forground: #{$snm-color-background-meddark};
    --background: #{$snm-color-element-light};
    --border: #{$snm-color-element-light};

    position: relative;
    font-family: $snm-font-content;
    font-size: 0.85rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    margin: 10px 0.5rem;
    padding: 10px;
    // box-shadow: 0px 3px 6px $snm-color-shadow;
    font-weight: bold;
    color: var(--foreground);
    background-color: var(--background);
    flex-shrink: 0;
    flex-grow: 0;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem 1.5rem;
    box-sizing: border-box;
    height: rem(40px);
    line-height: 1;

    svg {
        &:first-child {
            position: relative;
            left: -0.7rem;
        }

        * {
            fill: var(--foreground);
        }
    }

    &:hover,&:active {
        --background: #{$snm-color-background-meddark};
        --foreground: #{$snm-color-element-light};
    }

    &.contrast-bg {
        --background: #{$snm-color-background-medium};
    }

    &.contrast-fg {
        --border: #{$snm-color-background-medium};
    }

    &.tertiary {
        --foreground: #{$snm-color-element-med};
        --border: #{$snm-color-element-med};
        --background: #{$snm-color-element-light};

        &:hover,&:active {
            --foreground: #{$snm-color-element-light};
            --border: #{$snm-color-background-dark};
            --background: #{$snm-color-background-meddark};
        }
    }

    &.principal {
        --border: #{$snm-color-action-border};
        --background: #{$snm-color-action};
        --foreground: #{$snm-color-element-dark};

        &:hover,&:active {
            --background: #{$snm-color-heading-ondark};
        }

        &:disabled {
            --background: #{$snm-color-background-light};
            --foreground: #{$snm-color-disabled};
            --border: #{$snm-color-disabled};
        }
    }

    &.primary {
        --border: #{$snm-color-background-dark};
        --background: #{$snm-color-background-meddark};
        --foreground: #{$snm-color-element-light};

        &:hover,&:active {
            --background: #{$snm-color-background-dark};
        }

        &.contrast-bg {
            --background: #{$snm-color-background-meddark};
        }

        &.contrast-fg {
            --border: #{$snm-color-background-meddark};
        }

        &:disabled {
            --background: #{$snm-color-background-disabled};
            --foreground: #{$snm-color-disabled};
            --border: #{$snm-color-disabled};
        }
    }

    &.secondary {
        --border: #{$snm-color-action};
        --background: #{$snm-color-background};
        --foreground: #{$snm-color-element-dark};
        border-radius: 10px;
    }

    &:disabled {
        --background: #{$snm-color-background-light};
        --foreground: #{$snm-color-disabled};
        --border: #{$snm-color-background-light};
    }

    &.arrow {
        border-top-right-radius: 4px;
        border-bottom-right-radius: 4px;
        margin-right: 2rem;
    }

    &.arrow::after {
        content: "";
        display: block;
        position: absolute;
        top: 0.255rem;
        right: -0.8rem;
        width: 30px;
        height: 30px;
        border-top: 1px solid var(--border);
        border-right: 1px solid var(--border);
        background-color: var(--background);
        transform: rotate(50grad);
        border-radius: 4px;
        border-top-right-radius: 6px;
        clip-path: polygon(0 0, 100% 100%, 100% 0);
        box-sizing: border-box;
    }

    &.large {
      height: 48px;
      &.arrow::after {
        top: 0.355rem;
        right: -1rem;
        width: 34px;
        height: 34px;
        content: "";
        display: block;
        position: absolute;
        border-top: 1px solid var(--border);
        border-right: 1px solid var(--border);
        background-color: var(--background);
        transform: rotate(50grad);
        border-radius: 4px;
        border-top-right-radius: 6px;
        clip-path: polygon(0 0, 100% 100%, 100% 0);
        box-sizing: border-box;
      }
    }

    &.text {
      border: 0;
      background-color:transparent;
      box-shadow:none;
      font-weight:normal;
      color: $snm-color-element-med;
      padding:0;
      margin:0;
      width:auto;
      height:auto;
    }
    &.text2 {
      border: 0;
      background-color:transparent;
      box-shadow:none;
      font-weight:normal;
      color: $snm-color-element-med;
      padding:0;
      margin:0;
      width:auto;
      height:auto;
      text-decoration: underline;

      &:hover {
        color: $snm-color-background-meddark;
      }
    }

    &.gray {
        --border: #696969;
        --background: #696969;
        --foreground: #fff;

        &:hover,&:active {
            --background: #484848;
        }

        &:disabled {
            --background: #{$snm-color-background-light};
            --foreground: #{$snm-color-disabled};
            --border: #{$snm-color-disabled};
        }
    }

    &.tight {
      padding:4px 8px;
      box-shadow:none;
      height:auto;
      margin-left:0;

      .icon {
        margin-right:8px;
        height:auto;
        width:auto;
      }
      .icon svg {
        width:12px;
        height:auto;
        left:auto;

        path {
          fill: #fff;
        }
      }
    }

    &.primary.red {
        --border: #640C00;
        --background: #{$snm-color-info};
        --foreground: #{$snm-color-element-light};

        &:hover,&:active {
            --background:#640C00;
        }
      }

      &.tertiary.red {
          --border: #{$snm-color-info};
          --background: #fff;
          --foreground: #{$snm-color-info};

          &:hover,&:active {
              --background:#{$snm-color-info};
              --foreground: #fff;
          }
        }

      &.icon-only {
        padding:10px;
        .icon svg {
          padding-right:0!important;
          left:0!important;
        }

      }

}
</style>
