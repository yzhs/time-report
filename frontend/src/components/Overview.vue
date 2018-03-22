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
          <tr class="report" v-for="(report, index) in reports" :key="report.id">
            <td>
              <input type="text" class="title" name="title" v-model="report.title"
                     required minlength="8" maxlength="100" pattern="[^a-z][a-zA-Zäöuß0-9. ]+"
                     title="Bitte nur Buchstaben, Zahlen, Leerzeichen und Punkte verwenden"
                     v-on:change="updateReport(index)">
            </td>
            <td>
              <input type="date" name="start_date" v-model="report.start_date"
                     required :min="mindate" :max="maxdate" v-on:change="updateReport(index)">
            </td>
            <td>
              <input type="date" name="end_date" v-model="report.end_date"
                     required :min="mindate" :max="maxdate" v-on:change="updateReport(index)">
            </td>
            <td>
              <router-link class="edit" :to="{name: 'report', params: {id: report.id}}">
                Bearbeiten
              </router-link>
            </td>
            <td>
              <a class="edit" href="#" v-if="report.was_pdf_generated" v-on:click="downloadPdf(index)">PDF speichern</a>
              <a class="edit" href="#" v-else v-on:click="downloadPdf(index)">PDF erzeugen</a>
            </td>
          </tr>
        </tbody>
      </table>

      <button v-if="numReports() === 0 || reports[numReports() - 1].was_pdf_generated" name="new-report" v-on:click="newReport">Neue Abrechnung</button>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import axios from 'axios'

function formatDate (date: Date) {
  return date.toISOString().split('T')[0]
}

let useJsonHeader = {
  headers: {
    'Content-Type': 'application/json'
  }
}

export class Report {
  constructor(public id: number, public title: String, public start_date: String, public end_date: String) {}
}

export default Vue.extend({
  data (): {mindate: String, maxdate: String, reports: Report[]} {
    let mindate = '2017-07-01'
    let maxdate = formatDate(new Date())
    let reports: Report[] = []

    return { mindate, maxdate, reports }
  },

  methods: {
    numReports (): number {
      return this.reports.length
    },

    updateReport (i: number) {
      let report = this.reports[i]
      let id = report.id
      axios.put('reports/' + id, JSON.stringify(report), useJsonHeader).then((response: any) => {
        console.log('Updated report #' + id + ':', JSON.stringify(report))
      })
    },

    newReport () {
      axios.get('reports/new').then((response: any) => {
        let template = response.data
        console.log('Creating new report:', template)
        this.reports.push(template)
      })
    },

    downloadPdf (index: number) {
      let report = this.reports[index]
      let link = 'http://localhost:8000/api/reports/' + report.id + '/pdf/' + report.title + '.pdf'
      window.open(link, '_blank')
    }
  },

  beforeMount () {
    axios.get('reports').then((response: any) => {
      response.data.forEach((element: any) => {
        element.in_db = true
        this.reports.push(element)
      })
    })
  }
})
</script>

<style>
body {
  text-align: center;
}

input {
  background: none;
  border: 0;
}

input.title {
  min-width: 20em;
}

button {
  border-radius: 0;
  color: white;
  background: #369;
  padding: 0.5em;
  margin: 1em;
}

.edit {
  text-decoration: none;
  white-space: nowrap;
  color: #369;
}
</style>
