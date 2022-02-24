// deno-lint-ignore-file camelcase
import Vue from "vue";

function block_key(language, group, item, inline, removeParagraphs, fixLinks) {
  return "dyn:" + language + ":" + group + ":" + item + ":" + inline + ":" +
    removeParagraphs + ":" + fixLinks;
}

export const state = () => ({
  dynamic_blocks: {},

  user: {
    authenticated: false,
  },

  auth: {},

  language: "en",

  partners: [],

  topics: [],

  descriptors: {},

  here: null,

  last_search: null,
});

export const mutations = {
  save_dynamic_block(state, block) {
    Vue.set(
      state.dynamic_blocks,
      block_key(
        block.language,
        block.group,
        block.item,
        block.inline,
        block.remove_paragraphs,
        block.fix_links,
      ),
      block,
    );
  },

  save_user(state, user) {
    state.user = user;
    if (user.authenticated) {
      state.auth = {
        headers: {
          "Authorization": "Bearer " + user.token,
        },
        withCredentials: true,
      };
    } else {
      state.auth = {};
    }
  },

  set_user_reports_pending(state, num) {
    if (!state.user.authenticated) {
      return;
    }

    state.user.reports_pending = num;
  },

  increment_user_reports_pending(state) {
    if (!state.user.authenticated) {
      return;
    }

    state.user.reports_pending += 1;
  },

  save_language(state, language) {
    state.language = language;
  },

  save_partners(state, partners) {
    state.partners = partners;
  },

  save_topics(state, topics) {
    state.topics = topics;
  },

  save_descriptors(state, descriptors) {
    state.descriptors = descriptors;
  },

  here(state, place) {
    state.here = place;
  },

  set_last_search(state, search) {
    state.last_search = JSON.parse(JSON.stringify(search));
  },
};

