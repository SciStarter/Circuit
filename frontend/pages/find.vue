<template>
<section class="section">
<pre>{{matches}}</pre>
</section>
</template>

<style lang="scss" scoped>
</style>

<script>
function from_qs(qs, names) {
    const dest = {};

    for(const name of names) {
        let val = qs[name];

        if(val !== undefined) {
            dest[name.endsWith("[]") ? name.slice(0, -2) : name] = val;
        }
    }

    return dest;
}

export default {
    name: 'Find',

    async asyncData(context) {
        const query = from_qs(context.query, [
            "longitude",
            "latitude",
            "proximity",
            "online",
            "text",
            "beginning",
            "ending",
            "physical",
            "min_age",
            "max_age",
            "topics[]",
            "descriptors[]",
            "cost",
            "venue_type",
            "host",
            "partner",
            "sort",
            "page",
            "per_page",
            "saved",
            "participated",
            "reviewing",
            "withdrawn",
            "over",
        ]);

        let { payload } = await context.$axios.$get("/api/ui/finder/search", {params: query});

        return Object.assign(
            {
                pagination: {
                    page: 0,
                    per_page: query.per_page ? parseInt(query.per_page) : 10,
                    num_pages: 0
                },
                opportunities: []
            },
            payload
        );
    }
}
</script>
