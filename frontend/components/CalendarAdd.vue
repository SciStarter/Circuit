<template>
<a :href="url" target="_blank" class="calendar-add" @click="$emit('before', calendar)">
  <img :src="icon.url" :alt="icon.alt" :title="icon.title"> <span>{{icon.title}}</span>
</a>
</template>

<script>
import GoogleCalendarIcon from '~/assets/img/google-calendar.svg?data'
import MsftCalendarIcon from '~/assets/img/msft-calendar.svg?data'
import YahooIcon from '~/assets/img/yahoo.svg?data'
import Office365Icon from '~/assets/img/office-365.svg?data'

export default {
    props: {
        calendar: {
            type: String,
            required: true,
            default: 'google',
        },

        title: {
            type: String,
            required: true,
            default: '',
        },

        begin: {
            type: Date,
            required: true,
            default: new Date(),
        },

        end: {
            type: Date,
            required: true,
            default: new Date(),
        },

        location: {
            type: String,
            required: false,
            default: '',
        },

        description: {
            type: String,
            required: false,
            default: ''
        },
    },

    computed: {
        url() {
            let title = encodeURIComponent(this.title);
            let description = encodeURIComponent(this.description);
            let location = encodeURIComponent(this.location);
            let begin;
            let end;

            switch(this.calendar) {
            case 'google':
                begin = this.begin.toISOString().replace(/[-:]/g, '').split('.')[0];
                end = this.end.toISOString().replace(/[-:]/g, '').split('.')[0];
                return 'https://calendar.google.com/calendar/render?action=TEMPLATE&dates=' + begin + 'Z%2F' + end + 'Z&details=' + description + '&location=' + location + '&text=' + title;
            case 'outlook':
                begin = encodeURIComponent(this.begin.toISOString().split('.')[0] + '+00:00');
                end = encodeURIComponent(this.end.toISOString().split('.')[0] + '+00:00');
                return 'https://outlook.live.com/calendar/0/deeplink/compose?body=' + description + '&enddt=' + end + '&location=' + location + '&path=%2Fcalendar%2Faction%2Fcompose&rru=addevent&startdt=' + begin + '&subject=' + title;
            case '365':
                begin = encodeURIComponent(this.begin.toISOString().split('.')[0] + '+00:00');
                end = encodeURIComponent(this.end.toISOString().split('.')[0] + '+00:00');
                return 'https://outlook.office.com/calendar/0/deeplink/compose?body=' + description + '&enddt=' + end + '&location=' + location + '&path=%2Fcalendar%2Faction%2Fcompose&rru=addevent&startdt=' + begin + '&subject=' + title;
            case 'yahoo':
                begin = this.begin.toISOString().replace(/[-:]/g, '').split('.')[0];
                end = this.end.toISOString().replace(/[-:]/g, '').split('.')[0];
                return 'https://calendar.yahoo.com/?desc=' + description + '&et=' + end + 'Z&in_loc=' + location + '&st=' + begin + 'Z&title=' + title + '&v=60';
            default:
                console.warn("Unrecognized calendar type", this.calendar);
                return 'unrecognized';
            }
        },

        icon() {
            switch(this.calendar) {
            case 'google':
                return {
                    url: GoogleCalendarIcon,
                    title: 'Add to Google Calendar',
                    alt: 'Google calendar logo',
                };
            case 'outlook':
                return {
                    url: MsftCalendarIcon,
                    title: 'Add to Outlook Calendar',
                    alt: 'Outlook calendar logo',
                };
            case '365':
                return {
                    url: Office365Icon,
                    title: 'Add to Office 365 Calendar',
                    alt: 'Office 365 calendar logo',
                };
            case 'yahoo':
                return {
                    url: YahooIcon,
                    title: 'Add to Yahoo Calendar',
                    alt: 'Yahoo calendar logo',
                };
            default:
                console.warn("Unrecognized calendar type", this.calendar);
                return {
                    url: 'icon',
                    title: 'title',
                    alt: 'alt',
                };
            }
        },
    }
}
</script>

<style lang="scss" scoped>
img {
    display: block;
    width: 32px;
    height: 32px;
    margin-right:1rem;
}
</style>
