<template>
<div ref="display"></div>
</template>

<script>
import * as d3 from 'd3'

export default {
    name: "ProgressGauge",

    props: {
        value: {
            type: Number,
            required: true,
            default: 0.0,
        },

        max: {
            type: Number,
            required: false,
            default: 100.0,
        },

        thickness: {
            type: Number,
            required: false,
            default: 40.0,
        },

        foreground: {
            type: String,
            required: false,
            default: '#165E6F',
        },

        background: {
            type: String,
            required: false,
            default: '#5694A2',
        },

        showLabel: {
            type: Boolean,
            required: false,
            default: false,
        },

        reverse: {
            type: Boolean,
            required: false,
            default: false,
        },
    },

    data() {
        return {
            gauge: null,
        }
    },

    watch: {
        value(new_value, old_value) {
            if(this.gauge != null && new_value != old_value) {
                this.gauge.update(new_value, this.max);
            }
        },

        max(new_max, old_max) {
            if(this.gauge != null && new_max != old_max) {
                this.gauge.update(this.value, new_max);
            }
        },
    },

    mounted() {
        this.gauge = this.build_gauge(this.$refs.display, this.foreground, this.background, this.showLabel, this.thickness, this.reverse);
        this.gauge.update(this.value, this.max);
    },

    methods: {
        // based on https://codepen.io/dabrorius/pen/dWmOgB
        build_gauge(sel, fg, bg, show_label, default_thickness, reverse) {
            const HALF_PI = Math.PI / 2;
            const DEGREES = 180.0 / Math.PI;

            const parent = d3.select(sel)
            const size = parent.node().getBoundingClientRect()
            const svg = parent.append('svg')
                  .attr('width', size.width)
                  .attr('height', size.height);
            const radius = Math.min(size.width * 0.45, size.height * 0.9);
            const thickness = Math.min(radius, default_thickness);
            let value = 0;

            const arc = d3.arc()
                  .startAngle(-HALF_PI)
                  .endAngle(HALF_PI)
                  .innerRadius(radius-thickness)
                  .outerRadius(radius);

            svg.append("path")
                .attr('style', 'fill:' + bg)
                .attr('transform', `translate(${size.width/2},${size.height})`)
                .attr('d', arc());

            const arc_path = svg.append("path")
                  .attr('style', 'fill:' + fg)
                  .attr('transform', `translate(${size.width/2},${size.height})`);

            // const end = svg.append("circle")
            //       .attr('style', 'fill:' + (reverse ? bg : fg))
            //       .attr('transform', `translate(${size.width/2},${size.height-radius+thickness/2})`)
            //       .attr('width', thickness)
            //       .attr('height', thickness)
            //       .attr('r', thickness/2);

            if(show_label) {
                let progress_label = svg.append("text")
                    .attr('style', 'fill:' + fg)
                    .attr('transform', `translate(${size.width/2},${size.height - 16})`)
                    .text('');
            }

            return {
                update: function(progress, max) {
                    const start = value
                    const start_radians = (Math.PI * start / max) - HALF_PI;
                    const start_degrees = start_radians * DEGREES;
                    const delta_radians = (Math.PI * progress / max) - start_radians - HALF_PI;
                    const delta_degrees = delta_radians * DEGREES;
                    const animate_ms = 1500;

                    arc_path.transition().duration(animate_ms).attrTween('d', function(){
                        return function(t) {
                            arc.endAngle(start_radians + delta_radians * t)
                            return arc();
                        }
                    })

                    // end.transition().duration(animate_ms).attrTween('transform', function(){
                    //     return function(t) {
                    //         return `translate(${size.width/2},${size.height})`+
                    //             `rotate(${(start_degrees + delta_degrees * t)})`+
                    //             `translate(0,-${radius-thickness/2})`
                    //     }
                    // })

                    if(show_label) {
                        progress_label.transition().duration(animate_ms).tween('bla', function() {
                            return function(t) {
                                progress_label.text('' + Math.round(start + (progress - start) * t));
                            }
                        })
                    }

                    value = progress
                }
            }
        }
    }
}
</script>

<style lang="scss" scoped>
div {
    width: 100%;
    height: 100%;
}
</style>
