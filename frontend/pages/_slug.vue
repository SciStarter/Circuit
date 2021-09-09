<template>
<div>
  <!-- Selects a top level component to display based on the data pulled from the server -->
  <!-- If we add more top level components, they need to be added between just-content and opportunity, with a v-else-if="LOGIC" attribute, and should use the same @login and @signup handlers -->
  <just-content v-if="layout == 'just_content'" :entity="entity" :user="user" @login="$parent.$emit('login')" @signup="$parent.$emit('signup')" />
  <opportunity v-else :entity="entity" :user="user" @login="$parent.$emit('login')" @signup="$parent.$emit('signup')" />
</div>
</template>

<script>
import JustContent from '~/components/JustContent'
import Opportunity from '~/components/Opportunity'

export default {
    name: 'Dynamic',

    components: {
        JustContent,
        Opportunity
    },

    async asyncData ({ params, $axios }) {
        const entity = await $axios.$get('/api/ui/entity/' + params.slug);
        const layout = entity.entity_type.page ? entity.entity_type.page.layout : entity.entity_type;

        return { entity, layout };
    },

    head () {
        return {
            'title': this.entity.title + ' - Science Near Me',
        };
    },

    computed: {
        user() {
            return this.$store.state.user;
        },
    },
}
</script>
