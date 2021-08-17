<template>
<section class="section">
  <h1><dynamic-block group="homepage" item="demo-header" remove-paragraphs>The contents of a &lt;dynamic-block&gt; tag are displayed if the dynamic content can not be retrieved. If the dynamic content can't be retrieved <em>and</em> the tag has no contents, lorem ipsum is displayed.</dynamic-block></h1>
  <!-- Dynamic blocks are rendered as div elements, but you can use classes to change how they are displayed. -->
  <!-- The "content" class is provided by Bulma, and often makes WYSIWYG-created content look better-->
  <dynamic-block group="homepage" item="demo-dynamic" class="content demo"></dynamic-block>

  <p>
    Logged in user: {{username}}
  </p>
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
        DynamicBlock,
    },

    data() {
        return {
            show_login: false,
            show_signup: false,
        };
    },

    computed: {
        username() {
            return this.$store.state.user.username;
        }
    },

    async fetch() {
        await this.$store.dispatch('get_user');
    },
}
</script>
