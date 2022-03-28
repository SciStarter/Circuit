<template>
<div class="your-opportunities snm-container">
  <div class="flex-header">
    <h1>Your Opportunities</h1>
    <action-button primary @click="$router.push({name: 'my-submit-opportunity'})" class="add-btn"><div class="icon"><add-icon /></div>Add a new opportunity</action-button>
  </div>

  <div class="nav-tab-wrapper">
  <ul class="nav-tabs">
      <li><a class="tab-link":class="{'active':state==1}" @click="state=1">Current, Live Opportunities</a></li>
      <li><a class="tab-link":class="{'active':state==2}" @click="state=2">Draft, Unpublished &amp; Expired</a></li>
      <li><a class="tab-link":class="{'active':state==3}" @click="state=3">Trashed</a></li>
      <li class="push-right"><action-button text2 @click="export_records">Export Records</action-button></li>
  </ul>
  </div>

  <div v-if="state==1">
    <div class="flex-header filter-actions">
      <div class="flex filter-area">
        <h2>Current, Live Opportunities</h2>
        <filter-icon class="filter" @click="show_filters_current = !show_filters_current" />
      </div>

      <transition name="slide">
      <div class="flex header-actions" :class="{'show_filters':show_filters_current}">
        <b-field label="Search" label-position="inside" data-context="find-keywords">
          <b-input ref="search_keywords" v-model="live_search" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
        </b-field>
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="live_from"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="live_until"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
        <b-field class="mobile-fix">
          <b-button @click="load_live(0)">Search</b-button>
        </b-field>
      </div>
    </transition>
    </div>

    <section id="results">
      <template v-if="live.matches.length > 0">
        <opportunity-card v-for="(opp, i) in live.matches" :key="opp.uid" :opportunity="opp" owner="live" trash @trash="trash_live(i)"/>
        <pagination :page-index="live.pagination.page_index" :last-page="live.pagination.last_page" @switch="load_live($event)" />
      </template>
      <template v-else>
        <div class="alert no-results" style="margin-bottom:2rem;">
          <p>No live opportunties. Add some!</p>
        </div>
      </template>
    </section>

  </div><!-- state 1 -->

  <div v-if="state==2">
    <div class="flex-header filter-actions">
      <div class="flex filter-area">
        <h2>Draft &amp; Unpublished Opportunities</h2>
        <filter-icon class="filter" @click="show_filters_draft = !show_filters_draft" />
      </div>
      <div class="flex header-actions" :class="{'show_filters':show_filters_draft}">
        <b-field label="Search" label-position="inside" data-context="find-keywords">
          <b-input ref="search_keywords" v-model="draft_search" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
        </b-field>
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="draft_from"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="draft_until"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
        <b-field class="mobile-fix">
          <b-button @click="load_draft(0)">Search</b-button>
        </b-field>
      </div>
    </div>
    <section id="results">
      <template v-if="draft.matches.length > 0">
        <opportunity-card v-for="(opp, i) in draft.matches" :key="opp.uid" :opportunity="opp" owner="draft" trash @trash="trash_draft(i)"/>
        <pagination :page-index="draft.pagination.page_index" :last-page="draft.pagination.last_page" @switch="load_draft($event)" />
      </template>
      <template v-else>
        <div class="alert no-results" style="margin-bottom:2rem;">
          <p>No results.</p>
        </div>
      </template>
    </section>
  </div><!-- state 2 -->

  <div v-if="state==3">
    <div class="flex-header filter-actions">
      <div class="flex filter-area">
        <h2>Expired and Trashed Opportunities</h2>
        <filter-icon class="filter" @click="show_filters_trash = !show_filters_trash" />
      </div>
      <div class="flex header-actions" :class="{'show_filters':show_filters_trash}">
        <b-field label="Search" label-position="inside" data-context="find-keywords">
          <b-input ref="search_keywords" v-model="expired_search" :name="'new-' + Math.random()" placeholder="e.g. astronomy, bar crawl" icon="magnify" />
        </b-field>
        <b-field label="From" label-position="inside" data-context="find-beginning" class="date">
          <b-datepicker
            v-model="expired_from"
            editable
            icon="calendar-today"
            />
        </b-field>
        <b-field label="Until" label-position="inside" data-context="find-ending" class="date">
          <b-datepicker
            v-model="expired_until"
            editable
            position="is-bottom-left"
            icon="calendar-today"
            />
        </b-field>
        <b-field class="mobile-fix">
          <b-button @click="load_expired(0)">Search</b-button>
        </b-field>
      </div>
    </div>
    <section id="results">
      <template v-if="expired.matches.length > 0">
        <opportunity-card v-for="(opp, i) in expired.matches" :key="opp.uid" :opportunity="opp" owner="past" />
        <pagination :page-index="expired.pagination.page_index" :last-page="expired.pagination.last_page" @switch="load_expired($event)" />
      </template>
      <template v-else>
        <div class="alert no-results" style="margin-bottom:2rem;">
          <p>No results.</p>
        </div>
      </template>
    </section>
  </div><!-- state 3 -->


  <b-modal
    v-model="show_delete_confirm"
    has-modal-card
    trap-focus
    :destroy-on-hide="false"
    aria-role="dialog"
    aria-label="Show tooltip"
    aria-modal
    >
    <div class="card">
      <h2>Confirm Delete <span class="close" @click="show_delete_confirm = false">&times;</span></h2>
      <p>Once deleted, this opportunity and all of its data will be removed from Science Near Me.</p>
      <div>
          <action-button primary>Confirm Delete</action-button>
          <action-button tertiary @click="show_delete_confirm = false">Cancel</action-button>
      </div>

    </div>
  </b-modal>

