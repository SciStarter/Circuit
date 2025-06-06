// This is to make sure the client keeps an up-to-date cookie and
// localStorage value for the user.
window.onNuxtReady(async () => {
  // This version also updates the Vuex store. Should not be needed,
  // since the Vuex store will normally have been loaded during SSR,
  // and will be explictly updated if needed by page interactions.

  // let user = await window.$nuxt.$store.dispatch('get_user');

  // This version just updates the token in cookie and localStorage
  const user = await window.$nuxt.$axios.$get("/api/ui/auth/me", {
    headers: {
      Authorization: "Bearer " + window.localStorage.getItem("token"),
    },
  });

  function issue_tag() {
    const now = new Date();
    return "tag:sciencenearme.org," + now.getFullYear() + "-" + now.getMonth() +
      "-" + now.getDate() + ":" + now.getHours() + "" + now.getMinutes() +
      Math.random().toString().slice(2);
  }

  // Needed for either version
  if (user.authenticated) {
    window.localStorage.setItem("token", user.token);
    window.localStorage.setItem("session", user.uid);
  } else {
    window.localStorage.removeItem("token");
    if(!window.localStorage.getItem("session")) {
        window.localStorage.setItem("session", issue_tag());
    }
  }

  window.dataLayer = window.dataLayer || [];
  window.dataLayer.unshift({user_id: window.localStorage.getItem("session")});
  window.$nuxt.$gtm.init("GTM-PNHMH6L");
});
