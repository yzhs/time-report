<template>
  <div id="app" style="text-align:center">
    <h1>
      Abrechnung BetreuerInnen
    </h1>
    <input type="text" name="heading" id="heading" placeholder="Zeitraum"
           required minlength="8" maxlength="100" pattern="[^a-z][a-zA-Zäöuß0-9. ]+"
           title="Bitte nur Buchstaben, Zahlen, Leerzeichen und Punkte verwenden"
           v-model="globals.title" v-on:keyup="updateTitle" v-on:change="updateTitle"/>

    <table id="table">
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
      <tr v-for="item in items" :key="item.id">
        <td>
          <input type="text" name="name" placeholder="Vorname Nachname"
                 list="employees" spellcheck="false"
                 minlength="2" maxlength="100"
                 pattern=".*[^. ,-]+.*" required
                 v-model="item.name"/>
        </td>
        <td>
          <input type="date" name="day" placeholder="Datum" required
                 :min="globals.mindate" :max="globals.maxdate"
                 v-model="item.day" />
        </td>
        <td>
          <select name="week" v-model="item.type_of_week" tabindex="-1">
            <option value="0">A</option>
            <option value="1">B</option>
            <option value="2">C</option>
            <option value="3">D</option>
          </select>
        </td>
        <td>
          <input type="time" name="start" placeholder="von" step="300"
                 :min="globals.mintime" :max="globals.maxtime" required
                 v-model="item.start"/>
          </td>
          <td>
          <input type="time" name="end" placeholder="bis" step="300"
                 :min="globals.mintime" :max="globals.maxtime" required
                 v-model="item.end"/>
        </td>
        <td>
          <input type="text" name="remark" placeholder="Bemerkung"
                 v-model="item.remark"/>
        </td>
        <td>
          {{item.id}}
          <button class="add-item" v-on:click="addItem">neue Zeile</button>
        </td>
      </tr>
    </table>

    <button id="generate" name="generate">PDF erzeugen</button>

    <datalist id="employees">
      <option v-for="employee in employees" :key="employee">{{employee}}</option>
    </datalist>
  </div>
</template>

<script>
let formatDate = function (date) {
  return date.toISOString().split('T')[0]
}
export default {
  data () {
    let now = new Date()
    let maxdate = formatDate(now)
    let halfAYearAgo = new Date()
    halfAYearAgo.setMonth(now.getMonth() - 12)
    let mindate = formatDate(halfAYearAgo)

    return {
      globals: {mindate: mindate, maxdate: maxdate, mintime: '12:30', maxtime: '16:00'},
      numItems: 0,
      employees: [],
      items: []
    }
  },
  methods: {
    updateTitle: function (e) {
      document.title = 'Abrechung BetreuerInnen ' + e.target.value
    },
    addItem: function () {
      this.$http.get('new_item').then(response => {
        let obj = response.body
        obj.start = obj.start.substr(0, 5)
        obj.end = obj.end.substr(0, 5)
        this.items.push(obj)
      })
      this.numItems++
    }
  },
  beforeMount () {
    this.$http.get('items').then(response => {
      response.body.map(element => {
        element.start = element.start.substr(0, 5)
        element.end = element.end.substr(0, 5)
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
</style>
