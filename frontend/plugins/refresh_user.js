// This is to make sure the client keeps an up-to-date cookie and
// localStorage value for the user.
window.onNuxtReady(async () => {
    // This version also updates the Vuex store. Should not be needed,
    // since the Vuex store will normally have been loaded during SSR,
    // and will be explictly updated if needed by page interactions.

    // let user = await window.$nuxt.$store.dispatch('get_user');

    // This version just updates the token in cookie and localStorage
    const user = await window.$nuxt.$axios.$get("/api/ui/me", {
        headers: {
            "Authorization": "Bearer " + window.localStorage.getItem('token'),
        },
    });

    // Needed for either version
    if(user.authenticated) {
        window.localStorage.setItem('token', user.token);
    }
    else {
        window.localStorage.removeItem('token');
    }
});