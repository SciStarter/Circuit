<template>
<div :class="{'simplify':simplify}">
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

        simplify: {
            type:Boolean,
            required: false,
            default:false
        }
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


            let opt = {
                type: this.doughnut ? 'doughnut' : 'pie',
                data: this.data
            };

            if (this.simplify) {
                opt.options = {
                    cutout: '60%',
                    hover: {mode: null},
                    animation: false,
                    plugins: {
                        tooltip: {
                            enabled: false
                        },
                        legend: {
                            display: false
                        }
                    }
                };
            }


            this.chart = new Chart(this.$refs.display, opt);
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
div.simplify {
    width: 100px;
    height: 100px;
    margin: 0;
}
</style>
