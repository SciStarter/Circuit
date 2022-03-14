<template>
<ul class="opportunity-time">
  <li v-for="pair in upcoming" :key="pair[0].toISOString()">
    {{ display(pair) }}
  </li>
</ul>
</template>

<script>
import zip from 'lodash/zip'
import sortBy from 'lodash/sortBy'

const EARLIEST = new Date('0001-01-01');
const LATEST = new Date('9999-01-01');

export default {
    props: {
        opportunity: {
            type: Object,
            required: true
        }
    },

    computed: {
        pairs() {
            if(this.opportunity.end_datetimes.length === this.opportunity.start_datetimes.length) {
                return zip(
                    this.opportunity.start_datetimes.map(iso => new Date(iso)),
                    this.opportunity.end_datetimes.map(iso => new Date(iso))
                );
            }
            else if(this.opportunity.start_datetimes.length == 1) {
                let dt = new Date(this.opportunity.start_datetimes[0]);
                if(this.opportunity.has_end) {
                    // End flag is set but no end provided, so use start + 1 hour
                    return [[dt, new Date(dt.getTime() + 60 * 60 * 1000)]];
                }
                else {
                    // Assume it's ongoing after its single start date
                    return [[dt, LATEST]];
                }
            }
            else if(this.opportunity.start_datetimes.length == 0 && this.opportunity.end_datetimes.length == 1) {
                    return [[EARLIEST, this.opportunity.end_datetimes[0]]];
            }
            else {
                return this.opportunity.start_datetimes.map(iso => {
                    let dt = new Date(iso);
                    // Assume it ends one hour after each start
                    return [dt, new Date(dt.getTime() + 60 * 60 * 1000)];
                });
            }
        },

        chronological() {
            return sortBy(this.pairs, [0, 1]);
        },

        upcoming() {
            const now = new Date();
            const future_start = this.chronological.filter(pair => pair[0] > now);
            const future_end = this.chronological.filter(pair => pair[0] < now && pair[1] > now);

            if(future_start.length > 0) {
                return future_start;
            }
            if(future_end.length > 0) {
                return future_end;
            }
            else if(this.chronological.length) {
                return this.chronological;
            }
            else {
                return [[EARLIEST, LATEST]];
            }
        },
    },

    mounted() {
        this.$emit('upcoming', this.upcoming);
    },

    methods: {
        display(pair) {
            const now = new Date();

            if(pair[1] < now) {
                return "Finished " + pair[1].toLocaleString();
            }
            else if(pair[0] < now && pair[1] >= LATEST) {
                return "Ongoing";
            }
            else if(pair[0] < now && pair[1] > now) {
                return "Ongoing through " + pair[1].toLocaleString([], { month: 'long', day:'numeric', year:'numeric', hour: 'numeric', minute: '2-digit'});
            }
            else if(pair[0].getFullYear && pair[1].getFullYear){
                if(pair[0].getFullYear() == pair[1].getFullYear() && pair[0].getMonth() == pair[1].getMonth() && pair[0].getDate() == pair[1].getDate()) {
                    return pair[0].toLocaleString([], { month: 'long', day:'numeric', year:'numeric', hour: 'numeric', minute: '2-digit'}) + ' through ' + pair[1].toLocaleString([], {hour: 'numeric', minute: '2-digit'});
                }
                else {
                    return pair[0].toLocaleString([], { month: 'long', day:'numeric', year:'numeric', hour: 'numeric', minute: '2-digit'});
                }
            }
            else if(pair[0].getFullYear){
                return pair[0].toLocaleString([], { month: 'long', day:'numeric', year:'numeric', hour: 'numeric', minute: '2-digit'});
            }
            else if(pair[1].getFullYear){
                return pair[1].toLocaleString([], { month: 'long', day:'numeric', year:'numeric', hour: 'numeric', minute: '2-digit'});
            }
            else {
                return "";
            }
        }
    }
}
</script>

<style lang="scss" scoped>
.opportunity-time li:not(:first-child) {
    display: none;
}

@media (min-width: $fullsize-screen) {
    .opportunity-time li:not(:first-child) {
        display: list-item;
    }
}
</style>
