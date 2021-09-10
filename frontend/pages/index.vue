<template>
<section class="section">
  <h1>
    <dynamic-block group="homepage" item="demo-header" remove-paragraphs>
      The contents of a &lt;dynamic-block&gt; tag are displayed if the dynamic content can not be retrieved. If the dynamic content can't be retrieved <em>and</em> the tag has no contents, lorem ipsum is displayed.
    </dynamic-block>
  </h1>
  <!-- Dynamic blocks are rendered as div elements, but you can use classes to change how they are displayed. -->
  <!-- The "content" class is provided by Bulma, and often makes WYSIWYG-created content look better-->
  <dynamic-block group="homepage" item="demo-dynamic" class="content demo" />

  <p>
    Logged in user: {{ username }}
  </p>
</section>
</template>

<script>
// import Card from '~/components/Card'
// import External from '~/components/External'
import DynamicBlock from '~/components/DynamicBlock'

export default {
    name: 'HomePage',

    components: {
        // Card,
        // External,
        DynamicBlock
    },

    data () {
        return {
            show_login: false,
            show_signup: false
        }
    },

    head() {
        return {
            'meta': [
                { hid: 'og:image', property: 'og:image', content: require('~/assets/img/logo.jpg') },
            ]
        };
    },

    computed: {
        username () {
            return this.$store.state.user.username;
        }
    },
}
</script>

<style>
h1 {
    font-weight: bold;
    font-size: 120%;
}

.demo {
    color: #999;
}
</style>