export const actions = {
  get_language({ commit, state }) {
    // Load language preference here if we decide to support
    // multiple site translations.
    return state.language || "en";
  },

  async get_here({ commit, state }) {
    if (state.here !== null) {
      return state.here;
    }

    if (process.server) {
      return {};
    }

    if (!this.$geolocation.checkSupport()) {
      return {};
    }

    let pos, places;

    try {
      pos = await this.$geolocation
        .getCurrentPosition();
    } catch (error) {
      console.log(error);
      return {};
    }

    try {
      places = await this.$axios.$post("/api/ui/finder/geo", {
        lookup: "near",
        place: {
          longitude: pos.coords.longitude,
          latitude: pos.coords.latitude,
          near: "",
          proximity: 0,
        },
      });
    } catch (error) {
      console.log(error);
      return {};
    }

    if (places.places.length == 0) {
      return {};
    }

    const place = {
      near: places.places[0].near,
      longitude: pos.coords.longitude,
      latitude: pos.coords.latitude,
      proximity: 40233,
    };

    commit("here", place);

    return place;
  },

  async get_dynamic_block(
    { commit, state },
    { language, group, item, inline, removeParagraphs, fixLinks },
  ) {
    const key = block_key(
      language,
      group,
      item,
      inline,
      removeParagraphs,
      fixLinks,
    );

    if (state.dynamic_blocks[key] === undefined) {
      // // Hopefully this approach will let us remove the Vue compiler and disable unsafe-* CSP directives
      // block = await import(/* webpackIgnore: true */"/api/ui/content-compiled?language=" + language + "&group=" + group + "&item=" + item + "&inline=" + inline + "&remove_paragraphs=" + removeParagraph + "&fix_links=" + fixLinks);
      // commit("save_dynamic_block", block);

      try {
        const block = await this.$axios.$get("/api/ui/content", {
          params: {
            language,
            group,
            item,
          },
        });

        block.inline = inline;
        block.remove_paragraphs = removeParagraphs;
        block.fix_links = fixLinks;

        commit("save_dynamic_block", block);
      } catch (_) {
        commit("save_dynamic_block", {
          language,
          group,
          item,
          tags: "",
          label: "",
          content: "",
          inline: inline,
          remove_paragraphs: removeParagraphs,
          fix_links: fixLinks,
        });
      }
    }

    return state.dynamic_blocks[key] || null;
  },

  async login(
    { commit, dispatch, state },
    { email, password, next, next_query, via },
  ) {
    let user = { authenticated: false };

    let endpoint = "/api/ui/auth/login";

    if (via === "scistarter") {
      endpoint = "/api/ui/auth/login-scistarter";
    }

    try {
      user = await this.$axios.$post(endpoint, {
        email,
        password,
      });
    } catch (error) {
      console.warn(error);
      return {
        authenticated: false,
        message: error.response
          ? error.response.data
          : "invalid email or password",
      };
    }

    if (process.client) {
      if (user.authenticated) {
        window.localStorage.setItem("token", user.token);
      } else {
        window.localStorage.removeItem("token");
      }
    }

    commit("save_user", user);

    if (next) {
      if (next.startsWith("/api/")) {
        if (process.client) {
          window.location = next;
        }
      } else if (next[0] == "/") {
        this.$router.push({ path: next, query: next_query || {} });
      } else {
        this.$router.push({ name: next, query: next_query || {} });
      }
    } else {
      this.$router.go(0);
    }

    dispatch("sync_local_to_server");

    return user;
  },

  async signup(
    { commit, dispatch, state },
    {
      email,
      username,
      password,
      zip_code,
      phone,
      agree,
      newsletter,
      next,
      next_query,
    },
  ) {
    if (!agree) {
      return { authenticated: false };
    }

    const params = {
      email,
      password,
    };

    if (username) {
      params.username = username;
    }

    if (zip_code) {
      params.zip_code = zip_code;
    }

    if (phone) {
      params.phone = phone;
    }

    if (newsletter) {
      params.newsletter = newsletter;
    }

    let user = { authenticated: false };

    try {
      user = await this.$axios.$post("/api/ui/auth/signup", params);
    } catch (error) {
      console.error(error);
      return { authenticated: false };
    }

    if (process.client) {
      if (user.authenticated) {
        window.localStorage.setItem("token", user.token);
      } else {
        window.localStorage.removeItem("token");
      }
    }

    commit("save_user", user);

    if (next) {
      if (next.startsWith("/api/")) {
        if (process.client) {
          window.location = next;
        }
      } else if (next[0] == "/") {
        this.$router.push({ path: next, query: next_query || {} });
      } else {
        this.$router.push({ name: next, query: next_query || {} });
      }
    } else {
      this.$router.go(0);
    }

    dispatch("sync_local_to_server");

    return user;
  },

  async logout({ commit, state }, goto) {
    let user = state.user;

    try {
      user = await this.$axios.$post("/api/ui/auth/logout");
    } catch (error) {
      console.error(error);
      return { authenticated: false };
    }

    if (process.client) {
      if (user.authenticated) {
        window.localStorage.setItem("token", user.token);
      } else {
        window.localStorage.removeItem("token");
      }
    }

    commit("save_user", user);

    if (goto) {
      const { next, next_query } = goto;
      if (next) {
        this.$router.push({ name: next, query: next_query || {} });
      } else {
        this.$router.go(0);
      }
    } else {
      this.$router.go(0);
    }

    return user;
  },

  async get_user({ commit, dispatch, state }) {
    let token = null;

    let user = {
      authenticated: false,
    };

    // The cookie and localStorage value are saved on the client
    // by the refresh_user plugin.
    if (process.server) {
      if (process.env.NODE_ENV == "development") {
        token = this.$cookies.get("token");
      } else {
        token = this.$cookies.get("__Host-token");
      }
    } else if (process.client) {
      token = window.localStorage.getItem("token");
    }

    if (state.user.authenticated) {
      user = state.user;
    } else if (token) {
      try {
        user = await this.$axios.$get("/api/ui/auth/me", {
          headers: {
            Authorization: "Bearer " + token,
          },
          withCredentials: true,
        });
      } catch (error) {}
    }

    commit("save_user", user);

    dispatch("sync_local_to_server");

    return user;
  },

  async get_partners({ commit, state }) {
    if (state.partners.length > 0) {
      return state.partners;
    }

    const partners = await this.$axios.$get("/api/ui/finder/partners");

    commit("save_partners", partners);

    return partners;
  },

  async get_topics({ commit, state }) {
    if (state.topics.length > 0) {
      return state.topics;
    }

    const topics = await this.$axios.$get("/api/ui/finder/topics");

    commit("save_topics", topics);

    return topics;
  },

  async get_descriptors({ commit, state }) {
    if (state.descriptors.length > 0) {
      return state.descriptors;
    }

    const descriptors = await this.$axios.$get("/api/ui/finder/descriptors");

    commit("save_descriptors", descriptors);

    return descriptors;
  },

  async sync_local_to_server({ commit, dispatch, state }) {
    if (process.server || !state.user.authenticated) {
      return;
    }

    let local = await dispatch("get_local");

    // sync "I did this" records
    let didit = local.didit || [];
    try {
      for (let slug of didit) {
        await this.$axios.$post("/api/ui/entity/" + slug + "/didit", {}, {
          withCredentials: true,
        });
      }
      local.didit = [];
    } catch (error) {
      // abort and try again later
      return;
    }

    await dispatch("set_local", local);
  },

  // Doesn't actually operate on the store; it's here because it's
  // logically related.
  async get_local({ commit, state }) {
    if (process.client) {
      return JSON.parse(window.localStorage.getItem("local_state") || "{}");
    } else {
      return {};
    }
  },

  // Doesn't actually operate on the store; it's here because it's
  // logically related.
  async set_local({ commit, state }, local) {
    if (process.client) {
      window.localStorage.setItem("local_state", JSON.stringify(local));
    }
  },
};
