<template>
<div class="your-data-overview snm-container">
  <div class="flex-header">
    <h1>Your Data Overview</h1>
  </div>

  <progress-gauge class="gauge" :value="report['Demo Org'].current_opportunities" :max="report['Demo Org'].total_opportunities"/>

  <div class="nav-tab-wrapper">
    <ul class="nav-tabs">
      <li><a class="tab-link" :class="{'active':state==1}" @click="state=1">Current, Live Opportunities</a></li>
      <li><a class="tab-link" :class="{'active':state==2}" @click="state=2">Draft, Unpublished &amp; Expired</a></li>
      <li><a class="tab-link" :class="{'active':state==3}" @click="state=3">Expired or Trashed</a></li>
    </ul>
  </div>

</div>
</template>

<script>
export default {
    name: "MyDataOverview",

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });
        }

        return {
            report: {
                "Demo Org": {
                    total_opportunities: 23,
                    current_opportunities: 18,
                }
            }
        }
    },

    data() {
        return {
            state: 0,
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },
    },

    methods: {

    },
}
</script>

<style lang="scss" scoped>
.gauge {
    width: 400px;
    height: 300px;
}
</style>
