<template>
<div>
  <h1>Saved Science Opportunities</h1>
  <section id="manage">
    <b-field label="Search your saved opportunities">
      <b-input v-model="search.text" type="text" icon-right="close-circle" icon-right-clickable @icon-right-click="reload({text: ''})" />
      <b-button @click="reload">
        Search
      </b-button>
    </b-field>
    <button @click="trash_old">
      <trash-icon />
      Remove Past Opportunities
    </button>
  </section>
  <section id="sort">
    <mini-select v-model="search.sort" label="Sort:" data-context="find-sort-order" @input="reload({sort: $event})">
      <option value="closest">
        Closest
      </option>
      <option value="soonest">
        Soonest
      </option>
    </mini-select>
  </section>
  <section id="results">
    <opportunity-card v-for="opp in matches" :key="opp.uid" :opportunity="opp" :hidden="opp.trashed" trash @trash="trash(opp)" />
  </section>
  <section id="pagination">
    <pagination :page-index="pagination.page_index" :last-page="pagination.last_page" @switch="reload({page: $event})" />
  </section>
</div>
</template>

<script>
import Vue from 'vue'
import OpportunityCard from '~/components/OpportunityCard'
import MiniSelect from '~/components/MiniSelect'

import TrashIcon from '~/assets/img/trash.svg?inline'

export default {
    name: "MySaved",

    components: {
        OpportunityCard,
        MiniSelect,

        TrashIcon,
    },

    httpHeaders() {
        return {
            'X-XSS-Protection': '1; mode=block',
            'X-Frame-Options': 'DENY',
            'X-Content-Type-Options': 'nosniff',
            'Referrer-Policy': 'same-origin',
        };
    },

    async asyncData(context) {
        const user = await context.store.dispatch('get_user');

        if(!user.authenticated) {
            context.redirect({name: 'login', query: {next: 'my-saved'}});
            return;
        }

        const search = {
            page: parseInt(context.query.page) || 0,
            text: context.query.search || '',
            sort: context.query.sort || 'closest',
        };

        const results = await context.$axios.$get('/api/ui/finder/search', {
            params: {
                'page': search.page,
                'per_page': 10,
                'saved': true,
                'person': user.uid,
                'text': search.text,
            },
            ...context.store.state.auth,
        });

        return {search, ...results};
    },

    computed: {
        user() {
            return this.$store.state.user;
        },
    },

    methods: {
        async reload(assign) {
            if(assign !== undefined) {
                for(let [key, value] of Object.entries(assign)) {
                    Vue.set(this.search, key, value);
                }
            }

            let query = {};

            if(this.search.page) {
                query['page'] = this.search.page;
            }

            if(this.search.text) {
                query['search'] = this.search.text;
            }

            if(this.search.sort !== 'closest') {
                query['sort'] = this.search.sort;
            }

            this.$router.replace({name: 'my-saved', query});

            const results = await this.$axios.$get('/api/ui/finder/search', {
                params: {
                    'page': this.page,
                    'per_page': 10,
                    'saved': true,
                    'person': this.user.uid,
                    'text': this.search.text,
                    'sort': this.search.sort,
                },
                ...this.$store.state.auth,
            });

            this.pagination = results.pagination;
            this.matches = results.matches;
        },

        async trash(opp) {
            Vue.set(opp, 'trashed', true);

            await this.$axios.$delete('/api/ui/profile/saved/' + opp.uid, this.$store.state.auth);

            await this.reload();
        },

        async trash_old() {
            await this.$axios.$delete('/api/ui/profile/saved/old', this.$store.state.auth);

            await this.reload();
        }
    }
}
</script>

<style lang="scss" scoped>
h1 {
    font-family: $snm-font-heading;
    font-weight: bold;
    font-size: $snm-font-small;
    line-height: 19px;
    letter-spacing: 0px;
    color: $snm-color-background-dark;
    padding: 1rem;
}

#manage {
    display: flex;
    flex-wrap: wrap;
    justify-content: right;
    border-bottom: 1px solid $snm-color-border;
    border-top: 1px solid $snm-color-border;
    padding: 1rem;
    background-color: var(--background-color, $snm-color-background-light);

    .field {
      width: 100%;

      .field.has-addons, .field.has-addons .control {
        width: 100%;
      }
    }

    :deep(label) {
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: $snm-font-smaller;
        line-height: 16px;
        letter-spacing: 0px;
        color: var(--primary-color, $snm-color-element-dark);
    }

    > button {
        display: flex;
        align-items: center;
        color: var(--secondary-color, $snm-color-element-med);
        border: 1px solid var(--secondary-color, $snm-color-element-med);
        background-color: var(--background-color, #fff);
        border-radius: 10px;
        box-shadow: 0px 3px 6px $snm-color-shadow;
        height: 40px;
        padding: 0px 1em;
        cursor: pointer;

        svg {
            margin-right: 1em;
            * {
                fill: currentColor;
            }
        }
    }
}

#sort {
    padding: 1.5rem 0.75rem;
}

@media (min-width: $fullsize-screen) {
    h1 {
        font-size: $snm-font-large;
        line-height: 28px;
        padding: 0px;
    }

    #manage {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-top: 1rem;
        border: 1px solid $snm-color-border;
        padding: 2rem;
        border-radius: 10px;

        .field {
          width: auto;

          .field.has-addons, .field.has-addons .control {
            width: auto;
          }
        }

        > button {
            margin-top: 0.75rem;
        }
    }
    .opportunity-card {
      border:1px solid $snm-color-border!important;
      margin-bottom: 1rem;
    }
}
</style>
