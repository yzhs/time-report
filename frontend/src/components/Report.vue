<template>
  <div id="app" style="text-align:center">
    <h1>Abrechnung BetreuerInnen</h1>
    <input type="text" name="heading" id="heading" placeholder="Zeitraum"
           required minlength="8" maxlength="100" pattern="[^a-z][a-zA-Zäöuß0-9. ]+"
           title="Bitte nur Buchstaben, Zahlen, Leerzeichen und Punkte verwenden"
           v-model="report.title" v-on:keyup="updateTitle" v-on:change="titleChanged"/>

    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Datum</th>
          <th>Woche</th>
          <th>von</th>
          <th>bis</th>
          <th>Bemerkung</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item, index) in items" :key="index">
          <td>
            <input type="text" name="name" placeholder="Vorname Nachname"
                  list="employees" spellcheck="false"
                  minlength="2" maxlength="100"
                  pattern=".*[^. ,-]+.*" required
                  v-model="item.name" v-on:change="updateItem(index)"/>
          </td>
          <td>
            <button class="plus-minus minus" v-on:mousedown="previousDate(index)" tabindex="-1">–</button>
            <input type="date" name="day" placeholder="Datum" required
                  :min="report.mindate" :max="report.maxdate"
                  v-model="item.day"  v-on:change="updateItem(index)"/>
            <button class="plus-minus plus" v-on:mousedown="nextDate(index)" tabindex="-1">+</button>
          </td>
          <td>
            <select name="week" v-model.number="item.type_of_week" tabindex="-1"
                    v-on:change="updateItem(index)">
              <option value="0">A</option>
              <option value="1">B</option>
              <option value="2">C</option>
              <option value="3">D</option>
            </select>
          </td>
          <td>
            <input type="time" name="start" placeholder="von" step="300"
                  :min="mintime" :max="maxtime" required
                  v-model="item.start" v-on:change="updateItem(index)"/>
            </td>
            <td>
            <input type="time" name="end" placeholder="bis" step="300"
                  :min="mintime" :max="maxtime" required
                  v-model="item.end" v-on:change="updateItem(index)"/>
          </td>
          <td>
            <input type="text" name="remark" placeholder="Bemerkung"
                  v-model="item.remark" v-on:change="updateItem(index)"/>
          </td>
        </tr>
      </tbody>
    </table>

    <button id="add-item" v-on:click="newItem">neue Zeile</button>
    <button id="generate" name="generate" v-on:click="generatePdf">PDF erzeugen</button>

    <datalist id="employees">
      <option v-for="employee in employees" :key="employee">{{employee}}</option>
    </datalist>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { Route } from "vue-router"
import axios, { AxiosResponse } from 'axios'
import { Item, Report, formatDate, useJsonHeader } from '../util'

interface ReportData {
  id: number,
  mintime: string
  maxtime: string
  report: any
  employees: string[]
  items: Item[]
}

export default Vue.extend({
  data (): ReportData {
    let id: number = Number(this.$route.params.id)
    let maxdate: string = formatDate(new Date())

    return {
      id,
      mintime: '12:30',
      maxtime: '16:00',
      report: {id, title: '', mindate: '2017-08-01', maxdate},
      employees: [],
      items: []
    }
  },

  methods: {
    isComplete (i: number): boolean {
      let item: Item = this.items[i - 1]
      return item.name !== ''
    },

    setTitle (title: string) {
      document.title = 'Abrechung BetreuerInnen ' + title
    },

    updateTitle (e: Event) {
      this.setTitle((e.target as any).value)
    },

    titleChanged (e: Event) {
      this.updateTitle(e)
      axios.put('reports/' + this.report.id, this.report)
      .catch((reason: any) => {
        console.error('Error creating updating title:', reason.response.data.message)
      })
    },

    newItem () {
      axios.get('reports/' + this.report.id + '/items/template')
          .then((response: any) => {
        let obj = response.data
        obj.start = obj.start.substr(0, 5)
        obj.end = obj.end.substr(0, 5)
        obj.inDb = false
        this.items.push(obj)
      }).catch((reason: any) => {
        console.error('Error creating new item:', reason.response.data.message)
      })
    },

    updateItem (index: number) {
      let item = this.items[index]
      let updateItem = item as any
      if (!item.inDb) {
        updateItem.id = 0
      }
      updateItem.start_time = item.start
      updateItem.end_time = item.end
      axios.put('reports/' + this.report.id + '/items/' + updateItem.id, JSON.stringify(updateItem), useJsonHeader)
          .then((response: any) => {
        if (!item.inDb) {
          item.inDb = true
          item.id = response.data
        }
      }).catch((reason: any) => {
        console.error('Error creating updating item:', reason.response.data.message)
      })
    },

    nextDate (i: number) {
      axios.get('next_schoolday/' + this.items[i].day)
           .then((response: AxiosResponse<string>) => {
             this.items[i].day = response.data
           })
    },

    previousDate (i: number) {
      axios.get('previous_schoolday/' + this.items[i].day)
           .then((response: AxiosResponse<string>) => {
             this.items[i].day = response.data
           })
    },

    generatePdf () {
      let link = 'http://localhost:8000/api/reports/' + this.report.id + '/pdf/' + this.report.title + '.pdf'
      window.open(link, '_blank')
    }
  },

  beforeMount () {
    axios.get('reports/' + this.report.id).then((response: any) => {
      this.report = response.data
      this.setTitle(this.report.title)
    }).catch((reason: any) => {
      console.error('Error getting current report:', reason.response.data.message)
    })
    axios.get('reports/' + this.report.id + '/items').then((response: any) => {
      response.data.map((element: Item) => {
        // Strip trailing ':00' from time strings
        element.start = element.start.substr(0, 5)
        element.end = element.end.substr(0, 5)
        element.inDb = true
        this.items.push(element)
      })
      if (this.items.length === 0) {
        this.newItem()
      }
    }).catch((reason: any) => {
      console.error('Error getting all items:', reason.response.data.message)
    })

    axios.get('employees').then((response: any) => {
      // TODO update employees later?
      this.employees = response.data
    }).catch((reason: any) => {
      console.error('Error list of employees:', reason.response.data.message)
    })
  }
})
</script>

<style scoped>
#heading {
  width: 23em;
}

button {
  font-size: 150%;
  color: white;
  background-color: #369;
  margin: 1em;
  padding: 0.3em;
  border-radius: 0;
}

button.plus-minus {
  width: 1.5em;
  font-size: 100%;
  margin: 0;
  color: #369;
  background-color: rgba(0, 0, 0, 0.1);
  font-weight: bolder;
}

button.minus {
  border-radius: 5px 0 0 5px
}

button.plus {
  border-radius: 0 5px 5px 0
}

td:nth-child(1) {
  min-width: 20ex;
  text-align: left;
}

td:nth-child(2) {
  min-width: 11em;
}

</style>
