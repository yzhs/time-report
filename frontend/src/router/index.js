import Vue from 'vue'
import Router from 'vue-router'
import Report from '@/components/Report'
import Overview from '@/components/Overview'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/abrechnung',
      name: 'Abrechnung',
      component: Report
    },
    {
      path: '/',
      name: 'Übersicht',
      component: Overview
    }
  ]
})
