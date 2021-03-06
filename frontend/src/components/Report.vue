<template>
  <div style="text-align:center">
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
          <th>gespeichert</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item, index) in items" :key="index">
          <td v-bind:class="{ unmodified: !item.isModified('name') }">
            <input type="text" name="name" placeholder="Vorname Nachname"
                  list="employees" spellcheck="false"
                  minlength="2" maxlength="100"
                  pattern=".*[^. ,-]+.*" required
                  v-model="item.name" v-on:change="updateItem(index, 'name')"/>
          </td>
          <td v-bind:class="{ unmodified: !item.isModified('date') }">
            <button class="plus-minus minus" v-on:mousedown="previousDate(index)" tabindex="-1">–</button>
            <input type="date" name="day" placeholder="Datum" required
                  :min="report.mindate" :max="report.maxdate"
                  v-model="item.day"  v-on:change="updateItem(index, 'date')"
                  v-on:blur="item.modify('date')"/>
            <button class="plus-minus plus" v-on:mousedown="nextDate(index)" tabindex="-1">+</button>
          </td>
          <td v-bind:class="{ unmodified: !item.isModified('date') }">
            <select name="week" v-model.number="item.type_of_week" tabindex="-1"
                    v-on:change="updateItem(index)">
              <option value="0">A</option>
              <option value="1">B</option>
              <option value="2">C</option>
              <option value="3">D</option>
            </select>
          </td>
          <td v-bind:class="{ unmodified: !item.isModified('start') }">
            <input type="time" name="start" placeholder="von" step="300"
                  :min="mintime" :max="maxtime" required
                  v-model="item.start" v-on:change="updateItem(index, 'start')"
                  v-on:blur="item.modify('start')"/>
          </td>
          <td v-bind:class="{ unmodified: !item.isModified('end') }">
            <input type="time" name="end" placeholder="bis" step="300"
                  :min="mintime" :max="maxtime" required
                  v-model="item.end" v-on:change="updateItem(index, 'end')"
                  v-on:blur="item.modify('end')"/>
          </td>
          <td>
            <input type="text" name="remark" placeholder="Bemerkung"
                  v-model="item.remark" v-on:change="updateItem(index)"/>
          </td>
          <td>
            <input type="checkbox" onclick="return false;" v-model="item.inDb" tabindex="-1">
          </td>
        </tr>
      </tbody>
    </table>

    <button id="add-item" v-on:click="newItem">neue Zeile</button>
    <button id="generate" name="generate" v-on:click="generatePdf">PDF erzeugen</button>

    <datalist id="employees">
      <option v-for="employee in employees" :key="employee.id">{{employee.name}}</option>
    </datalist>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { Route } from "vue-router"
import axios, { AxiosResponse } from 'axios'
import { Item, Report, formatDate, newItem, useJsonHeader } from '../util'

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
      // TODO get data from backend
      mintime: '12:00',
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
      // The previous record seems to have been finished, so we can mark it as done.
      if (this.items.length !== 0) {
        this.items[this.items.length - 1].allModified()
      }

      axios.get('reports/' + this.report.id + '/items/template')
          .then((response: any) => {
        let item = newItem(response.data)
        this.items.push(item)
      }).catch((reason: any) => {
        console.error('Error creating new item:', reason.response.data.message)
      })
    },

    updateItem (index: number, field?: string) {
      let item = this.items[index]

      if (field !== null) {
        item.modify(field!)
      }

      let updateItem = item as any
      if (!item.inDb) {
        updateItem.id = 0
      }
      updateItem.start_time = item.start
      updateItem.end_time = item.end
      axios.put('reports/' + this.report.id + '/items/' + updateItem.id, JSON.stringify(updateItem), useJsonHeader)
          .then((response: any) => {
        let newItem = response.data
        if (!item.inDb) {
          item.inDb = true
          item.id = newItem.id
        }
        item.type_of_week = newItem.type_of_week
      }).catch((reason: any) => {
        console.error('Error creating updating item:', reason.response.data.message)
      })
    },

    nextDate (i: number) {
      axios.get('next_schoolday/' + this.items[i].day)
           .then((response: AxiosResponse<string>) => {
              this.items[i].day = response.data
              this.updateItem(i, 'date')
           })
    },

    previousDate (i: number) {
      axios.get('previous_schoolday/' + this.items[i].day)
           .then((response: AxiosResponse<string>) => {
              this.items[i].day = response.data
              this.updateItem(i, 'date')
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
        let item = newItem(element)
        item.inDb = true
        item.allModified()
        this.items.push(item)
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

.unmodified > * {
  background: lightblue;
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
