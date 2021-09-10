import Vue from 'vue';

function block_key (language, group, item) {
    return 'dyn:' + language + ':' + group + ':' + item;
}

export const state = () => ({
    dynamic_blocks: {},

    user: {
        authenticated: false
    },

    language: 'en',

    partners: [],

    topics: [],

    descriptors: {}
});

export const mutations = {
    save_dynamic_block(state, { language, group, item, content }) {
        Vue.set(state.dynamic_blocks, block_key(language, group, item), content);
    },

    save_user(state, user) {
        state.user = user;
    },

    save_language(state, language) {
        state.language = language;
    },

    save_partners(state, partners) {
        state.partners = partners;
    },

    save_topics(state, topics) {
        state.topics = topics;
    },

    save_descriptors(state, descriptors) {
        state.descriptors = descriptors;
    },
};

export const actions = {
    get_language({ commit, state }) {
        // Load language preference here if we decide to support
        // multiple site translations.
        return state.language || 'en';
    },

    async get_dynamic_block({ commit, state }, { language, group, item }) {
        const key = block_key(language, group, item);

        if (state.dynamic_blocks[key] === undefined) {
            try {
                const content = await this.$axios.$get('/api/ui/content', {
                    params: {
                        language,
                        group,
                        item
                    }
                });

                commit('save_dynamic_block', { language, group, item, content });
            } catch (x) {
                commit('save_dynamic_block', { language, group, item, content: '' });
            }
        }

        return state.dynamic_blocks[key] || null;
    },

    async login({ commit, dispatch, state }, { email, password }) {
        let user = { authenticated: false };

        try {
            user = await this.$axios.$post('/api/ui/auth/login', {
                email,
                password
            });
        } catch (error) {
            console.error(error);
            return { authenticated: false };
        }

        if (process.client) {
            if (user.authenticated) {
                window.localStorage.setItem('token', user.token);
            } else {
                window.localStorage.removeItem('token');
            }
        }

        commit('save_user', user);

        dispatch('sync_local_to_server');

        return user;
    },

    async signup({ commit, dispatch, state }, { email, username, password, zip_code, phone }) {
        const params = {
            email,
            password
        };

        if (username) {
            params.username = username;
        }

        if (zip_code) {
            params.zip_code = zip_code;
        }

        if (phone) {
            params.phone = phone;
        }

        let user = { authenticated: false };

        try {
            user = await this.$axios.$post('/api/ui/auth/signup', params);
        } catch (error) {
            console.error(error);
            return { authenticated: false };
        }

        if (process.client) {
            if (user.authenticated) {
                window.localStorage.setItem('token', user.token);
            } else {
                window.localStorage.removeItem('token');
            }
        }

        commit('save_user', user);

        dispatch('sync_local_to_server');

        return user;
    },

    async logout({ commit, state }) {
        let user = state.user;

        try {
            user = await this.$axios.$post('/api/ui/auth/logout');
        } catch (error) {
            console.error(error);
            return { authenticated: false };
        }

        if (process.client) {
            if (user.authenticated) {
                window.localStorage.setItem('token', user.token);
            } else {
                window.localStorage.removeItem('token');
            }
        }

        commit('save_user', user);

        return user;
    },

    async get_user({ commit, dispatch, state }) {
        let token = null;

        let user = {
            authenticated: false
        };

        // The cookie and localStorage value are saved on the client
        // by the refresh_user plugin.
        if (process.server) {
            token = this.$cookies.get('token');
        } else if (process.client) {
            token = window.localStorage.getItem('token');
        }

        if (state.user.authenticated) {
            user = state.user;
        } else if (token) {
            try {
                user = await this.$axios.$get('/api/ui/auth/me', {
                    headers: {
                        Authorization: 'Bearer ' + token
                    }
                }, { withCredentials: true });
            }
            catch(error) {}
        }

        commit('save_user', user);

        dispatch('sync_local_to_server');

        return user;
    },

    async get_partners({ commit, state }) {
        if(state.partners.length > 0) {
            return state.partners;
        }

        const partners = await this.$axios.$get('/api/ui/finder/partners');

        commit('save_partners', partners);

        return partners;
    },

    async get_topics({ commit, state }) {
        if(state.topics.length > 0) {
            return state.topics;
        }

        const topics = await this.$axios.$get('/api/ui/finder/topics');

        commit('save_topics', topics);

        return topics;
    },

    async get_descriptors({ commit, state }) {
        if(state.descriptors.length > 0) {
            return state.descriptors;
        }

        const descriptors = await this.$axios.$get('/api/ui/finder/descriptors');

        commit('save_descriptors', descriptors);

        return descriptors;
    },

    async sync_local_to_server({ commit, dispatch, state }) {
        if(process.server || !state.user.authenticated) {
            return;
        }

        let local = await dispatch('get_local');

        // sync "I did this" records
        let didit = local.didit || [];
        try {
            for(let slug of didit) {
                console.log(slug);
                await this.$axios.$post('/api/ui/entity/' + slug + '/didit', {}, { withCredentials: true });
            }
            local.didit = [];
        }
        catch(error) {
            // abort and try again later
            return;
        }

        await dispatch('set_local', local);
    },

    // Doesn't actually operate on the store; it's here because it's
    // logically related.
    async get_local({commit, state}) {
        if (process.client) {
            return JSON.parse(window.localStorage.getItem('local_state') || '{}');
        }
        else {
            return {};
        }
    },

    // Doesn't actually operate on the store; it's here because it's
    // logically related.
    async set_local({commit, state}, local) {
        if (process.client) {
            window.localStorage.setItem('local_state', JSON.stringify(local));
        }
    },
}
