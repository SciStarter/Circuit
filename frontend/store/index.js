import Vue from 'vue';

function block_key (language, group, item) {
    return 'dyn:' + language + ':' + group + ':' + item;
}

export const state = () => ({
    dynamic_blocks: {},

    user: {
        authenticated: false
    },

    language: 'en'
});

export const mutations = {
    save_dynamic_block (state, { language, group, item, content }) {
        Vue.set(state.dynamic_blocks, block_key(language, group, item), content);
    },

    save_user (state, user) {
        state.user = user;
    },

    save_language (state, language) {
        state.language = language;
    }
};

export const actions = {
    get_language ({ commit, state }) {
        // Load language preference here if we decide to support
        // multiple site translations.
        return state.language || 'en';
    },

    async get_dynamic_block ({ commit, state }, { language, group, item }) {
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

    async login ({ commit, state }, { email, password }) {
        let user = { authenticated: false };

        try {
            const resp = await this.$axios.$post('/api/ui/auth/login', {
                email,
                password
            });

            user = resp.payload;
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

    async signup ({ commit, state }, { email, username, password, zip_code, phone }) {
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
            const resp = await this.$axios.$post('/api/ui/auth/signup', params);
            user = resp.payload;
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

    async logout ({ commit, state }) {
        let user = state.user;

        try {
            const resp = await this.$axios.$post('/api/ui/auth/logout');
            user = resp.payload;
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

    async get_user ({ commit, state }) {
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
            const resp = await this.$axios.$get('/api/ui/auth/me', {
                headers: {
                    Authorization: 'Bearer ' + token
                }
            });

            user = resp.payload;
        }

        commit('save_user', user);

        return user;
    }
}
