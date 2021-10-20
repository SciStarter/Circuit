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
    if (window.isSecureContext) {
      const hashed = btoa(
        String.fromCharCode.apply(
          null,
          new Uint8Array(
            await crypto.subtle.digest(
              "SHA-256",
              new ArrayBuffer(user.token + Math.random().toString().slice(2)),
            ),
          ),
        ),
      );
      window.localStorage.setItem("session", hashed);
    } else {
      window.localStorage.setItem("session", issue_tag());
    }
  } else {
    window.localStorage.removeItem("token");
    window.localStorage.setItem("session", issue_tag());
  }
});
