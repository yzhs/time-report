// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import Vue from 'vue'
import axios from 'axios'
import App from './App.vue'
import router from './router'

Vue.config.productionTip = false
Vue.prototype.$http = axios

axios.defaults.baseURL = 'http://localhost:8000/api'

/* eslint-disable no-new */
let v = new Vue({
  el: '#app',
  router,
  components: { App },
  template: '<App/>'
})