</div>
</template>

<script>

import AddIcon from '~/assets/img/submit-opportunity.svg?inline'
import FilterIcon from '~/assets/img/filter.svg?inline'

export default {
    name: "MyOpportunities",

    components: {
        AddIcon,
        FilterIcon
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
            context.error({
                statusCode: 401,
                message: "Authentication required"
            });
        }

        let live = {
            pagination: {
                "page_index": 0,
                "per_page": 10,
                "last_page": 0,
                "total": 0,
            },
            matches: [],
        };

        let draft = {
            pagination: {
                "page_index": 0,
                "per_page": 10,
                "last_page": 0,
                "total": 0,
            },
            matches: [],
        };

        let expired = {
            pagination: {
                "page_index": 0,
                "per_page": 10,
                "last_page": 0,
                "total": 0,
            },
            matches: [],
        };

        try {
            live = await context.$axios.$get('/api/ui/finder/search?mine=true&current=true&sort=alphabetical', context.store.state.auth);
            draft = await context.$axios.$get('/api/ui/finder/search?mine=true&current=false&withdrawn=true&sort=alphabetical', context.store.state.auth);
            expired = await context.$axios.$get('/api/ui/finder/search?mine=true&current=false&withdrawn=false&sort=alphabetical', context.store.state.auth);
        }
        catch(err) {
            context.error({
                statusCode: err.response.status,
                message: err.response.data
            });
        }

        return {
            live,
            draft,
            expired,
            live_search: "",
            draft_search: "",
            expired_search: "",
            live_from: null,
            live_until: null,
            draft_from: null,
            draft_until: null,
            expired_from: null,
            expired_until: null,
        }
    },

    data() {
        return {
            state:1,
            show_delete_confirm: false,
            show_filters_current: false,
            show_filters_draft: false,
            show_filters_trash: false,
        }
    },

    computed: {
        user() {
            return this.$store.state.user;
        },
    },

    methods: {
        async export_records() {
            let data = await this.$axios.$get('/api/ui/profile/opportunities.csv', this.$store.state.auth);
            let blob = new Blob([data.content], { type: "text/csv" });
            let link = document.createElement("a");
            link.href = URL.createObjectURL(blob);
            link.download = data.filename;
            link.click();
            URL.revokeObjectURL(link.href);
        },

        async trash_live(idx) {
            let opp = this.live.matches[idx];
            this.live.matches.splice(idx, 1);
            opp.accepted = false;
            await this.$axios.$put('/api/ui/entity/' + opp.slug, opp, this.$store.state.auth);
        },

        async trash_draft(idx) {
            let opp = this.draft.matches[idx];
            this.draft.matches.splice(idx, 1);
            opp.accepted = false;
            await this.$axios.$put('/api/ui/entity/' + opp.slug, opp, this.$store.state.auth);
        },

        async load_live(page) {
            this.live = await this.$axios.$get('/api/ui/finder/search?mine=true&current=true&sort=alphabetical' + (this.live_search ? '&text=' + encodeURIComponent(this.live_search) : '') + (this.live_from ? '&beginning=' + this.live_from.toISOString() : '') + (this.live_until ? '&ending=' + this.live_until.toISOString() : '') + '&page=' + page, this.$store.state.auth);
        },

        async load_draft(page) {
            this.draft = await this.$axios.$get('/api/ui/finder/search?mine=true&current=false&withdrawn=true&sort=alphabetical' + (this.draft_search ? '&text=' + encodeURIComponent(this.draft_search) : '') + (this.draft_from ? '&beginning=' + this.draft_from.toISOString() : '') + (this.draft_until ? '&ending=' + this.draft_until.toISOString() : '') + '&page=' + page, this.$store.state.auth);
        },

        async load_expired(page) {
            this.expired = await this.$axios.$get('/api/ui/finder/search?mine=true&current=false&withdrawn=false&sort=alphabetical' + (this.expired_search ? '&text=' + encodeURIComponent(this.expired_search) : '') + (this.expired_from ? '&beginning=' + this.expired_from.toISOString() : '') + (this.expired_until ? '&ending=' + this.expired_until.toISOString() : '') + '&page=' + page, this.$store.state.auth);
        },
    },
}
</script>

