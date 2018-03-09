import Vue from 'vue'
import Router from 'vue-router'
import Report from '@/components/Report'
import Weeks from '@/components/Weeks'

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
      name: 'A-D Wochen',
      component: Weeks
    }
  ]
})
