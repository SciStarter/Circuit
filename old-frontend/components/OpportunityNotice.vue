<template>
<aside v-if="notice_text" class="opportunity-notice">
  {{ notice_text }}
</aside>
</template>

<script>
import haversine from 'haversine'

export default {
    props: {
        opportunity: {
            type: Object,
            required: true
        },

        mode: {
            type: String,
            required: false,
            default: 'all'
        }
    },

    computed: {
        notice_text() {
            switch(this.mode) {
            case 'all':
                return this.notice_all;
            case 'place':
                return this.notice_place;
            case 'time':
                return this.notice_time;
            case 'test':
                return 'Test notification';
            default:
                return '';
            }
        },

        notice_place() {
            let subject = this.$geolocation.coords;
            let object = this.opportunity.location_point;

            if(!(subject && subject.longitude && subject.latitude &&
                 object && object.geometry && object.geometry.longitude && object.geometry.latitude)
              ) {
                return "";
            }

            let distance = Math.floor(
                haversine(
                    {
                        longitude: subject.longitude,
                        latitude: subject.latitude,
                    },
                    {
                        longitude: object.geometry.coordinates[0],
                        latitude: object.geometry.coordinates[1],
                    },
                    {unit: 'mile'}
                )
            );

            if(distance < 1) {
                return "less than a mile!";
            }
            else if(distance <= 20) {
                return "" + distance + " miles away";
            }
            else {
                return "";
            }
        },

        notice_time() {
            const now = new Date();

            let future = this.opportunity.start_datetimes
                               .map(iso => new Date(iso))
                               .filter(dt => dt > now);

            if(future.length > 0) {
                future.sort(function(a, b) {
                    if(a < b) {
                        return -1;
                    }
                    if(a > b) {
                        return 1;
                    }
                    return 0;
                });

                const until = (future[0] - now) / (60 * 60 * 1000);

                if(until > 24 && until < 168) {
                    const days = Math.floor(until / 24);
                    return "Happening in " + days + ((days > 1) ? " days" : " day");
                }
                else if(until > 1 && until < 24) {
                    const hours = Math.floor(until);
                    return "Happening in " + hours + ((hours > 1) ? " hours" : " hour");
                }
                else if(until < 1) {
                    return "Happening in a few minutes";
                }
            }

            return "";
        },

        notice_all() {
            return [this.notice_time, this.notice_place].filter(x => !!x).join(" and ");
        }
    }
}
</script>

<style lang="scss" scoped>
aside {
    display: block;
    font-family: $snm-font-content;
    font-size: $snm-font-smaller;
    color: $snm-color-info;
    letter-spacing: 0px;
    line-height: 16px;
}

@media (min-width: $fullsize-screen) {

}
</style>
