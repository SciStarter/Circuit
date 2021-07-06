import Vue from 'vue';

function block_key(language, group, item) {
    return "dyn:" + language + ":" + group + ":" + item;
}

export const state = () => ({
    dynamic_blocks: {},
});

export const mutations = {
    save_dynamic_block(state, {language, group, item, content}) {
        Vue.set(state.dynamic_blocks, block_key(language, group, item), content);
    }
};

export const actions = {
    async get_dynamic_block({commit, state}, {language, group, item}) {
        const key = block_key(language, group, item);

        if(state.dynamic_blocks[key] === undefined) {
            try {
                const content = await this.$axios.$get("/api/ui/content", {
                    params: {
                        language,
                        group,
                        item
                    }
                });

                commit('save_dynamic_block', {language, group, item, content});
            }
            catch(x) {
                commit('save_dynamic_block', {language, group, item, content: ""});
            }
        }

        return state.dynamic_blocks[key] || null;
    }
};
