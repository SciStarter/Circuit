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
  <h1><dynamic-block group="homepage" item="demo-header">The contents of a &lt;dynamic-block&gt; tag are displayed if the dynamic content can not be retrieved. If the dynamic content can't be retrieved <em>and</em> the tag has no contents, lorem ipsum is displayed.</dynamic-block></h1>
  <!-- Dynamic blocks are rendered as div elements, but you can use a class to change how they are displayed -->
  <dynamic-block group="homepage" item="demo-dynamic" class="demo"></dynamic-block>
</section>
</template>

<style>
h1 {
    font-weight: bold;
    font-size: 120%;
}

.demo {
    color: #999;
}
</style>

<script>
import Card from '~/components/Card'
import External from '~/components/External'
import DynamicBlock from '~/components/DynamicBlock'

export default {
  name: 'HomePage',

  components: {
      Card,
      External,
      DynamicBlock
  }
}
</script>
