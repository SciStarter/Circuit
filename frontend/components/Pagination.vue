<template>
<div class="pagination-selector">
  <div>
    <action-button primary title="go to first page" :disabled="!more_begin" @click="$emit('switch', firstPage)">
      <span class="center-lowercase">&laquo;</span>
    </action-button>
    <action-button primary title="go to previous page" :disabled="pageIndex <= firstPage" @click="$emit('switch', pageIndex - 1)">
      <span class="center-lowercase">&lsaquo;</span>
    </action-button>
    <action-button primary title="go to next page" :disabled="pageIndex >= lastPage" class="mobile-only" @click="$emit('switch', pageIndex + 1)">
      <span class="center-lowercase">&rsaquo;</span>
    </action-button>
    <action-button primary title="go to last page" :disabled="!more_end" class="mobile-only" @click="$emit('switch', lastPage)">
      <span class="center-lowercase">&raquo;</span>
    </action-button>
  </div>
  <div>
    <span v-if="more_begin">&hellip;</span>
    <a v-for="idx in range_begin" :title="'go to page ' + (idx + indexOffset)" @click="$emit('switch', idx)">{{ idx + indexOffset }}</a>
    <span>{{ pageIndex + indexOffset }}</span>
    <a v-for="idx in range_end" :title="'go to page ' + (idx + indexOffset)" @click="$emit('switch', idx)">{{ idx + indexOffset }}</a>
    <span v-if="more_end">&hellip;</span>
  </div>
  <div class="no-mobile">
    <action-button primary title="go to next page" :disabled="pageIndex >= lastPage" @click="$emit('switch', pageIndex + 1)">
      <span class="center-lowercase">&rsaquo;</span>
    </action-button>
    <action-button primary title="go to last page" :disabled="!more_end" @click="$emit('switch', lastPage)">
      <span class="center-lowercase">&raquo;</span>
    </action-button>
  </div>
</div>
</template>

<script>
import range from 'lodash/range'

export default {
    name: "Pagination",

    props: {
        pageIndex: {
            type: Number,
            required: false,
            default: 0,
        },

        firstPage: {
            type: Number,
            required: false,
            default: 0,
        },

        lastPage: {
            type: Number,
            required: false,
            default: 0,
        },

        total: {
            type: Number,
            required: false,
            default: 0,
        },

        indexOffset: {
            type: Number,
            required: false,
            default: 1,
        },
    },

    computed: {
        begin() {
            return Math.max(this.firstPage, this.pageIndex - 4);
        },

        end() {
            return Math.min(this.lastPage, this.pageIndex + 4);
        },

        more_begin() {
            return this.begin > this.firstPage;
        },

        more_end() {
            return this.end < this.lastPage;
        },

        range_begin() {
            return range(this.begin, this.pageIndex);
        },

        range_end() {
            return range(this.pageIndex + 1, this.end + 1);
        },
    },
}
</script>

<style lang="scss" scoped>
div.pagination-selector {
    display: flex;
    margin: 1rem;
    justify-content: center;
    align-items: center;

    >div {
        display: flex;

        >* {
            flex-grow: 0;
            font-size: 2rem;
            line-height: 0px;
            margin: 0.5rem 0.25rem;
            height: 2rem;
            display: flex;
            justify-content: center;
            align-items: center;
            line-height: 0px;
        }
    }

    .center-lowercase {
        position: relative;
        top: -0.1ex;
    }
}

@media (max-width: $mobile-screen) {
    div.pagination-selector {
        flex-direction: column;
    }
}

</style>
