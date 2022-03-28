<template>
<article v-if="!hidden" class="opportunity-card" :class="{'rule': rule,'has-trash':trash, 'opp-loader':loader, 'widget':widget, 'owner':owner}">
  <template v-if="loader">
    <div class="primary">
      <div class="oc-loader-img linear-background" />
      <div style="flex:1">
        <!-- <div class="oc-loader-h2 linear-background" /> -->
        <div class="oc-loader-h1 linear-background" />
        <div class="oc-loader-p linear-background" />
        <div class="oc-loader-p linear-background" />
      </div>
    </div>
    <div class="secondary" />
  </template>

  <!-- widget -->
  <template v-else-if="widget">
    <div :class="widgetlayout">
      <a :href="'https://sciencenearme.org/' + opportunity.slug" class="primary" target="_blank">
        <img :src="image">
        <div>
          <h2>{{ subtitle }}</h2>
          <h1>{{ opportunity.title }}</h1>
          <small><vue-markdown :source="opportunity.short_desc" class="content" /></small>
          <opportunity-notice :opportunity="opportunity" mode="all" />
        </div>
      </a>
      <div class="secondary">
        <div class="info location">
          <location-icon />
          <div>
            <opportunity-location :opportunity="opportunity" :short="true" :shortstacked="true" />
            <opportunity-notice :opportunity="opportunity" mode="place" />
          </div>
        </div>
        <div class="info time">
          <time-icon />
          <div>
            <opportunity-time :opportunity="opportunity" />
            <opportunity-notice :opportunity="opportunity" mode="time" />
          </div>
        </div>
      </div>
    </div>
  </template>
  <!-- end widget -->

  <template v-else-if="owner">
    <nuxt-link :to="(partner !== null ? '/exchange/' + partner.uid + '/' : '/') + opportunity.slug" class="primary">
      <img :src="image">
      <div>
        <h1>{{ opportunity.title }}</h1>
        <opportunity-location :opportunity="opportunity" :short="true" class="info" />
        <opportunity-time :opportunity="opportunity" class="info" />
      </div>
    </nuxt-link>
    <div class="owner-actions">
          <action-button v-if="owner=='live' || owner=='draft'" tertiary @click="$router.push({name: partner !== null ? 'exchange-uid-edit-opp' : 'my-opportunity-uid', params: partner !== null ? {uid: partner.uid, opp: opportunity.uid} : {uid: opportunity.uid}})" class="no-mobile-edit"><div class="icon"><edit-icon /></div>Edit</action-button>
          <b-dropdown aria-role="list" position="is-bottom-left">
            <template #trigger="{ active }">
                <b-button class="more-btn"><div class="icon"><more-icon /></div></b-button>
            </template>
            <b-dropdown-item aria-role="listitem" @click="$router.push({name: 'my-opportunity-uid', params: {uid: opportunity.uid}})" class="mobile-edit">Edit</b-dropdown-item>
            <b-dropdown-item aria-role="listitem" @click="view">View</b-dropdown-item>
            <!-- <b-dropdown-item aria-role="listitem">Duplicate</b-dropdown-item> -->
            <b-dropdown-item v-if="trash" aria-role="listitem" @click="$emit('trash', opportunity)">Trash</b-dropdown-item>
        </b-dropdown>
    </div>
  </template>
  <!-- end owner -->

  <template v-else>
  <button v-if="trash" class="trash" @click="$emit('trash', opportunity)">
    <trash-icon />
  </button>
  <nuxt-link :to="(partner !== null ? '/exchange/' + partner.uid + '/' : '/') + opportunity.slug" class="primary">
    <img :src="image">
    <div>
      <h2>{{ subtitle }}</h2>
      <h1>{{ opportunity.title }}</h1>
      <small><vue-markdown :source="opportunity.short_desc" class="content" /></small>
      <opportunity-notice :opportunity="opportunity" mode="all" />
    </div>
  </nuxt-link>
  <div class="secondary">
    <div class="info location">
      <location-icon />
      <div>
        <opportunity-location :opportunity="opportunity" :short="true" />
        <opportunity-notice :opportunity="opportunity" mode="place" />
      </div>
    </div>
    <div class="info time">
      <time-icon />
      <div>
        <opportunity-time :opportunity="opportunity" />
        <opportunity-notice :opportunity="opportunity" mode="time" />
      </div>
    </div>
    <div class="info keywords">
      <keywords-icon />
      <opportunity-keywords :opportunity="opportunity" />
    </div>
  </div>
</template>
</article>
</template>

<script>
import VueMarkdown from "vue-markdown"

