export default {
  //__fake__: console.log(process.env),

  prettify: false,

  server: {
    host: "::",
    port: 3000,
  },

  loading: "~/components/LoadingSpinner.vue",

  // Global page headers: https://go.nuxtjs.dev/config-head
  head: {
    title: "Science Near Me",

    link: [
      { rel: "icon", type: "image/x-icon", href: "/favicon.ico" },
      { rel: "preconnect", href: "https://fonts.googleapis.com" },
      {
        rel: "preconnect",
        href: "https://fonts.gstatic.com",
        crossorigin: true,
      },
      {
        rel: "stylesheet",
        type: "text/css",
        href:
          "https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,400;0,700;1,400&family=Roboto:ital,wght@0,400;0,700;1,400&display=swap",
      },
    ],
    htmlAttrs: {
      lang: "en",
    },
    meta: [
      { charset: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      {
        hid: "description",
        name: "description",
        content: "Find opportunities to do real science, anywhere.",
      },
      {
        hid: "og:description",
        property: "og:description",
        content: "Find opportunities to do real science, anywhere.",
      },
      { hid: "og:title", property: "og:title", content: "Science Near Me" },
      {
        hid: "og:url",
        property: "og:url",
        content: "https://sciencenearme.org/",
      },
      { hid: "og:type", property: "og:type", content: "website" },
    ],
  },
  modern: "client",

  render: {
    csp: {
      //addMeta: true,
      hashAlgorithm: "sha256",
      policies: {
        "default-src": ["'none'"],
        "script-src": [
          "'self'",
          "'unsafe-eval'", // Needed to compile dynamic content into Vue components. TODO Investigate alternatives.
          "'unsafe-inline'", // Needed for bootstrapping the root Vue component. 'strict-dynamic' with addMeta=true would also work, but not alongside 'unsafe-eval'
          "https://sciencenearme.org/",
          "www.googletagmanager.com",
          "www.google-analytics.com",
          "secure.quantserve.com",
          "rules.quantcount.com",
        ],
        "style-src": [
          "'self'",
          "'unsafe-inline'",
          "fonts.googleapis.com",
          "cdn.jsdelivr.net",
        ],
        "img-src": ["'self'", "https:", "data:", "blob:", "region1.google-analytics.com", "region1.analytics.google.com", "pixel.quantserve.com"],
        "connect-src": [
          "'self'",
          "data:",
          "https://sciencenearme.org/",
          "https://*.tiles.mapbox.com",
          "https://api.mapbox.com",
          "https://events.mapbox.com",
          "www.google-analytics.com",
          "https://geocode.arcgis.com",
          "region1.google-analytics.com",
          "region1.analytics.google.com",
        ],
        "font-src": ["'self'", "fonts.gstatic.com", "cdn.jsdelivr.net"],
        "object-src": ["'self'"],
        "media-src": ["'self'"],
        "frame-src": ["'self'"], // Overridden by child-src if the browser supports CSP 3 (Safari doesn't yet)
        "child-src": ["'self'", "blob:"],
        "form-action": ["'self'"],
        //"frame-ancestors": ["'none'"], // Need a way to override this for the widget and exchange pages specifically
        "plugin-types": ["'none'"],
        "base-uri": ["'self'"],
        "worker-src": ["'self'", "blob:"],
        "manifest-src": ["'self'"],
        "prefetch-src": ["'self'"],
      },
    },
  },

  // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
  plugins: [
    { src: "~/plugins/refresh_user.js", mode: "client" },
    { src: "~/plugins/clickstream.js", mode: "client" },
  ],

  // Auto import components: https://go.nuxtjs.dev/config-components
  components: true,

  // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
  buildModules: [
    ["@nuxtjs/eslint-module", { emitWarning: true, emitError: false }],
    "@nuxtjs/style-resources",
    "@nuxtjs/svg",
  ].concat(
    (process.env.NODE_ENV === "development")
      ? ["nuxt-build-optimisations"]
      : [],
  ),

  buildOptimisations: {
    profile: "risky",
  },

  // Modules: https://go.nuxtjs.dev/config-modules
  modules: [
    // https://go.nuxtjs.dev/buefy
    ["nuxt-buefy", { css: false, defaultProgrammaticPromise: true }],
    // https://go.nuxtjs.dev/axios
    "@nuxtjs/axios",
    // https://www.npmjs.com/package/cookie-universal-nuxt
    "cookie-universal-nuxt",
    "@nuxtjs/gtm",
    "@nuxtjs/proxy",
    "vue-geolocation-api/nuxt",
    "nuxt-custom-headers",
  ],

  css: ["@/assets/vars/buefy.scss"],

  styleResources: {
    scss: [
      "./assets/vars/*.scss",
      "./assets/abstracts/_mixins.scss",
      "./assets/utilities/*.scss",
    ],
  },

  // Build Configuration: https://go.nuxtjs.dev/config-build
  build: {
    extractCSS: (process.env.NODE_ENV === "development") ? false : true,

    babel: {
      plugins: [
        ["@babel/plugin-proposal-private-property-in-object", { loose: true }],
        ["@babel/plugin-proposal-private-methods", { loose: true }],
        ["@babel/plugin-proposal-class-properties", { loose: true }],
      ],
    },

    prettify: false,

    extend(config) {
      // Include the Vue compiler, so dynamic content can be
      // compiled into Vue components.
      config.resolve.alias["vue"] = "vue/dist/vue.common";
    },

    loaders: {
      vue: {
        prettify: false,
        compilerOptions: {
          preserveWhitespace: false,
          prettify: false,
        },
      },
    },
  },

  // In production, these requests won't normally make it to the
  // Nuxt server, so this is mostly for local development.
  proxy: {
    "/api": process.env.LOCAL_API_URL || "https://" + process.env.DOMAIN,
  },

  // Axios module configuration: https://go.nuxtjs.dev/config-axios
  axios: {
    baseURL: process.env.LOCAL_API_URL || "https://" + process.env.DOMAIN,
  },

  // These two sections override config values at runtime
  publicRuntimeConfig: {
    mapboxToken: process.env.MAPBOX_TOKEN,
    axios: {
      browserBaseURL: process.env.LOCAL_API_URL ? "/"
      : "https://" + process.env.DOMAIN,
    },
  },

  privateRuntimeConfig: {
    axios: {
      baseURL: process.env.LOCAL_API_URL ? process.env.LOCAL_API_URL
      : ((process.env.DOMAIN == "beta.sciencenearme.org")
        ? (
          "http://" + process.env.CIRCUIT_API_SERVICE_BETA_SERVICE_HOST +
          ":" +
          process.env.CIRCUIT_API_SERVICE_BETA_SERVICE_PORT
        )
        : ("http://" + process.env.CIRCUIT_API_SERVICE_SERVICE_HOST + ":" +
          process.env.CIRCUIT_API_SERVICE_SERVICE_PORT)),
    },
  },
};
