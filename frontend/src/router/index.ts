import Vue from 'vue'
import Router, { RouteConfig } from 'vue-router'
import Report from '../components/Report.vue'
import Overview from '../components/Overview.vue'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/abrechnung/:id',
      name: 'report',
      component: Report
    } as RouteConfig,
    {
      path: '/',
      name: 'overview',
      component: Overview
    }
  ]
})
