<template>
  <button class="action-button" :class="{'principal': principal, 'primary': primary, 'secondary': secondary, 'tertiary': tertiary, 'contrast-bg': contrastBg, 'contrast-fg': contrastFg}" :disabled="disabled" @click="$emit('click')">
    <slot />
  </button>
</template>

<script>
export default {
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
        }
    }
}
</script>

<style lang="scss" scoped>
button.action-button {
    font-family: $snm-font-content;
    font-size: 16px;
    border: 1px solid $snm-color-element-light;
    border-radius: 6px;
    margin: 10px 0.5rem;
    padding: 10px;
    box-shadow: 0px 3px 6px $snm-color-shadow;
    font-weight: bold;
    color: $snm-color-background-meddark;
    background-color: $snm-color-element-light;
    flex-shrink: 1;
    flex-grow: 1;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    padding: 0.75rem 1.5rem;

    svg {
        &:first-child {
            position: relative;
            left: -0.7rem;
        }

        * {
            fill: currentColor;
        }
    }

    &:hover,&:active {
        background-color: $snm-color-background-meddark;
        color: $snm-color-element-light;
    }

    &.contrast-bg {
        background-color: $snm-color-background-medium;
    }

    &.contrast-fg {
        border-color: $snm-color-background-medium;
    }

    &.tertiary {
        color: $snm-color-element-med;
        border-color: $snm-color-element-med;
        background-color: $snm-color-element-light;
    }

    &.principal {
        border-color: $snm-color-action-border;
        background-color: $snm-color-action;
        color: $snm-color-element-dark;
        flex-grow: 2;
        flex-shrink: 0;

        &:hover,&:active {
            background-color: $snm-color-heading-ondark;
        }

        &:disabled {
            background-color: $snm-color-background-light;
            color: $snm-color-disabled;
            border-color: $snm-color-disabled;
        }
    }

    &.primary {
        border-color: $snm-color-background-dark;
        background-color: $snm-color-element-med;
        color: $snm-color-element-light;
        flex-grow: 2;
        flex-shrink: 0;

        &:hover,&:active {
            color: $snm-color-element-med;
            background-color: $snm-color-element-light;
        }

        &.contrast-bg {
            background-color: $snm-color-background-meddark;
        }

        &.contrast-fg {
            border-color: $snm-color-background-meddark;
        }

        &:disabled {
            background-color: $snm-color-background-light;
            color: $snm-color-disabled;
            border-color: $snm-color-disabled;
        }
    }

    &.secondary {
        border-color: $snm-color-action;
        background-color: $snm-color-background;
        color: $snm-color-element-dark;
        border-radius: 10px;
    }

    &:disabled {
        background-color: $snm-color-background-light;
        color: $snm-color-disabled;
        border-color: $snm-color-disabled;
    }
}
</style>
