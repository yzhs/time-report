<template>
  <div id="app">
    <nav>
      <router-link :to="{name: 'overview'}">Übersicht</router-link>
      <router-link :to="{name: 'report', params: {id: 1}}">Abrechnung</router-link>
    </nav>
    <router-view/>
  </div>
</template>

<script>
export default {
  name: 'App'
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
  background: lightgrey;
  padding: 1em;
  color: #369;
}

nav > a.router-link-exact-active {
  background: #369;
  color: white;
}
</style>
