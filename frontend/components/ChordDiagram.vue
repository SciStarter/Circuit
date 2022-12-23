<template>
    <div id="chord-wrap">
        <div ref="display"></div>
        <div id="tooltip">

        </div>
    </div>
</template>
    
    <script>
    import * as d3 from 'd3'
    
    export default {
        name: "ChordDiagram",
    
        props: {
            chart_data: {
                type: Object,
                required: true,
                default: {},
            },
            metric: {
                type: String,
                required: true,
                default: 'Views'
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
            },
            metric(){
                this.set_chart()
            }
        },

        computed:{
            matrix(){
                let data = this.chart_data;
                let metric = this.metric;
                let arr = [];

                let fields = ['citizen_science','formal_education','live_science','maker','museum_or_science_center','out_of_school_time_program','policy','science_communications'];

                fields.forEach(d=>{
                    let a = [];
                        fields.forEach(e=>{
                            if (d == e) {
                                a.push(0);
                            } else {
                                // a.push(data[d][e][metric]);
                                if(data[d][e]) {
                                    a.push(data[d][e][metric] * data[d].proportion);
                                }
                                else {
                                    a.push(10);
                                }
                            }
                        });
                    arr.push(a);
                });

            

                return arr;
            }
        },
    
        mounted() {
            this.getChartWidth();
            this.set_chart();
        },
    
        methods: {
            set_chart() {

                let matrix = this.matrix;

                let metric = this.metric;

                let chart_data = this.chart_data;


            // based off https://bl.ocks.org/JulienAssouline/2847e100ac7d4d3981b0f49111e185fe
                function makeChordDiagram(data){

                    const width = 600;
                    const height = 600;

                    const svg = d3.create("svg")
                        .attr("viewBox", `0 0 ${width} ${height}`)
                        .attr("preserveAspectRatio","xMinYMid")
                        .attr("style", "max-width: 100%; height: auto; height: intrinsic;")
                        .attr("fill", "currentColor")
                        .attr("font-size", 10)
                        .attr("font-family", "sans-serif")
                        .attr("text-anchor", "middle")
                        .attr("id","chord");

                    const wrapper = svg.append("g")
                         .attr("transform", "translate(" + width / 2 + "," + height / 2 + ")");

                    const outerRadius = Math.min(width, height) * 0.5 -55
                    const innerRadius = outerRadius - 10

                    const Names = ['citizen_science','formal_education','live_science','maker','museum_or_science_center','out_of_school_time_program','policy','science_communications'];
                    const displayNames = ['Citizen Science','Formal Education','Live Science','Maker','Museum or Science Center','Out of School Time Program','Policy','Science Communications'];
                    const colors = ["#51addf", "#c582aa", "#005b9d", "#35a993", "#cc373c", "#f7d783","#ff7f00", "#f781bf"];

                    const nameByIndex = {
                        0: 'Citizen Science',
                        1: 'Formal Education',
                        2: 'Live Science',
                        3: 'Maker',
                        4: 'Museum or Science Center',
                        5: 'Out of School Time Program',
                        6: 'Policy',
                        7: 'Science Communications'
                    };

                    const e_lang = {
                        "Views" : 'viewed',
                        "Unique" : 'viewed (as unique viewers)',
                        "Clicks to Website" : 'clicked to'
                    }

                    const matrix = data;

                    const chordGenerator = d3.chord()
                        .padAngle(0.1)
                        .sortSubgroups(d3.descending)
                        .sortChords(d3.descending);

                    const chord = chordGenerator(matrix);

                    const arcs = d3.arc()
                        .innerRadius(innerRadius)
                        .outerRadius(outerRadius + 10);

                    const ribbon = d3.ribbon()
                        .radius(250)

                    const color = d3.scaleOrdinal()
                        .domain(d3.range(6))
                        .range(colors)


                    // creating the fill gradient
                     function getGradID(d){ return "linkGrad-" + d.source.index + "-" + d.target.index; }

                     const grads = svg.append("defs")
                        .selectAll("linearGradient")
                        .data(chord)
                        .enter()
                        .append("linearGradient")
                        .attr("id", getGradID)
                        .attr("gradientUnits", "userSpaceOnUse")
                        .attr("x1", function(d, i){ return innerRadius * Math.cos((d.source.endAngle-d.source.startAngle) / 2 + d.source.startAngle - Math.PI/2); })
                        .attr("y1", function(d, i){ return innerRadius * Math.sin((d.source.endAngle-d.source.startAngle) / 2 + d.source.startAngle - Math.PI/2); })
                        .attr("x2", function(d,i){ return innerRadius * Math.cos((d.target.endAngle-d.target.startAngle) / 2 + d.target.startAngle - Math.PI/2); })
                        .attr("y2", function(d,i){ return innerRadius * Math.sin((d.target.endAngle-d.target.startAngle) / 2 + d.target.startAngle - Math.PI/2); });

                    grads.append("stop")
                        .attr("offset", "0%")
                        .attr("stop-color", function(d){ return color(d.source.index)});

                     //set the ending color (at 100%)
                    grads.append("stop")
                        .attr("offset", "100%")
                        .attr("stop-color", function(d){ return color(d.target.index)});

                    //ribbons
                    wrapper
                        .selectAll("path")
                        .data(chord)
                        .enter()
                        .append("path")
                        .attr("class", function(d,i) {
                            return "chord chord-" + d.source.index + " chord-" + d.target.index // The first chord allows us to select all of them. The second chord allows us to select each individual one. 
                        })
                        .style("fill", function(d){ return "url(#" + getGradID(d) + ")"; })
                        .attr("d", ribbon)
                        .on('mouseenter',function(e,d){
                            
                            wrapper.selectAll(".chord")    
                                .filter( q => q !== d)
                                .style("opacity", 0.1);

                            d3.select('#tooltip')
                                .style("opacity",1)
                                .style("left", (e.pageX) + "px")		
                                .style("top", (e.pageY - 150) + "px")
                            
                                .html(
                                    '<p><strong>' + ((d.source.value/chart_data[Names[d.source.index]].proportion) * 100).toFixed(1) + '%</strong> who ' + e_lang[metric] + " a <strong>" + nameByIndex[d.source.index] + "</strong> opportunity " + e_lang[metric] + " a <strong>"  + nameByIndex[d.target.index] + '</strong> opportunity.</p>' +
                                    '<p><strong>' + ((d.target.value/chart_data[Names[d.target.index]].proportion) * 100).toFixed(1) + '%</strong> who ' + e_lang[metric] + " a <strong>" + nameByIndex[d.target.index] + "</strong> opportunity " + e_lang[metric] + " a <strong>"  + nameByIndex[d.source.index] + '</strong> opportunity.</p>'

                                );

                        })
                        .on('mouseleave',function(){
                            wrapper.selectAll(".chord").style("opacity", 1);
                            d3.select('#tooltip')
                                .style("opacity",0);
                        })
                        .on('mousemove',function(e){
                            d3.select('#tooltip')
                                .style("opacity",1)
                                .style("left", (e.pageX) + "px")		
                                .style("top", (e.pageY - 150) + "px")
                        });
                 
                    //arcs  
                    const g = wrapper.selectAll("g")
                        .data(chord.groups)
                        .enter()
                        .append("g")
                        .attr("class", "group");  


                    g.append("path")
                        .style("fill", function(d){ return color(d.index)})
                        .attr("d", arcs)
                        .style("opacity", 1)
                        .on('mouseenter',function(e,d){
                            d3.select('#tooltip')
                                .style("opacity",1)
                                .style("left", (e.pageX) + "px")		
                                .style("top", (e.pageY - 150) + "px")
                            
                                .html(
                                    '<p><strong>' + (chart_data[Names[d.index]].proportion * 100).toFixed(1) + '%</strong> of SNM projects are ' + "<strong>" + nameByIndex[d.index] + "</strong> opportunities" +'</p>'

                                );
                        });

                    //labels
                    g.append("text")
                        .each(function(d){ d.angle = (d.startAngle + d.endAngle) / 2; })
                        .attr("dy", ".35em")
                        .attr("class", "titles")
                        .attr("text-anchor", function(d) { return d.angle > Math.PI ? "middle" : null; })
                        .attr("transform", function(d) {
                            return "rotate(" + (d.angle * 180 / Math.PI - 90) + ")"
                                + "translate(" + (outerRadius + 20) + ")"
                                // + (d.angle > Math.PI ? "rotate(90)" : "");
                                + "rotate(90)"
                                })
                        .text(function(d,i){ return displayNames[i]; })
                        .style("font-size", "15px");   

                    return  Object.assign(svg.node());
                }

                

                let chart = makeChordDiagram(matrix);
    
                this.chart = chart;
                this.$refs.display.replaceChildren(chart);
  
                

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

    #tooltip {
        opacity: 0;
        position: absolute;	
        display:flex;
        flex-direction: column;
        width: 240px;
        height: auto;
        background-color: #fff;
        box-shadow: 0 0 4px rgba(0,0,0,.5);
        border-radius: 6px;
        padding:1rem;

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
            font-size: 12px;
            line-height: 1.1;
            margin-bottom: 10px;
        }

    }
    </style>
    