<style lang="scss" scoped>
.flex {
  display:flex;
}
.flex-header {
  display:flex;
  align-items:center;
  justify-content:space-between;
  margin-bottom:1rem;

  h2 {
    color: var(--secondary-color, $snm-color-element-med);
    font-size:1.4rem;
    font-weight:bold;
    font-family: $snm-font-heading;
  }
  .datepicker {
    width:150px;
  }
}

h1 {
  font-family: $snm-font-heading;
  font-size: 1.8rem;
  font-weight:bold;
  color: var(--secondary-color, $snm-color-element-med);
  margin-bottom:0;
}
.header-actions > div {
  margin-left:1rem;
}

.header-actions button.button {
    margin-top: 0.25rem;
}

.push-right {
  margin-left:auto;
  font-size:16px!important;
  align-self:center;
}
#results {
  margin-bottom:4rem;
}

@media (max-width:1159px) {
  .snm-container {
    padding:1rem;
  }
  .flex-header.filter-actions {
    flex-direction:column;
    align-items: flex-start;
    .header-actions > div:first-child {
      margin-left:0;
    }
    h2 {
      margin-bottom:1rem;
    }
  }
}

@media (max-width:767px) {
  #results  {
    margin-left:-1rem;
    margin-right:-1rem;
  }
}

.filter {
  display:none;
}

@media (max-width:600px) {
  .filter-area {
    width:100%;
    justify-content:space-between;
    h2 {
      font-size:18px;
    }
  }
  .filter {
    display:block;
    width:30px;
    height:30px;
    path {
      fill: $snm-color-background-meddark
    }
  }
  .header-actions {
    top:0;
    left:0;
    background-color:$snm-color-background-medlight;
    padding:20px;
    flex-wrap: wrap;
    justify-content:space-between;
    display:none;
    > div:first-child {
      min-width:100%!important;
    }
    .field.is-floating-label, .field.is-floating-in-label{
      margin:10px 0;
      width:48%;
      .datepicker {
        width:100%;
      }
    }
    .mobile-fix {
      margin:auto;
    }
    &.show_filters {
      display:flex;
    }
  }
  .add-btn {
    display:none!important;
  }
  .no-results {
    padding:20px;
  }
}

.nav-tab-wrapper {
  width:100%;
  overflow:auto;
  .nav-tabs {
    min-width: 680px
  }
}
.nav-tab-wrapper::-webkit-scrollbar {
  display: none;
}
.slide-enter-active {
    -moz-transition-duration: 0.2s;
    -webkit-transition-duration: 0.2s;
    -o-transition-duration: 0.2s;
    transition-duration: 0.2s;
    -moz-transition-timing-function: ease-in;
    -webkit-transition-timing-function: ease-in;
    -o-transition-timing-function: ease-in;
    transition-timing-function: ease-in;
}

.slide-leave-active {
    -moz-transition-duration: 0.1s;
    -webkit-transition-duration: 0.1s;
    -o-transition-duration: 0.1s;
    transition-duration: 0.1s;
    -moz-transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
    -webkit-transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
    -o-transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
    transition-timing-function: cubic-bezier(0, 1, 0.5, 1);
}

.slide-enter-to, .slide-leave {
    max-height: 100px;
    overflow: hidden;
}

.slide-enter, .slide-leave-to {
    overflow: hidden;
    max-height: 0;
}
</style>
