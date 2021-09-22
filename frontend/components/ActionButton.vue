<template>
  <button class="action-button" :class="{'principal': principal, 'primary': primary, 'secondary': secondary, 'tertiary': tertiary, 'contrast-bg': contrastBg, 'contrast-fg': contrastFg, 'arrow': arrow}" :disabled="disabled" @click="$emit('click')">
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
    }
}
</script>

<style lang="scss" scoped>
button.action-button {
    --forground: #{$snm-color-background-meddark};
    --background: #{$snm-color-element-light};
    --border: #{$snm-color-element-light};

    position: relative;
    font-family: $snm-font-content;
    font-size: $snm-font-small;
    border: 1px solid var(--border);
    border-radius: 6px;
    margin: 10px 0.5rem;
    padding: 10px;
    box-shadow: 0px 3px 6px $snm-color-shadow;
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
    height: 3rem;

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
            --background: #{$snm-color-background-medlight};
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
        --background: #{$snm-color-element-med};
        --foreground: #{$snm-color-element-light};

        &:hover,&:active {
            --background: #{$snm-color-background-medium};
        }

        &.contrast-bg {
            --background: #{$snm-color-background-meddark};
        }

        &.contrast-fg {
            --border: #{$snm-color-background-meddark};
        }

        &:disabled {
            --background: #{$snm-color-background-light};
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
        --border: #{$snm-color-disabled};
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
        top: 0.355rem;
        right: -1rem;
        width: 2.25rem;
        height: 2.25rem;
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
</style>
