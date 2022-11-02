<template>
<div>
  <canvas ref="display"/>
</div>
</template>

<script>
import Chart from 'chart.js/auto';

export default {
    name: "PieChart",

    props: {
        doughnut: {
            type: Boolean,
            required: false,
            default: false,
        },

        data: {
            type: Object,
            required: true,
        },
    },

    data() {
        return {
          chart: null,  
        };
    },

    watch: {
        data() {
            this.refresh();
        },

        doughnut() {
            this.refresh();
        },
    },

    mounted() {
        this.refresh();
    },

    methods: {
        refresh() {
            if(this.chart !== null) {
                this.chart.destroy();
            }

            this.chart = new Chart(this.$refs.display, {
                type: this.doughnut ? 'doughnut' : 'pie',
                data: this.data,
            });
        },
    },
}
</script>

<style lang="scss" scoped>
div {
    width: 300px;
    height: 300px;
    margin: 2rem auto;
}
</style>
