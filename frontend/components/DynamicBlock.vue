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
import tags from '~/assets/lib/tags'
import ExternalLink from '~/components/ExternalLink'

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

        // implies removeParagraphs
        inline: {
            type: Boolean,
            required: false,
            default: false,
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
        block: null,
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

        this.block = await this.$store.dispatch('get_dynamic_block', {
            language: this.language || this.default_language,
            group: this.group,
            item: this.item,
            inline: this.inline,
            removeParagraphs: this.removeParagraphs,
            fixLinks: this.fixLinks,
        })
    },

    computed: {
        content () {
            if(this.block === null || this.block.content === null) {
                return null;
            }

            let working = this.block.content;

            if(this.inline || this.removeParagraphs) {
                working = working.replace(/<\s*\/?\s*p\b.*?>/igs, '').trim()
            }

            if(this.fixLinks) {
                working = working.replace(
                    /<\s*a\s+(.*?)\bhref="(.*?)"(.*?)>(.*?)<\s*\/\s*a\s*>/igs,
                    (match, before, href, after, content) => {
                        const extra = before + ' ' + after;

                        const new_tab = extra.indexOf('target="_blank"') >= 0;
                        const title_match = extra.match(/\btitle="(.*?)"/is);
                        const title = title_match ? title_match[1] : '';

                        if(href.slice(0, 25) == 'https://sciencenearme.org') {
                            href = href.slice(25);
                        }

                        if(href[0] == '/') {
                            if(new_tab) {
                                return '<a href="' + href + '" target="_blank"' + (title ? ' title="' + title + '"' : ' ') + 'rel="noopener">' + content + '</a>';
                            }
                            else {
                                return '<nuxt-link to="' + href + '"' + (title ? ' title="' + title + '"' : '') + '>' + content + '</nuxt-link>';
                            }
                        }
                        else {
                            if(new_tab) {
                                return '<external-link href="' + href + '"' + (title ? ' title="' + title + '"' : '') + ' new-tab>' + content + '</external-link>';
                            }
                            else {
                                return '<external-link href="' + href + '"' + (title ? ' title="' + title + '"' : '') + '>' + content + '</external-link>';
                            }
                        }
                    }
                );
            }

            let wrap = tags.tagged(this.block.tags, 'wrap');
            if(wrap !== false && wrap.length > 0) {
                return '<' + wrap[0] + ' class="dynamic-block">' + working + '</' + wrap[0] + '>';
            }
            else if(this.inline) {
                return '<span class="dynamic-block">' + working + '</span>';
            }
            else {
                return '<div class="dynamic-block">' + working + '</div>';
            }
        },

        generated_component() {
            return Vue.extend({
                components: {
                    ExternalLink,
                },
                template: this.content,
            });
        }
    },
}
</script>