import OpportunityLocation from "~/components/OpportunityLocation"
import OpportunityTime from "~/components/OpportunityTime"
import OpportunityKeywords from "~/components/OpportunityKeywords"
import OpportunityNotice from "~/components/OpportunityNotice"

import NoImage from "~/assets/img/no-image-thumb.jpg"
import LocationIcon from '~/assets/img/location-marker.svg?inline'
import TimeIcon from '~/assets/img/calendar.svg?inline'
import KeywordsIcon from '~/assets/img/tag.svg?inline'
import TrashIcon from '~/assets/img/trash.svg?inline'
import EditIcon from '~/assets/img/edit-alt.svg?inline'
import MoreIcon from '~/assets/img/more.svg?inline'

export default {
    components: {
        VueMarkdown,

        OpportunityLocation,
        OpportunityTime,
        OpportunityKeywords,
        OpportunityNotice,

        LocationIcon,
        TimeIcon,
        KeywordsIcon,
        TrashIcon,
        EditIcon,
        MoreIcon
    },

    props: {
        opportunity: {
            type: [Object, null],
            required: false,
            default: null,
        },

        trash: {
            type: Boolean,
            required: false,
            default: false,
        },

        rule: {
            type: Boolean,
            required: false,
            default: true,
        },

        hidden: {
            type: Boolean,
            required: false,
            default: false,
        },
        loader: {
            type: Boolean,
            required: false,
            default: false,
        },
        widget: {
            type: Boolean,
            required: false,
            default: false,
        },
        widgetlayout: {
            type: String,
            required: false,
            default: undefined,
        },
        owner: {
            type: String,
            required: false,
            default: undefined,
        },
        partner: {
            type: [Object, null],
            required: false,
            default: null,
        },
    },

    computed: {
        image() {
            return this.opportunity.image_url || NoImage;
        },

        subtitle() {
            return this.opportunity.organization_name || ""; //this.opportunity.partner_name;
        },
    },

    methods: {
        view() {
            window.open((this.partner !== null ? '/exchange/' + this.partner.uid + '/' : '/') + this.opportunity.slug, '_blank');
        }
    },
}
</script>

<style lang="scss" scoped>


.opportunity-card {
  display: flex;
  flex-direction: column;


    &.rule {
        border-bottom: 2px solid $snm-color-border;
    }
    &.rule:first-child {
      border-top: 2px solid $snm-color-border;
    }
}

.has-trash {
  position: relative;

  .trash {
      position: absolute;
      top:0.5rem;
      right:1rem;
      display: flex;
      align-items: center;
      justify-content: center;
      width: 40px;
      height: 40px;
      color: var(--secondary-color, $snm-color-element-med);
      border: 1px solid var(--secondary-color, $snm-color-element-med);
      border-radius: 10px;
      box-shadow: 0px 3px 6px $snm-color-shadow;
      cursor: pointer;

      svg * {
          fill: currentColor;
      }
  }
  .primary {
    padding-right: 2rem;
  }

}

.primary {
    display: flex;
    padding: 1rem;
    flex: 0 1 100px;
    align-items:center;

    img, .oc-loader-img {
        width: 80px;
        height: 80px;
        display: block;
        border: none;
        border-radius: 10px;
        overflow: hidden;
        flex-shrink: 0;
        flex-grow: 0;
        object-fit: cover;
        object-position: center center;
        margin: 0px 16px 0px 0px;
    }

    div {
        h2, .oc-loader-h2 {
            font-size: $snm-font-smaller;
            font-weight: bold;
            font-family: $snm-font-content;
            letter-spacing: 0px;
            color: var(--primary-color, $snm-color-element-dark);
            line-height: 16px;
        }

        h1, .oc-loader-h1 {
            text-decoration: underline;
            color: var(--secondary-color, $snm-color-element-med);
            font-family: $snm-font-heading;
            letter-spacing: 0px;
            font-size: $snm-font-small;
            font-weight: bold;
            line-height: 22px;
            margin-bottom: 8px;
        }
        h1 {
          line-height:1.2;
        }

        small, .oc-loader-p {
            display: none;
        }
    }
}

