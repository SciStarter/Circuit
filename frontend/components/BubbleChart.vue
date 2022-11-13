<template>
    <div id="bubble-wrap">
        <div ref="display"></div>
        <div id="bubble-tooltip">

        </div>
    </div>
</template>
    
    <script>
    import * as d3 from 'd3'
    
    export default {
        name: "BubbleChart",
    
        props: {
            chart_data: {
                type: Object,
                required: true,
                default: {},
            },
            org: {
                type: String,
                required: true,
                default: '',
            },
            current_opp: {
                type: Object,
                required: true,
                default: {},
            }
            
        },
    
        data() {
            return {
                chart: null,
                windowWidth: window.innerWidth,
            }
        },
    
        watch: {
            chart_data() {
                if(this.chart === null) {
                    return;
                }
    
                this.set_chart()
            }
        },

        computed:{
            bubble_data(){
                let opp = [{
                    name: this.current_opp.title,
                    overlap: 1,
                    host: this.org,
                    activity_types:[],
                    format:"",
                    max_age:null,
                    min_age:null,
                    venue_types:[],
                    color: '#dedede'
                }];
                const host = this.org;
                let data = this.chart_data.table.map(d=> {
                    d.color = (d.host == host) ? '#1B4A54' : '#7CB4BF';
                    return d;
                 });
                return opp.concat(data);
            }
        },
    
        mounted() {
            this.getChartWidth();
            this.set_chart();
        },
    
        methods: {
            set_chart() {

                let engage = this.chart_data.engagement_type.toLowerCase();

                let opp = this.current_opp.title;
                

                // Copyright 2021 Observable, Inc.
                // Released under the ISC license.
                // https://observablehq.com/@d3/bubble-chart
                function BubbleChart(data, {
                    name = ([x]) => x, // alias for label
                    label = name, // given d in data, returns text to display on the bubble
                    value = ([, y]) => y, // given d in data, returns a quantitative size
                    group, // given d in data, returns a categorical value for color
                    title, // given d in data, returns text to show on hover
                    link, // given a node d, its link (if any)
                    linkTarget = "_blank", // the target attribute for links, if any
                    width = 1000, // outer width, in pixels
                    height = 1000, // outer height, in pixels
                    padding = 3, // padding between circles
                    margin = 1, // default margins
                    marginTop = margin, // top margin, in pixels
                    marginRight = margin, // right margin, in pixels
                    marginBottom = margin, // bottom margin, in pixels
                    marginLeft = margin, // left margin, in pixels
                    groups, // array of group names (the domain of the color scale)
                    colors = d3.schemeTableau10, // an array of colors (for groups)
                    fill = "#ccc", // a static fill color, if no group channel is specified
                    fillOpacity = 1, // the fill opacity of the bubbles
                    stroke, // a static stroke around the bubbles
                    strokeWidth, // the stroke width around the bubbles, if any
                    strokeOpacity, // the stroke opacity around the bubbles, if any
                    } = {}) {
                    // Compute the values.
                    const D = d3.map(data, d => d);
                    const V = d3.map(data, value);
                    const G = group == null ? null : d3.map(data, group);
                    const I = d3.range(V.length).filter(i => V[i] > 0);

                    // Unique the groups.
                    if (G && groups === undefined) groups = I.map(i => G[i]);
                    groups = G && new d3.InternSet(groups);

                    // Construct scales.
                    const color = G && d3.scaleOrdinal(groups, colors);

                    // Compute labels and titles.
                    const L = label == null ? null : d3.map(data, label);
                    const T = title === undefined ? L : title == null ? null : d3.map(data, title);

                    // KEVIN EDIt
                    // set the max size of the circles
                    const sizeScale = d3.scaleSqrt().range([0,100]);

                    // Compute layout: create a 1-deep hierarchy, and pack it.
                    const root = d3.pack()
                        .size([width - marginLeft - marginRight, height - marginTop - marginBottom])
                        .padding(padding)
                        .radius(d=>sizeScale(d.value))
                        (d3.hierarchy({children: I})
                        .sum(i => V[i]));


                    const svg = d3.create("svg")
                        // .attr("width", width)
                        // .attr("height", height)
                        .attr("viewBox", `0 0 ${width} ${height}`)
                        .attr("preserveAspectRatio","xMinYMid")
                        // .attr("viewBox", [-marginLeft, -marginTop, width, height])
                        .attr("style", "max-width: 100%; height: auto; height: intrinsic;")
                        .attr("fill", "currentColor")
                        .attr("font-size", 10)
                        .attr("font-family", "sans-serif")
                        .attr("text-anchor", "middle")
                        .attr("id","bubble");

                    const g = svg.append('g')
                        .attr("id","bubble-g");

                    const tooltip = d3.select("#bubble-tooltip");

                    const leaf = g.selectAll("a")
                        .data(root.leaves())
                        .join("a")
                        // .attr("xlink:href", link == null ? null : (d, i) => link(D[d.data], i, data))
                        // .attr("target", link == null ? null : linkTarget)
                        .attr("transform", d => `translate(${d.x},${d.y})`)
                        .style('opacity',function(d){
                            return 1;
                        });


                       leaf.on("mouseover", function(e,d) {		
               
                            if (D[d.data].name != opp)
                                tooltip.transition().duration(200).style("opacity", 1);		

                            tooltip.html('<h2><strong>' + (D[d.data].overlap * 100).toFixed(0) + '%</strong> overlap of ' + engage  +'</h2><p class="opp">' +  D[d.data].name + '</p><p>' + D[d.data].host + '</p>');
                            // `${D[d.data].name}`
                             tooltip
                                .style("left", (e.pageX) + "px")		
                                .style("top", (e.pageY - 28) + "px");	
                            })	
                        .on("mousemove", function(d){
                            tooltip.style("left", (d.pageX) + "px")		
                            .style("top", (d.pageY - 120) + "px");	
                        })		
                         .on("mouseleave", function(d) {		
                            tooltip.transition().duration(500).style("opacity", 0);	
                        });

                    leaf.append("circle")
                        .attr("stroke", stroke)
                        .attr("stroke-width", strokeWidth)
                        .attr("stroke-opacity", strokeOpacity)
                        .attr("fill", d => D[d.data].color)
                        .attr("r", d => d.r);
                         

                    if (T) leaf.append("title")
                        .text(d => T[d.data]);

                    if (L) {
                        // A unique identifier for clip paths (to avoid conflicts).
                        const uid = `O-${Math.random().toString(16).slice(2)}`;

                        leaf.append("clipPath")
                            .attr("id", d => `${uid}-clip-${d.data}`)
                        .append("circle")
                            .attr("r", d => d.r);

                        leaf.append("text")
                            .attr("clip-path", d => `url(${new URL(`#${uid}-clip-${d.data}`, location)})`)
                        .selectAll("tspan")
                        .data(d => `${L[d.data]}`.split(/\n/g))
                        .join("tspan")
                            .attr("x", 0)
                            .attr("y", (d, i, D) => `${i - D.length / 2 + 0.85}em`)
                            .attr("fill-opacity", (d, i, D) => i === D.length - 1 ? 1 : null)
                            .attr('color','#fff')
                            .text(d => d);
                    }

                    return Object.assign(svg.node(), {scales: {color}});
                }

                 //add the base opp data
                 


                let chart = BubbleChart(this.bubble_data, {
                    label: d => d.name,
                    value: d => d?.overlap,
                    fill: d => {
                        if (d.name == this.current_opp.title) {
                            return "#dedede";
                        } else {
                            return "#990000";
                        }
                    },
                    // width: this.windowWidth,
                    // height: 600
                    })
    
                this.chart = chart;
                this.$refs.display.replaceChildren(chart);
                
                // let b = d3.select('#bubble');
                // b.style('height',Math.round(d3.select('#bubble-g').node().getBoundingClientRect().height + 16) + 'px');
                // b.attr('viewBox',`0 0 640 ${Math.round(d3.select('#bubble-g').node().getBoundingClientRect().height + 16)}`);
                

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
        max-width: 650px;
        margin: 0 auto;
    }

    a {
        cursor:default;
    }

    #bubble-tooltip {
        opacity: 0;
        position: absolute;	
        display:block;
        width: 200px;
        height: 100px;
        background-color: #fff;
        box-shadow: 0 0 4px rgba(0,0,0,.5);
        border-radius: 6px;

       :deep(h2) {
            border-bottom: 1px solid #dee2e6;
            box-shadow: 0 4px 4px rgba(0,0,0,.05);
            padding:.5rem 1rem .3rem;
            color: #888888;
            margin-bottom: .5rem;
            strong {
                color: #357382;
                font-size: 18px;
            }
        }
        :deep(.opp) {
            font-weight: bold;;
        }
        :deep(p) {
            padding: 0 1rem;
            font-size: 12px;
            line-height: 1.1;
        }

    }
    </style>
    