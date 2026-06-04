import Vue from 'vue'
import Router from 'vue-router'
import { normalizeURL, decode } from 'ufo'
import { interopDefault } from './utils'
import scrollBehavior from './router.scrollBehavior.js'

const _6ae34234 = () => interopDefault(import('../pages/account.vue' /* webpackChunkName: "pages/account" */))
const _5ce8ff5f = () => interopDefault(import('../pages/display-opportunities.vue' /* webpackChunkName: "pages/display-opportunities" */))
const _27424928 = () => interopDefault(import('../pages/evolveme.vue' /* webpackChunkName: "pages/evolveme" */))
const _c3f0ea8c = () => interopDefault(import('../pages/exchange.vue' /* webpackChunkName: "pages/exchange" */))
const _13f8c3f6 = () => interopDefault(import('../pages/exchange/_uid.vue' /* webpackChunkName: "pages/exchange/_uid" */))
const _fbb80c0e = () => interopDefault(import('../pages/exchange/_uid/index.vue' /* webpackChunkName: "pages/exchange/_uid/index" */))
const _4bcb9d10 = () => interopDefault(import('../pages/exchange/_uid/login.vue' /* webpackChunkName: "pages/exchange/_uid/login" */))
const _4cc23a66 = () => interopDefault(import('../pages/exchange/_uid/opps.vue' /* webpackChunkName: "pages/exchange/_uid/opps" */))
const _9bf1c9a2 = () => interopDefault(import('../pages/exchange/_uid/partner.vue' /* webpackChunkName: "pages/exchange/_uid/partner" */))
const _7a01f201 = () => interopDefault(import('../pages/exchange/_uid/signup.vue' /* webpackChunkName: "pages/exchange/_uid/signup" */))
const _1ca94101 = () => interopDefault(import('../pages/exchange/_uid/submit.vue' /* webpackChunkName: "pages/exchange/_uid/submit" */))
const _651ea208 = () => interopDefault(import('../pages/exchange/_uid/edit/_opp.vue' /* webpackChunkName: "pages/exchange/_uid/edit/_opp" */))
const _ff254e9e = () => interopDefault(import('../pages/exchange/_uid/_slug.vue' /* webpackChunkName: "pages/exchange/_uid/_slug" */))
const _96a06260 = () => interopDefault(import('../pages/find.vue' /* webpackChunkName: "pages/find" */))
const _6e78337a = () => interopDefault(import('../pages/forgot.vue' /* webpackChunkName: "pages/forgot" */))
const _bc8a5f5c = () => interopDefault(import('../pages/geo-explorer.vue' /* webpackChunkName: "pages/geo-explorer" */))
const _63cd7a62 = () => interopDefault(import('../pages/login.vue' /* webpackChunkName: "pages/login" */))
const _637ffd5e = () => interopDefault(import('../pages/login-scistarter.vue' /* webpackChunkName: "pages/login-scistarter" */))
const _824d2dfa = () => interopDefault(import('../pages/my.vue' /* webpackChunkName: "pages/my" */))
const _4081d7b0 = () => interopDefault(import('../pages/my/data-overview.vue' /* webpackChunkName: "pages/my/data-overview" */))
const _4536f534 = () => interopDefault(import('../pages/my/goals.vue' /* webpackChunkName: "pages/my/goals" */))
const _34abb2bd = () => interopDefault(import('../pages/my/hosts-explorer.vue' /* webpackChunkName: "pages/my/hosts-explorer" */))
const _4d2463a5 = () => interopDefault(import('../pages/my/opportunities.vue' /* webpackChunkName: "pages/my/opportunities" */))
const _3568bcfc = () => interopDefault(import('../pages/my/opportunity-data-explorer.vue' /* webpackChunkName: "pages/my/opportunity-data-explorer" */))
const _43a3dfef = () => interopDefault(import('../pages/my/organization.vue' /* webpackChunkName: "pages/my/organization" */))
const _1f2a90c6 = () => interopDefault(import('../pages/my/profile.vue' /* webpackChunkName: "pages/my/profile" */))
const _578d770a = () => interopDefault(import('../pages/my/saved.vue' /* webpackChunkName: "pages/my/saved" */))
const _7672f2d8 = () => interopDefault(import('../pages/my/science.vue' /* webpackChunkName: "pages/my/science" */))
const _462dce15 = () => interopDefault(import('../pages/my/snm-data-overview.vue' /* webpackChunkName: "pages/my/snm-data-overview" */))
const _0fde4eda = () => interopDefault(import('../pages/my/submit-opportunity.vue' /* webpackChunkName: "pages/my/submit-opportunity" */))
const _da46e5ee = () => interopDefault(import('../pages/my/opportunity/_uid.vue' /* webpackChunkName: "pages/my/opportunity/_uid" */))
const _623bbeef = () => interopDefault(import('../pages/signup.vue' /* webpackChunkName: "pages/signup" */))
const _6e5d321b = () => interopDefault(import('../pages/widget.vue' /* webpackChunkName: "pages/widget" */))
const _cbb4516a = () => interopDefault(import('../pages/index.vue' /* webpackChunkName: "pages/index" */))
const _cf2193fa = () => interopDefault(import('../pages/_slug.vue' /* webpackChunkName: "pages/_slug" */))

