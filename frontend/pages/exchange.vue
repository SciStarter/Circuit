<template>
  <div class="exchange">
    <nuxt-child />
  </div>
</template>

<script>
export default {
    name: "Exchange",

    layout: "exchange",

    httpHeaders() {
        return {
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },
}
</script>

