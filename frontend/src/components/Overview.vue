<template>
  <div id="app">
      <h1>Abrechnungen</h1>

      <ul>
        <li v-for="report in reports" :key="report.id">
          {{report.title}}
        </li>
      </ul>
  </div>
</template>

<script>
export default {
  data () {
    return {
      reports: []
    }
  },
  beforeMount () {
    this.$http.get('reports').then(response => {
      response.body.forEach(element => {
        this.reports.push(element)
      })
    })
  }
}
</script>

<style>
ul {
  list-style-type: none;
}

li {
  font-size: 120%;
}
</style>
