import Vue from 'vue';

function block_key(language, group, item) {
    return "dyn:" + language + ":" + group + ":" + item;
}

export const state = () => ({
    dynamic_blocks: {},

    user: {
        authenticated: false,
    },
});

export const mutations = {
    save_dynamic_block(state, {language, group, item, content}) {
        Vue.set(state.dynamic_blocks, block_key(language, group, item), content);
    },

    save_user(state, user) {
        state.user = user;
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
                        item,
                    }
                });

                commit('save_dynamic_block', {language, group, item, content});
            }
            catch(x) {
                commit('save_dynamic_block', {language, group, item, content: ""});
            }
        }

        return state.dynamic_blocks[key] || null;
    },

    async login({commit, state}, {email, password}) {
        let user = { authenticated: false };

        try {
            user = await this.$axios.$post("/api/ui/login", {
                email,
                password,
            });
        }
        catch(error) {
            console.log(error);
            return { authenticated: false };
        }

        if(process.client) {
            if(user.authenticated) {
                window.localStorage.setItem('token', user.token);
            }
            else {
                window.localStorage.removeItem('token');
            }
        }

        commit('save_user', user);

        return user;
    },

    async signup({commit, state}, {email, username, password, zip_code, phone}) {
        let params = {
            email,
            password,
        };

        if(username) {
            params['username'] = username;
        }

        if(zip_code) {
            params['zip_code'] = zip_code;
        }

        if(phone) {
            params['phone'] = phone;
        }

        let user = { authenticated: false };

        try {
            user = await this.$axios.$post("/api/ui/signup", params);
        }
        catch(error) {
            console.log(error);
            return { authenticated: false };
        }

        if(process.client) {
            if(user.authenticated) {
                window.localStorage.setItem('token', user.token);
            }
            else {
                window.localStorage.removeItem('token');
            }
        }

        commit('save_user', user);

        return user;
    },

    async get_user({commit, state}) {
        let token = null;

        let user = {
            authenticated: false,
        };

        // The cookie and localStorage value are saved on the client
        // by the refresh_user plugin.
        if(process.server) {
            token = this.$cookies.get('token');
        }
        else if(process.client) {
            token = window.localStorage.getItem('token');
        }

        if(state.user.authenticated) {
            user = state.user;
        }
        else if(!!token) {
            user = await this.$axios.$get("/api/ui/me", {
                headers: {
                    "Authorization": "Bearer " + token,
                },
            });
        }

        commit('save_user', user);

        return user;
    }
};
