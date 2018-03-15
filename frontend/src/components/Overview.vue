<template>
  <div id="app">
      <h1>Abrechnungen</h1>

      <table>
        <thead>
          <th>Titel</th>
          <th>von</th>
          <th>bis</th>
        </thead>
        <tbody>
          <tr class="report" v-for="report in reports" :key="report.id">
            <td>
              <input type="text" name="title" v-model="report.title"
                     required minlength="8" maxlength="100" pattern="[^a-z][a-zA-Zäöuß0-9. ]+"
                     title="Bitte nur Buchstaben, Zahlen, Leerzeichen und Punkte verwenden">
            </td>
            <td>
              <input type="date" name="start_date" v-model="report.start_date"
                     required :min="mindate" :max="maxdate">
            </td>
            <td>
              <input type="date" name="end_date" v-model="report.end_date"
                     required :min="mindate" :max="maxdate">
            </td>
            <td>
              <router-link :to="{name: 'report', params: {id: report.id}}">Bearbeiten</router-link>
            </td>
          </tr>
        </tbody>
      </table>

      <button name="new-report" v-on:click="newReport">Neue Abrechnung</button>
  </div>
</template>

<script>
function formatDate (date) {
  return date.toISOString().split('T')[0]
}

export default {
  data () {
    return {
      mindate: '2017-07-01',
      maxdate: formatDate(new Date()),
      reports: []
    }
  },
  beforeMount () {
    this.$http.get('reports').then(response => {
      response.body.forEach(element => {
        element.in_db = true
        this.reports.push(element)
      })
    })
  },
  methods: {
    newReport () {
      let lastReport = this.reports[this.reports.length - 1]
      let reportTemplate = {
        id: lastReport.id + 1,
        title: '',
        start_date: lastReport.end_date,
        end_date: '2018-01-01',
        was_pdf_generated: false,
        in_db: false
      }
      this.reports.push(reportTemplate)
    }
  }
}
</script>

<style>
body {
  text-align: center;
}

input {
  background: none;
  border: 0;
}

button {
  border-radius: 0;
  color: white;
  background: #369;
  padding: 0.5em;
  margin: 1em;
}
</style>
