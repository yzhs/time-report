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
                  :min="globals.mintime" :max="globals.maxtime" required
                  v-model="item.start" v-on:change="onItemChange(index)"/>
            </td>
            <td>
            <input type="time" name="end" placeholder="bis" step="300"
                  :min="globals.mintime" :max="globals.maxtime" required
                  v-model="item.end" v-on:change="onItemChange(index)"/>
          </td>
          <td>
            <input type="text" name="remark" placeholder="Bemerkung"
                  v-model="item.remark" v-on:change="onItemChange(index)"/>
          </td>
          <td>
            <span style="height:2em;" class="id">{{item.id}}</span>
          </td>
        </tr>
      </tbody>
    </table>

    <button id="add-item" v-on:click="addItem">neue Zeile</button>
    <button id="generate" name="generate">PDF erzeugen</button>

    <datalist id="employees">
      <option v-for="employee in employees" :key="employee">{{employee}}</option>
    </datalist>
  </div>
</template>

<script>
function formatDate (date) {
  return date.toISOString().split('T')[0]
}
let useJsonHeader = {
  headers: {
    'Content-Type': 'application/json'
  }
}

export default {
  params: {
    id: 1
  },
  data () {
    let now = new Date()
    let maxdate = formatDate(now)

    return {
      globals: {mintime: '12:30', maxtime: '16:00'},
      report: {id: 1, title: '', mindate: '2017-08-01', maxdate: maxdate},
      numItems: 0,
      employees: [],
      items: []
    }
  },
  methods: {
    isComplete: function (id) {
      let item = this.items[id - 1]
      return item.name !== ''
    },
    updateTitle: function (e) {
      document.title = 'Abrechung BetreuerInnen ' + e.target.value
    },
    titleChanged: function (e) {
      this.id = this.report.id
      this.updateTitle(e)
      this.$http.put('reports/' + this.id, JSON.stringify(this.report), useJsonHeader).then(response => {
        console.log('Updated report #' + this.id + ':', this.report)
      })
    },
    addItem: function () {
      this.$http.get('items/template').then(response => {
        let obj = response.body
        obj.start = obj.start.substr(0, 5)
        obj.end = obj.end.substr(0, 5)
        obj.in_db = false
        this.items.push(obj)
      })
      this.numItems++
    },
    onItemChange: function (index) {
      let item = this.items[index]
      console.log('Changed item #' + item.id + ':', item.name, item.day, item.type_of_week, item.start, item.end, item.remark)
      let updateItem = Object.assign({}, item)
      if (!item.in_db) {
        updateItem.id = 0
      }
      this.$http.put('items/' + updateItem.id, JSON.stringify(updateItem), {
        headers: {
          'Content-Type': 'application/json'
        }
      }).then(response => {
        console.log('Done', item.in_db ? 'updating' : 'inserting')
        if (!item.in_db) {
          item.in_db = true
          item.id = response.body
        }
      })
    }
  },
  beforeMount () {
    this.$http.get('reports/' + this.report.id).then(response => {
      this.report = response.body
    })
    this.$http.get('items').then(response => {
      response.body.map(element => {
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

    this.$http.get('employees').then(response => {
      this.employees = response.body
    })
  }
}
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
}

td:nth-child(1) {
  min-width: 20ex;
  text-align: left;
}
</style>
