<template>
  <section class="section">
    <div class="columns is-mobile">
      <card title="Free" icon="github">
        Open source on <external href="https://github.com/buefy/buefy">GitHub</external>
      </card>

      <card title="Responsive" icon="cellphone-link">
        <b class="has-text-grey">Every</b> component is responsive
      </card>

      <card title="Modern" icon="alert-decagram">
        Built with <external href="https://vuejs.org/">Vue.js</external> and <external href="http://bulma.io/">Bulma</external>
      </card>

      <card title="Lightweight" icon="arrange-bring-to-front">
        No other internal dependency
      </card>
    </div>
  </section>
</template>

<script>
import Card from '~/components/Card'
import External from '~/components/External'

export default {
  name: 'HomePage',

  components: {
      Card,
      External
  }
}
</script>
