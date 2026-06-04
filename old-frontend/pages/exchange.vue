<template>
  <div class="exchange">
    <nuxt-child />
    <PageView/>
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
