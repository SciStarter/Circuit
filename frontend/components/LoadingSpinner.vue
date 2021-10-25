<template />

<script>
export default {
    name: "LoadingSpinner",

    data() {
        return {
            current: null,
        };
    },

    methods: {
        start() {
            this.current = this.$buefy.loading.open({ container: null });
        },

        finish() {
            this.current.close();
            this.current = null;
        }
    },
}
</script>