const emptyFn = () => {}

Vue.use(Router)

export const routerOptions = {
  mode: 'history',
  base: '/',
  linkActiveClass: 'nuxt-link-active',
  linkExactActiveClass: 'nuxt-link-exact-active',
  scrollBehavior,

  routes: [{
    path: "/account",
    component: _6ae34234,
    name: "account"
  }, {
    path: "/display-opportunities",
    component: _5ce8ff5f,
    name: "display-opportunities"
  }, {
    path: "/evolveme",
    component: _27424928,
    name: "evolveme"
  }, {
    path: "/exchange",
    component: _c3f0ea8c,
    name: "exchange",
    children: [{
      path: ":uid?",
      component: _13f8c3f6,
      children: [{
        path: "",
        component: _fbb80c0e,
        name: "exchange-uid"
      }, {
        path: "login",
        component: _4bcb9d10,
        name: "exchange-uid-login"
      }, {
        path: "opps",
        component: _4cc23a66,
        name: "exchange-uid-opps"
      }, {
        path: "partner",
        component: _9bf1c9a2,
        name: "exchange-uid-partner"
      }, {
        path: "signup",
        component: _7a01f201,
        name: "exchange-uid-signup"
      }, {
        path: "submit",
        component: _1ca94101,
        name: "exchange-uid-submit"
      }, {
        path: "edit/:opp?",
        component: _651ea208,
        name: "exchange-uid-edit-opp"
      }, {
        path: ":slug",
        component: _ff254e9e,
        name: "exchange-uid-slug"
      }]
    }]
  }, {
    path: "/find",
    component: _96a06260,
    name: "find"
  }, {
    path: "/forgot",
    component: _6e78337a,
    name: "forgot"
  }, {
    path: "/geo-explorer",
    component: _bc8a5f5c,
    name: "geo-explorer"
  }, {
    path: "/login",
    component: _63cd7a62,
    name: "login"
  }, {
    path: "/login-scistarter",
    component: _637ffd5e,
    name: "login-scistarter"
  }, {
    path: "/my",
    component: _824d2dfa,
    name: "my",
    children: [{
      path: "data-overview",
      component: _4081d7b0,
      name: "my-data-overview"
    }, {
      path: "goals",
      component: _4536f534,
      name: "my-goals"
    }, {
      path: "hosts-explorer",
      component: _34abb2bd,
      name: "my-hosts-explorer"
    }, {
      path: "opportunities",
      component: _4d2463a5,
      name: "my-opportunities"
    }, {
      path: "opportunity-data-explorer",
      component: _3568bcfc,
      name: "my-opportunity-data-explorer"
    }, {
      path: "organization",
      component: _43a3dfef,
      name: "my-organization"
    }, {
      path: "profile",
      component: _1f2a90c6,
      name: "my-profile"
    }, {
      path: "saved",
      component: _578d770a,
      name: "my-saved"
    }, {
      path: "science",
      component: _7672f2d8,
      name: "my-science"
    }, {
      path: "snm-data-overview",
      component: _462dce15,
      name: "my-snm-data-overview"
    }, {
      path: "submit-opportunity",
      component: _0fde4eda,
      name: "my-submit-opportunity"
    }, {
      path: "opportunity/:uid?",
      component: _da46e5ee,
      name: "my-opportunity-uid"
    }]
  }, {
    path: "/signup",
    component: _623bbeef,
    name: "signup"
  }, {
    path: "/widget",
    component: _6e5d321b,
    name: "widget"
  }, {
    path: "/",
    component: _cbb4516a,
    name: "index"
  }, {
    path: "/:slug",
    component: _cf2193fa,
    name: "slug"
  }],

  fallback: false
}

export function createRouter (ssrContext, config) {
  const base = (config._app && config._app.basePath) || routerOptions.base
  const router = new Router({ ...routerOptions, base  })

  // TODO: remove in Nuxt 3
  const originalPush = router.push
  router.push = function push (location, onComplete = emptyFn, onAbort) {
    return originalPush.call(this, location, onComplete, onAbort)
  }

  const resolve = router.resolve.bind(router)
  router.resolve = (to, current, append) => {
    if (typeof to === 'string') {
      to = normalizeURL(to)
    }
    return resolve(to, current, append)
  }

  return router
}
