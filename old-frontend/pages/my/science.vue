<template>
<div>
  <b-tabs v-model="current_tab" type="is-boxed">
    <b-tab-item class="pending">
      <template #header>
        Report Your Science<span v-if="user.reports_pending > 0" class="bubble">{{ user.reports_pending }}</span>
      </template>
      <!-- <section v-if="show_explanation" class="explanation">
        <a class="close" @click="show_explanation = false">&times;</a>
        <h2>What is this section?</h2> -->
        <p class="my-intro">
          This section lists opportunities you’ve expressed an
          interest in by clicking to a website, saving, or adding to
          your calendar. Recording and reporting this activity helps
          scientists learn how people engage in science.
        </p>
      <!-- </section> -->
      <b-message 
            title="Thanks for Letting Us Know!" 
            type="is-warning" 
            aria-close-label="Close message"
            v-model="show_activity">
            You'll find this in your activity log (in the tab above)!
        </b-message>
      <section class="results">
        <div v-for="inv in report.matches" :key="inv.id" class="reportable">
          <opportunity-card :opportunity="inv.opportunity" :rule="false" />
          <div class="actions">
            <action-button tertiary @click="did(inv)">
              <span class="bigger">&check;</span> I did this
            </action-button>
            <action-button tertiary @click="did_not(inv)">
              <span class="bigger">&times;</span> I didn't do this
            </action-button>
          </div>
        </div>
      </section>
      <section class="pagination">
        <pagination :page-index="report.pagination.page_index" :last-page="report.pagination.last_page" @switch="reload_report({page: $event})" />
      </section>
    </b-tab-item>
    <b-tab-item label="Your Activity Log" class="log">
      <section class="manage">
        <div class="status">
          <span>{{ log.pagination.total }}</span>
          <span>Total&nbsp;Science Opportunities&nbsp;Logged</span>
        </div>
        <div class="search">
          <b-field label="Search your opportunities">
            <b-input v-model="log_search.text" type="text" icon-right="close-circle" icon-right-clickable @icon-right-click="reload_log({text: ''})" />
            <b-button @click="reload_log">
              Search
            </b-button>
          </b-field>
        </div>
      </section>
      <section class="results">
        <opportunity-card v-for="inv in log.matches" :key="inv.id" :opportunity="inv.opportunity" />
      </section>
      <section class="pagination">
        <pagination :page-index="log.pagination.page_index" :last-page="log.pagination.last_page" @switch="reload_log({page: $event})" />
      </section>
    </b-tab-item>
  </b-tabs>
</div>
</template>

<script>
import Vue from 'vue'
import OpportunityCard from '~/components/OpportunityCard'
import ActionButton from '~/components/ActionButton'

export default {
    name: "MyScience",

    components: {
        OpportunityCard,
        ActionButton,
    },

    data(){
      return {
        show_activity: false
      }
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
            context.redirect({name: 'login', query: {next: 'my-science'}});
        }

        const auth = context.store.state.auth;

        const report_search = {
            page: 0,
            min: 10,
            max: 20,
            opp: true,
        };

        const report = await context.$axios.$get('/api/ui/profile/involved', {
            params: report_search,
            ...auth,
        });

        const log_search = {
            page: 0,
            min: 30,
            opp: true,
            text: "",
        };

        const log = await context.$axios.$get('/api/ui/profile/involved', {
            params: log_search,
            ...auth,
        });

        return {
            report_search,
            report,
            log_search,
            log,
            current_tab: 0,
            show_explanation: true
        };
    },

    computed: {
        user() {
            return this.$store.state.user;
        },
    },

    methods: {
        async did(inv) {
            await this.$axios.$post('/api/ui/profile/involved', {
                id: inv.id,
                mode: 30, // Mode::Logged
            }, this.$store.state.auth);

            await this.reload_report();
            this.show_activity = true;
        },

        async did_not(inv) {
            await this.$axios.$post('/api/ui/profile/involved', {
                id: inv.id,
                mode: 5, // Mode::Ignored
            }, this.$store.state.auth);

            await this.reload_report();
        },

        async reload_report(assign) {
            if(assign !== undefined) {
                for(let [key, value] of Object.entries(assign)) {
                    Vue.set(this.report_search, key, value);
                }
            }

            const results = await this.$axios.$get('/api/ui/profile/involved', {
                params: this.report_search,
                ...this.$store.state.auth,
            });

            this.report = results;

            this.$store.commit('set_user_reports_pending', results.pagination.total);
        },

        async reload_log(assign) {
            if(assign !== undefined) {
                for(let [key, value] of Object.entries(assign)) {
                    Vue.set(this.log_search, key, value);
                }
            }

            const results = await this.$axios.$get('/api/ui/profile/involved', {
                params: this.log_search,
                ...this.$store.state.auth,
            });

            this.log = results;
        },
    }
}
</script>

