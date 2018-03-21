<template>
  <div id="app" style="text-align:center">
    <h1>
      Abrechnung BetreuerInnen
    </h1>
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
        <tr v-for="(item, index) in items" :key="item.id">
          <td>
            <input type="text" name="name" placeholder="Vorname Nachname"
                  list="employees" spellcheck="false"
                  minlength="2" maxlength="100"
                  pattern=".*[^. ,-]+.*" required
                  v-model="item.name" v-on:change="onItemChange(index)"/>
          </td>
          <td>
            <input type="date" name="day" placeholder="Datum" required
                  :min="report.mindate" :max="report.maxdate"
                  v-model="item.day"  v-on:change="onItemChange(index)"/>
          </td>
          <td>
            <select name="week" v-model.number="item.type_of_week" tabindex="-1"
                    v-on:change="onItemChange(index)">
              <option value="0">A</option>
              <option value="1">B</option>
              <option value="2">C</option>
              <option value="3">D</option>
            </select>
          </td>
          <td>
            <input type="time" name="start" placeholder="von" step="300"
                  :min="mintime" :max="maxtime" required
                  v-model="item.start" v-on:change="onItemChange(index)"/>
            </td>
            <td>
            <input type="time" name="end" placeholder="bis" step="300"
                  :min="mintime" :max="maxtime" required
                  v-model="item.end" v-on:change="onItemChange(index)"/>
          </td>
          <td>
            <input type="text" name="remark" placeholder="Bemerkung"
                  v-model="item.remark" v-on:change="onItemChange(index)"/>
          </td>
        </tr>
      </tbody>
    </table>

    <button id="add-item" v-on:click="addItem">neue Zeile</button>
    <button id="generate" name="generate" v-on:click="generatePdf">PDF erzeugen</button>

    <datalist id="employees">
      <option v-for="employee in employees" :key="employee">{{employee}}</option>
    </datalist>
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

export class Item {
  in_db: boolean = false
  constructor(public id: number, public name: string, public day: string,
              public type_of_week: number, public start: string,
              public end: string, public remark: string) {}
}

export default Vue.extend({
  data () {
    let id: number = 1
    let maxdate: String = formatDate(new Date())

    return {
      id,
      mintime: '12:30' as String,
      maxtime: '16:00' as String,
      report: {id, title: '', mindate: '2017-08-01', maxdate},
      numItems: 0 as number,
      employees: [] as String[],
      items: [] as Item[]
    }
  },

  methods: {
    isComplete (i: number): boolean {
      let item: Item = this.items[i - 1]
      return item.name !== ''
    },

    updateTitle (e: Event) {
      document.title = 'Abrechung BetreuerInnen ' + (e.target as any).value
    },

    titleChanged (e: Event) {
      this.updateTitle(e)
      axios.put('reports/' + this.report.id, JSON.stringify(this.report), useJsonHeader)
          .then((response: any) => {
        console.log('Updated report #' + this.report.id + ':', this.report)
      })
    },

    addItem () {
      axios.get('reports/' + this.report.id + '/items/template')
          .then((response: any) => {
        let obj = response.body
        obj.start = obj.start.substr(0, 5)
        obj.end = obj.end.substr(0, 5)
        obj.in_db = false
        this.items.push(obj)
      })
      this.numItems++
    },

    onItemChange (index: number) {
      let item = this.items[index]
      console.log('Changed item #' + item.id + ':', item.name, item.day, item.type_of_week, item.start, item.end, item.remark)
      let updateItem = item as any
      if (!item.in_db) {
        updateItem.id = 0
      }
      updateItem.start_time = item.start
      updateItem.end_time = item.end
      axios.put('reports/' + this.report.id + '/items/' + updateItem.id, JSON.stringify(updateItem), useJsonHeader)
          .then((response: any) => {
        console.log('Done', item.in_db ? 'updating' : 'inserting')
        if (!item.in_db) {
          item.in_db = true
          item.id = response.body
        }
      })
    },

    generatePdf () {
      let link = 'http://localhost:8000/api/reports/' + this.report.id + '/pdf'
      window.open(link, '_blank')
    }
  },

  beforeMount () {
    axios.get('reports/' + this.report.id).then((response: any) => {
      this.report = response.body
    })
    axios.get('reports/' + this.report.id + '/items').then((response: any) => {
      response.body.map((element: Item) => {
        element.start = element.start.substr(0, 5)
        element.end = element.end.substr(0, 5)
        element.in_db = true
        this.items.push(element)
        this.numItems++
      })
      if (this.numItems === 0) {
        this.addItem()
      }
    })

    axios.get('employees').then((response: any) => {
      this.employees = response.body
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
  color: white;
  margin: 1em;
  padding: 0.3em;
  border-radius: 0;
}

td:nth-child(1) {
  min-width: 20ex;
  text-align: left;
}
</style>
