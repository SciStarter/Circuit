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
            { hid: 'description', name: 'description', content: '' }
        ],
        link: [
            { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
        ]
    },

    // Global CSS: https://go.nuxtjs.dev/config-css
    css: [
    ],

    // Plugins to run before rendering page: https://go.nuxtjs.dev/config-plugins
    plugins: [
        {src: "~/plugins/clickstream.js", mode: "client"}
    ],

    // Auto import components: https://go.nuxtjs.dev/config-components
    components: true,

    // Modules for dev and build (recommended): https://go.nuxtjs.dev/config-modules
    buildModules: [
        // https://go.nuxtjs.dev/eslint
        '@nuxtjs/eslint-module',
    ],

    // Modules: https://go.nuxtjs.dev/config-modules
    modules: [
        // https://go.nuxtjs.dev/buefy
        'nuxt-buefy',
        // https://go.nuxtjs.dev/axios
        '@nuxtjs/axios',
        '@nuxtjs/gtm',
    ],

    gtm: {
        id: 'GTM-5ZT2954'
    },

    // Build Configuration: https://go.nuxtjs.dev/config-build
    build: {

    },

    // Axios module configuration: https://go.nuxtjs.dev/config-axios
    axios: {
        baseURL: "https://beta.sciencenearme.org"
    },

    // These two sections override config values at runtime
    publicRuntimeConfig: {
        axios: {
            browserBaseURL: process.env.LOCAL_API_URL || "https://beta.sciencenearme.org"
        }
    },

    privateRuntimeConfig: {
        axios: {
            baseURL: process.env.LOCAL_API_URL || "http://" + process.env.CIRCUIT_API_SERVICE_BETA_SERVICE_HOST + ":" + process.env.CIRCUIT_API_SERVICE_BETA_SERVICE_PORT
        }
    }
}