<style lang="scss" scoped>
.my-intro {
  margin:1rem;
}
.explanation {
    padding: 24px 30px;
    background-color: $snm-color-note;
    border: 1px solid $snm-color-action;
    position: relative;
    border-radius: 10px;
    margin: 2rem 1rem;

    .close {
        position: absolute;
        top: 14px;
        right: 30px;
        font-family: $snm-font-content;
        font-size: $snm-font-larger;
        color: $snm-color-action-border;
        font-weight:bold;
    }

    h2 {
        font-family: $snm-font-heading;
        font-weight: bold;
        font-size: $snm-font-medium;
        line-height: 26px;
        color: $snm-color-background-dark;
        margin-bottom: 0.5em;
    }

    p {
        font-family: $snm-font-content;
        font-weight: normal;
        font-size: $snm-font-small;
        line-height: 22px;
        color: var(--primary-color, $snm-color-element-dark);
    }
}

.reportable {
    border: 1px solid $snm-color-border;
    border-radius: 8px;
    // padding: 1em 0px;
    margin-bottom: 1rem;

    .opportunity-card {
      border: 0!important;
      margin-bottom: 0;
    }

    .actions {
        .bigger {
            font-size: 150%;
            padding-right: 0.5rem;
        }
    }

}

.manage {
    .status {
        display: flex;
        border-radius: 10px;
        height: 63px;
        border: 1px solid $snm-color-border;
        overflow: hidden;

        :first-child {
            width: 73px;
            display: flex;
            flex-grow: 0;
            justify-content: center;
            align-items: center;
            background-color: var(--secondary-color, $snm-color-element-med);
            color: $snm-color-element-light;
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-large;
        }

        :not(:first-child) {
            flex-grow: 1;
            display: flex;
            align-items: center;
            font-family: $snm-font-heading;
            font-weight: bold;
            font-size: $snm-font-small;
            margin-left: 16px;
        }
    }

    .search {
        margin: 17px 0px;
        border-top: 1px solid $snm-color-border;
        border-bottom: 1px solid $snm-color-border;
        padding: 17px;

        :deep(label) {
            font-family: $snm-font-meta;
            font-weight: normal;
            font-size: $snm-font-smaller;
            line-height: 18px;
            color: var(--primary-color, $snm-color-element-dark);
        }
    }
}

.log {
  .status{
    margin:1rem 1rem 0;
  }
  .manage .search {
    background-color: var(--background-color, $snm-color-background-light);
    border: 1px solid $snm-color-border;

    .field.has-addons, .field.has-addons .control {
      width: 100%;
    }
  }
}

.pagination {
    display: flex;
    justify-content: center;
}

@media (min-width: $tablet-screen){
  .reportable .opportunity-card {
    width: 100%;
  }
}

@media (min-width: $fullsize-screen) {
  .explanation{
    margin: 2rem 0;
  }
  .reportable {
        display: flex;
        border: 1px solid $snm-color-border;
        border-radius: 10px;
        margin-bottom: 24px;
        padding: 0px;

        .opportunity-card {
            flex-grow: 1;
            margin:0;
            border:0;
        }

        .secondary {
          display:flex!important;
          background-color: #fff;
          flex-direction: column;
        }

        .actions {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: stretch;
            border-left: 1px solid $snm-color-border;
            padding: 1rem;
            background-color: var(--background-color, $snm-color-background-light);
            min-width: 230px;

            .action-button {
                flex-grow: 0;
            }
        }
    }

    .manage {
        .search {
            border: 0px;
            border-bottom: 1px solid $snm-color-border;
            border-radius: 10px;
        }
    }
    .log {
      .status{
        margin:1rem 0 0;
      }
      .opportunity-card{
        width: 100%;
        border: 1px solid $snm-color-border!important;
      }
      .manage .search .field.has-addons, .manage .search .field.has-addons .control {
        width: auto;
      }
    }
}

.bubble {
    position: relative;
    top: -0.25em;
    display: inline-flex;
    font-size: 8pt;
    color: #fff;
    background-color: #991b08;
    border-radius: 50%;
    align-items: center;
    justify-content: center;
    min-width: 1rem;
    height: 1rem;
    line-height: 1rem;
    margin-left: 0.5em;
}
</style>
