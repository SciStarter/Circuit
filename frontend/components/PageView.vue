<template><span ref="pv"></span></template>

<script>
export default {
    name: "PageView",
    async mounted() {
        let place = await this.$store.dispatch("get_here");

        window.$nuxt.$axios.$post("/api/ui/activity/view", {
            session: window.localStorage.getItem("session") || "",
            page: window.location.pathname + window.location.search + window.location.hash,
            longitude: place.longitude,
            latitude: place.latitude
        });  
    }
}
</script>
