<template>
  <div id="app">
      <h1>Abrechnungen</h1>

      <table>
        <thead>
          <th style="min-width: 20em;">Titel</th>
          <th style="min-width: 10em;">von</th>
          <th style="min-width: 10em;">bis</th>
          <th style="min-width: 5em;">Bearbeiten</th>
          <th style="min-width: 5em;" colspan="2">PDF</th>
        </thead>
        <tbody>
          <tr class="report" v-for="(report, index) in reports" :key="index">
            <td>
              <input type="text" class="title" name="title" v-model="report.title"
                     required minlength="8" maxlength="100" pattern="[^a-z][a-zA-Zäöuß0-9. ]+"
                     title="Bitte nur Buchstaben, Zahlen, Leerzeichen und Punkte verwenden"
                     v-on:change="updateReport(index)">
            </td>
            <td>
              <input type="date" name="start_date" v-model="report.start_date"
                     required :min="report.mindate" :max="report.end_date" v-on:change="updateReport(index)">
            </td>
            <td>
              <input type="date" name="end_date" v-model="report.end_date"
                     required :min="report.start_date" :max="report.maxdate" v-on:change="updateReport(index)">
            </td>
            <td>
              <router-link class="edit" style="padding: 0 1.5em;"
                           :to="{name: 'report', params: {id: report.id}}">
                
              </router-link>
            </td>
            <td>
              <input type="checkbox" onclick="return false;" v-model="report.was_pdf_generated">
            </td>
            <td>
              <a class="edit" href="#" v-on:click="downloadPdf(index)"></a>
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
  inDb: boolean = false
  was_pdf_generated: boolean = false
  mindate: String = '2017-08-01'
  maxdate: String = formatDate(new Date())

  constructor(public id: number, public title: String, public start_date: String, public end_date: String) {}
}

export default Vue.extend({
  data (): {mindate: String, maxdate: String, reports: Report[]} {
    let mindate = '2017-08-01'
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
      if (report.inDb) {
        let id = report.id
        axios.put('reports/' + id, JSON.stringify(report), useJsonHeader).then((response: any) => {
          console.log('Updated report #' + id)
        })
      } else {
        axios.post('reports', report, useJsonHeader).then((response: any) => {
          console.log('Created new report #' + response.data)
          report.id = response.data
          report.inDb = true
        })
      }

      this.updateNeighbors(i)
    },

    newReport () {
      axios.get('reports/new').then((response: any) => {
        let template: Report = response.data
        template.inDb = false

        console.log('Creating new report:', template)

        this.pushReport(template)
      })
    },

    /**
     * Adjust the `mindate` and `maxdate` of the following or previous report,
     * respectively.
     */
    updateNeighbors (i: number = -1) {
      if (i === -1) {
        i = this.reports.length - 1
      }
      let len = this.reports.length
      let report = this.reports[i]
      if (len == 1) {
        report.mindate = this.mindate
        report.maxdate = this.maxdate
        return
      }

      if (i > 0) {
        this.reports[i - 1].maxdate = report.start_date
        report.mindate = this.reports[i - 1].end_date
      }
      if (i < len - 1) {
        this.reports[i + 1].mindate = report.end_date
        report.maxdate = this.reports[i + 1].start_date
      }
    },

    /**
     * Add a new report to the list of reports. Set the `maxdate` property on
     * that report and adjust the maxdate on the previous report.
     */
    pushReport (report: Report) {
      report.maxdate = formatDate(new Date())

      this.reports.push(report)

      this.updateNeighbors()
    },

    downloadPdf (index: number) {
      let report = this.reports[index]
      report.was_pdf_generated = true
      let link = 'http://localhost:8000/api/reports/' + report.id + '/pdf/' + report.title + '.pdf'
      window.open(link, '_blank')
    }
  },

  beforeMount () {
    axios.get('reports').then((response: any) => {
      response.data.forEach((element: any) => {
        element.inDb = true
        this.pushReport(element)
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

@font-face {
  font-family: FontAwesome;
  src: url("../assets/fa-regular-400.woff2");
}

.edit {
  font-family: FontAwesome;
  text-decoration: none;
  white-space: nowrap;
  color: #369;
  font-size: 150%;
}
</style>
