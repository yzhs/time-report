<template>
  <div id="app">
    <nav>
      <router-link :to="{name: 'overview'}">Übersicht</router-link>
    </nav>
    <router-view/>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'

export default class App extends Vue {
  name = 'App'
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

nav > a {
  text-decoration: none;
  background: lightgrey;
  padding: 1em 1em 0.8em;
  color: #369;
}

nav > a.router-link-exact-active {
  background: #369;
  color: white;
}
</style>