.secondary {
    font-size: rem(14px);
    padding: 0.5rem 1rem;
    flex: 1;
    background-color: var(--background-color, $snm-color-background-light);

    > div {
      margin-bottom: 0.5rem;
    }

    .info {
        display: flex;
        align-items: flex-start;

        svg,img {
            display: block;
            width: 10px;
            height: 12px;
            flex-grow: 0;
            flex-shrink: 0;
            margin-right: 5px;
            margin-top: 0.4em;
        }

        .opportunity-notice {
            display: none;
            font-size: $snm-font-smallest;
            line-height: 14px;
        }
    }
}
#homepage .primary {
  padding: 1rem;
  flex: 0 1 130px;
}
#homepage .secondary {
  display: none;
}
.reportable {
  .secondary {
    display: none;
  }
}
@media (min-width: $tablet-screen) {
  #homepage .secondary {
    display: block;
    background-color: var(--background-color, $snm-color-background-light);
    padding: 1rem;
  }

  .opportunity-card {
    width: calc(50% - 0.5rem);
    border: 1px solid $snm-color-border!important;
    margin-bottom: 1rem;
    border-radius: rem(6px);
  }

  #results {
    .opportunity-card {
      width: 100%;
      align-self: start;
    }
  }

}

@media (min-width: $fullsize-screen) {

    .primary {
        div {
            small, .oc-loader-p {
                display: block;
                font-family: $snm-font-content;
                font-size: $snm-font-smaller;
                color: var(--primary-color, $snm-color-element-dark);
                letter-spacing: 0.14px;
            }

            .opportunity-notice {
                display: none;
            }
        }
    }

    .secondary {
        display: flex;
        flex-wrap: wrap;

        .info {
            width: 20vw;
            min-width: 13rem;

            .opportunity-notice {
                display: block;
            }
        }

        > :last-child {
            width: 100%;
            margin-bottom:0;
        }
    }
    .reportable {
      .opportunity-card {
        border: 0!important;
      }
      .secondary {
        display: flex;
        background-color: var(--background-color, #fff);
      }

    }
}


@keyframes placeHolderShimmer{
    0%{
        background-position: -468px 0
    }
    100%{
        background-position: 468px 0
    }
}
.linear-background {
    animation-duration: 1s;
    animation-fill-mode: forwards;
    animation-iteration-count: infinite;
    animation-name: placeHolderShimmer;
    animation-timing-function: linear;
    background: #f6f7f8;
    background: linear-gradient(to right, #eeeeee 8%, #dddddd 18%, #eeeeee 33%);
    background-size: 1000px 104px;
    position: relative;
    overflow: hidden;
}

.oc-loader-h1, .oc-loader-h2, .oc-loader-p {
  display: block;
  height: 36px;
  width: 100%;
  margin-bottom:8px;
}

.oc-loader-h2 {
  height: 14px;
}

.oc-loader-p {
  height:11px;
  margin-bottom:3px;
}

.opp-loader {
  max-height: 200px;
}

/************* WIDGET ********************/
.opportunity-card.widget {
  width:100%;
  border:0!important;


  .short-thin, .tall-thin {
    width:100%;
    border:0;

    .primary {
      display: block;
      padding:0 10px;
      img {
        width:100%;
        max-height:92px;
        object-fit: contain;
        background-color:#efefef;
        margin-bottom:4px;
        border:1px solid $border;
      }
      h1 {
        line-height:1.1;
      }
      h2 {
        font-size:12px;
        line-height:1;
        margin-bottom:2px;
      }
    }

    .secondary {
      width:100%;
      padding:0 10px;
      background-color:var(--background-color, #fff);
      .info {
        width:100%;
        min-width:1px;
        line-height:1.2;
        font-size:13px;
        svg {
          margin-top:0.15rem;
        }
      }
    }

  }
}
.tall-thin .opportunity-card.widget:not(:first-child), .short-thin .opportunity-card.widget:not(:first-child) {
  border-top:1px solid $border!important;
  border-radius:0;
  padding-top:10px;
}

.tall-wide, .short-wide {
  .opportunity-card.widget {
    border:1px solid $border!important;
    border-radius:6px;
    margin:8px;
    width: calc(100% - 16px);
  }
}

/*** OWNER **/
.owner.opportunity-card {
  display:flex;
  justify-content: space-between;
  flex-direction:row;
  h1 {
    margin-bottom:2px;
  }
  a {
    flex:auto;
  }
  .info {
    font-size:12px;
    color: #000;
    line-height:1.1;
    margin-bottom:2px;
  }
  .owner-actions {
    background-color: #F5F5F5;
    display:flex;
    align-items:center;
    border-left:1px solid $snm-color-border;
    padding:0 1rem;
  }
  .more-btn {
    background-color:transparent;
    border:0;
    display:flex;
    align-items:center;
    .icon {
      position:relative;
      top:3px;
    }
  }
}

@media (min-width:700px){

}

@media (max-width:699px){

  button.action-button.no-mobile-edit {
    display:none;
  }
  .mobile-edit {
    display:block;
  }

  .owner .primary img, .primary .oc-loader-img {
    display:none;
  }
}

</style>
