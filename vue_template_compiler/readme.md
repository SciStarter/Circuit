Nothing here yet. A Vue 2 template compiler implemented in Rust
appears to be needed in order to clean up the frontend's security
profile, with the added bonus that we could also reduce the bundle
size. It’ll have to wait for when we’re not in crunch time, though.

References:

https://vuejs.org/v2/guide/render-function.html
https://github.com/vuejs/vue/tree/dev/packages/vue-template-compiler

An alternative approach would be to do the compilation in the frontend
server, using the vue-template-compiler directly from within a Nuxt
custom endpoint, but while that is probably easier it seems less
efficient in the long term.

https://nuxtjs.org/docs/configuration-glossary/configuration-servermiddleware/#custom-api-endpoint
