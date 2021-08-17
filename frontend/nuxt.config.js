const DOMAIN = "beta.sciencenearme.org";

export default {
    // Global page headers: https://go.nuxtjs.dev/config-head
    head: {
        title: 'frontend',
        htmlAttrs: {
            lang: 'en'
        },
        meta: [
            { charset: 'utf-8' },
            { name: 'viewport', content: 'width=device-width, initial-scale=1' },
            { hid: 'description', name: 'description', content: "Find opportunities to do real science, anywhere." }
        ],
        link: [
            { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' },
            { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
            { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: true },
            { rel: 'stylesheet', type: 'text/css', href: 'https://fonts.googleapis.com/css2?family=Cabin:ital,wght@0,400;0,700;1,400&family=Fira+Sans:ital,wght@0,400;0,700;1,400&family=Roboto:ital,wght@0,400;0,700;1,400&display=swap' },
        ]
    },

    // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
    plugins: [
        {src: "~/plugins/refresh_user.js", mode: "client"},
        {src: "~/plugins/clickstream.js", mode: "client"},
    ],

    // Auto import components: https://go.nuxtjs.dev/config-components
    components: true,

    // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
    buildModules: [
        //['@nuxtjs/eslint-module', { emitWarning: true, emitError: false }],
        '@nuxtjs/style-resources',
        "@nuxtjs/svg",
    ],

    // Modules: https://go.nuxtjs.dev/config-modules
    modules: [
        // https://go.nuxtjs.dev/buefy
        ['nuxt-buefy', { css: false }],
        // https://go.nuxtjs.dev/axios
        '@nuxtjs/axios',
        // https://www.npmjs.com/package/cookie-universal-nuxt
        'cookie-universal-nuxt',
        '@nuxtjs/gtm',
        '@nuxtjs/proxy',
    ],

    css: ['@/assets/vars/buefy.scss'],

    styleResources: {
        scss: [
            './assets/vars/*.scss',
            './assets/abstracts/_mixins.scss'
        ]
    },


    gtm: {
        id: 'GTM-5ZT2954'
    },

    // Build Configuration: https://go.nuxtjs.dev/config-build
    build: {
        babel: {
            plugins: [
                ["@babel/plugin-proposal-private-property-in-object", { "loose": true }],
                ["@babel/plugin-proposal-private-methods", { "loose": true }],
                ["@babel/plugin-proposal-class-properties", { "loose": true }],
            ]
        }
    },

    // In production, these requests won't normally make it to the
    // Nuxt server, so this is mostly for local development.
    proxy: {
        "/api": process.env.LOCAL_API_URL || "https://" + DOMAIN,
    },

    // Axios module configuration: https://go.nuxtjs.dev/config-axios
    axios: {
        baseURL: "https://" + DOMAIN,
    },

    // These two sections override config values at runtime
    publicRuntimeConfig: {
        axios: {
            browserBaseURL: process.env.LOCAL_API_URL ? "/" : "https://" + DOMAIN,
        }
    },

    privateRuntimeConfig: {
        axios: {
            baseURL: process.env.LOCAL_API_URL || "http://" + process.env.CIRCUIT_API_SERVICE_BETA_SERVICE_HOST + ":" + process.env.CIRCUIT_API_SERVICE_BETA_SERVICE_PORT,
        }
    }
}
