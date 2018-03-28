import Vue from 'vue'
import Router, { RouteConfig } from 'vue-router'
import Employees from '../components/Employees.vue'
import Overview from '../components/Overview.vue'
import Report from '../components/Report.vue'

Vue.use(Router)

export default new Router({
  routes: [
    {
      path: '/abrechnung/:id',
      name: 'report',
      component: Report
    } as RouteConfig,
    {
      path: '/mitarbeiter',
      name: 'employees',
      component: Employees
    },
    {
      path: '/',
      name: 'overview',
      component: Overview
    }
  ]
})
