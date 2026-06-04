<template>
<div>
    <div ref="display" v-if="rows.length > 0"></div>
    <div v-else id="no-data">
        <h1>No Data to Display</h1>
        <p>Please select another time period</p>
    </div>
</div>
</template>

<script>
import * as d3 from 'd3'
import * as Plot from "@observablehq/plot";

export default {
    name: "LineChart",

    props: {
        rows: {
            type: Array,
            required: true,
            default: [],
        },

        xaxis: {
            type: [String, Function],
            required: true,
            default: "",
        },

        yaxes: {
            type: [String, Function, Array],
            required: true,
            default: [],
        },

        colors: {
            type: [Array],
            required: false,
            default: [],
        },

        maxy: {
            type: Number,
            required: false,
            default: -1,
        }
    },

    data() {
        return {
            chart: null,
            sorted_rows: [],
            windowWidth: window.innerWidth
        }
    },

    watch: {
        rows() {
            if(this.chart === null) {
                return;
            }

            if(this.rows.length > 0){
             this.set_chart()
            }
        },

        lines(new_lines) {
            if(this.chart === null) {
                return;
            }

            this.set_chart()
        }
    },

    mounted() {
        this.getChartWidth();
        if(this.rows.length > 0){
            this.set_chart();
        }

    },

    methods: {
        set_chart() {
            let xaxis = this.getter(this.xaxis);
            let yaxes = this.getters(this.yaxes);
            let colors = this.colors;

            this.sorted_rows = [...this.rows];

            this.sorted_rows.sort((a, b) => {
                let ax = xaxis(a);
                let bx = xaxis(b);

                if(ax > bx) {
                    return 1;
                }
                else if(ax < bx) {
                    return -1;
                }
                else {
                    return 0;
                }
            });

            if(colors.length < yaxes.length) {
                colors = yaxes.map(f => `rgb(${128 + Math.random() * 64}, ${128 + Math.random() * 64}, ${128 + Math.random() * 64})`);
            }

            let maxy = this.maxy;

            function dbg(v) {
                console.log(v);
                return v;
            }

            let chart = Plot.plot({
                width: this.windowWidth,
                height: 225,
                y: {
                    grid: true
                },
                marks: yaxes.map((ya, i) => Plot.line(this.sorted_rows, {
                    x: xaxis,
                    y: ya,
                    stroke: colors[i],
                    curve: "catmull-rom",
                })),
            });

            this.chart = chart;

            this.$refs.display.replaceChildren(chart);
        },

        getter(x) {
            var maxy = this.maxy;

            if(maxy < 0 || this.yaxes.indexOf(x) < 0) {
                if(typeof(x) === 'function') {
                    return x;
                }
                else {
                    return d => d[x];
                }
            }
            else {
                if(typeof(x) === 'function') {
                    return d => { return Math.min(x(d), maxy); };
                }
                else {
                    return d => Math.min(d[x], maxy);
                }
            }
        },

        getters(x) {
            if(Array.isArray(x)) {
                return x.map(sx => this.getter(sx));
            }
            else {
                return [this.getter(x)];
            }
        },

        getChartWidth() {
            if (window.innerWidth >= 1200 ) {
                this.windowWidth = window.innerWidth - 280;
            } else if (window.innerWidth <= 959) {
                this.windowWidth = window.innerWidth;
            } else {
                this.windowWidth = window.innerWidth - 200;
            }
        }
    },
}
</script>

<style lang="scss" scoped>
div {
    width: 100%;
    height: 100%;
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
