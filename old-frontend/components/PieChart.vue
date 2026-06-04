<template>
<div>

    <div v-if="!hasData" id="no-data">
        <h1>No Data to Display</h1>
    </div>

    <div v-else :class="{'simplify':simplify}">
        <canvas ref="display" />
    </div>

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
    
    computed:{
        hasData(){
            if (this.data.datasets[0].data.length == 0){
                return false;
            } else if (this.data.datasets[0].data.reduce( (num,total) => total + num ) == 0){
                return false;
            } else {
                return true;
            }
        }
    },

    watch: {
        data() {
            if (this.hasData) {
                this.refresh();
            }
             
        },

        doughnut() {
            if (this.hasData) {
                this.refresh();
            }
        },
    },

    mounted() {
        if (this.hasData) {
                this.refresh();
            }
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
#no-data {
    padding:3rem;
    text-align: center;
    h1 {
        font-weight: bold;
    }
    p {
        font-size: .8rem;
    }
}
</style>
