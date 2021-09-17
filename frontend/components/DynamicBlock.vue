<template>
<component :is="generated_component" v-if="content" />
<div v-else class="dynamic-block">
  <slot>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam fringilla libero non elit semper, eu imperdiet tellus venenatis. Aenean et diam a nulla egestas pellentesque eu ut mauris. Praesent lacinia, odio in mollis condimentum, diam massa placerat lorem, dictum maximus dolor magna nec ipsum. Vivamus vitae diam ac leo rhoncus varius. Donec dapibus augue eu pretium bibendum. Nunc ac dignissim libero, sed hendrerit nulla. Etiam nec rutrum magna, a euismod tortor. Donec eu urna feugiat, commodo nunc et, rhoncus tortor. Phasellus nec convallis risus. Nunc nec turpis quis neque egestas lacinia non in urna. Nulla dictum arcu nec turpis venenatis, at consectetur ligula accumsan. Etiam est neque, interdum id magna sit amet, bibendum vestibulum diam. Vivamus vel porttitor justo, quis consectetur ipsum. Phasellus ac finibus magna, non convallis tortor. Nullam turpis dolor, tempor sit amet imperdiet in, eleifend vitae augue.</slot>
</div>
</template>

<script>
/*

  This component is used to present a block of HTML which can be
  edited in the management interface. This allows non-programmers to
  adjust the content of the site.

  These blocks are for content which is common to every instance of a
  particular page type. They're not intended for content which is
  specific to a particular person or which contains information
  requiring authentication to access.

  Default placeholder content can be placed between
  the <dynamic-block> and </dynamic-block> tags, and will be displayed
  in case the requested content can not retrieved. If that is not
  supplied wither, a paragraph of lorem ipsum text will be used.

*/

import Vue from 'vue'
import External from '~/components/External'

export default {
    name: "DynamicBlock",

    props: {
        language: {
            type: String,
            default: '',
            required: false,
        },

        group: {
            type: String,
            required: true,
        },

        item: {
            type: String,
            required: true,
        },

        removeParagraphs: {
            type: Boolean,
            required: false,
            default: false,
        },

        fixLinks: {
            type: Boolean,
            required: false,
            default: true,
        },
    },

    data: () => ({
        raw_content: null,
        default_language: ''
    }),

    async fetch () {
        // We could have used this.$axios.$get directly here, and for
        // things that are more likely to change that's exactly what
        // we should do.

        // It's done through the Vuex state store here so that we can
        // cache the content locally (in the state store) during a
        // session rather than fetching it repeatedly.

        this.default_language = await this.$store.dispatch('get_language')

        this.raw_content = await this.$store.dispatch('get_dynamic_block', {
            language: this.language || this.default_language,
            group: this.group,
            item: this.item
        })
    },

    computed: {
        content () {
            if(this.raw_content === null) {
                return null;
            }

            let working = this.raw_content;

            if(this.removeParagraphs) {
                working = working.replaceAll(/<\s*\/?\s*p\b.*?>/igs, '').trim()
            }

            if(this.fixLinks) {
                working = working.replaceAll(
                    /<\s*a\s+(.*?)\bhref="(.*?)"(.*?)>(.*?)<\s*\/\s*a\s*>/igs,
                    (match, before, href, after, content) => {
                        const extra = before + ' ' + after;

                        const new_tab = extra.indexOf('target="_blank"') >= 0;
                        const title = extra.match(/title="(.*?)"/is)[1];

                        if(href.slice(0, 25) == 'https://sciencenearme.org') {
                            href = href.slice(25);
                        }

                        if(href[0] == '/') {
                            if(new_tab) {
                                return '<a href="' + href + '" target="_blank"' + (title ? ' title="' + title + '"' : ' ') + 'rel="noopener">' + content + '</a>';
                            }
                            else {
                                return '<nuxt-link to="' + href + '"' + (title ? ' title="' + title + '"' : '') + '>' + content + '</next-link>';
                            }
                        }
                        else {
                            if(new_tab) {
                                return '<external href="' + href + '"' + (title ? ' title="' + title + '"' : ' ') + 'new-tab>' + content + '</external>';
                            }
                            else {
                                return '<external href="' + href + '"' + (title ? ' title="' + title + '"' : '') + '>' + content + '</external>';
                            }
                        }
                    }
                );
            }

            return '<div class="dynamic-block">' + working + '</div>';
        },

        generated_component() {
            return Vue.extend({
                components: {
                    External,
                },
                template: this.content,
            });
        }
    }
}
</script>
