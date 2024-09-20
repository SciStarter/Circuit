export default {
    props: ["data"],

    render: function (createElement) {
        return createElement('script', {
            "attrs": {
                "type": "application/ld+json"
            },
            "domProps": {
                "innerHTML": JSON.stringify(this.data)
            }
        });
    }
}
