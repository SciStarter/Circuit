<template>
  <b-loading v-model="active" />
</template>

<script>
export default {
    name: "LoadingSpinner",

    data() {
        return {
            active: false,
        };
    },

    methods: {
        start() {
            this.active = true;
        },

        finish() {
            this.active = false;
        }
    },
}
</script>
