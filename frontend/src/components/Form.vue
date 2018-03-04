<template>
  <div id="app" style="text-align:center">
    <h1>
      Abrechnung BetreuerInnen
    </h1>
    <input type="text" name="heading" id="heading" placeholder="Zeitraum"
          v-model="globals.title" required v-on:keyup="updateTitle" v-on:change="updateTitle"/>

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
                 minlength="2" maxlength="100"
                 pattern=".*[^. ,-]+.*" required
                 v-model="item.name"/>
        </td>
        <td>
          <input type="date" name="day" placeholder="Datum" required
                 :min="globals.mindate" :max="globals.maxdate"
                 v-model="item.date" />
        </td>
        <td>
          <select name="week" v-model="item.week" tabindex="-1">
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
          <button class="add-item" name="submit" v-on:click="addItem">neue Zeile</button>
        </td>
      </tr>
    </table>

    <button id="generate" name="generate">PDF erzeugen</button>
  </div>
</template>

<script>
export default {
  data () {
    let now = new Date()
    let maxdate = now.toISOString().split('T')[0]
    let halfAYearAgo = new Date()
    halfAYearAgo.setMonth(now.getMonth() - 6)
    let mindate = halfAYearAgo.toISOString().split('T')[0]

    return {
      globals: {mindate: mindate, maxdate: maxdate},
      numItems: 1,
      items: [
        {id: 0, name: 'Alice A', date: '2018-03-01', week: '1', start: '13:00', end: '15:30'}
      ]
    }
  },
  methods: {
    updateTitle: function (e) {
      document.title = 'Abrechung BetreuerInnen ' + e.target.value
    },
    addItem: function () {
      this.items.push({id: this.numItems, name: ''})
      this.numItems++
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h1, input#heading {
  color: #369;
  font-weight: 600;
  font-family: sans-serif;
  font-size: 250%;
  text-align: center;
}

input#heading {
  margin-top: -0.7em;
  padding-bottom: 0.5em;
}

#table {
  border-collapse: collapse;
  background-color: #eee;
  max-width: 800px;
  margin: 0 auto;
  box-shadow: 0 0 10px #aaa;
}

td {
  border: 0;
  padding: 0;
}

td:nth-child(1) {
  min-width: 20ex;
  text-align: left;
}

th {
  padding: 1ex 0ex;
  background-color: #eee;
}

tr:nth-child(odd) {
  background-color: #fff;
}

input[type="date"],
input[type="text"],
input[type="time"] {
  font-size: inherit;
  background-color: inherit;
  height: 3ex;
  padding: 1ex 2ex;
  border: none;
  border-radius: 5px;
}

input:focus {
  box-shadow: 0 0 10px #369;
}

select {
  font-size: inherit;
  border: none;
  border-radius: 5px;
  background: transparent;
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  padding: 1ex 2ex;
  height: 5.2ex;
  /* TODO Why can't we use 3ex here? */
}
select:focus {
  box-shadow: 0 0 10px #369;
}

input#heading:focus {
  box-shadow: none;
}

button {
  background-color: yellow;
  border: none;
  border-radius: 5px;
  margin-right: 10px;
}

button.add-item {
  background-color: darkblue;
  color: white;
  visibility: hidden;
}

button#generate {
  background-color: #369;
  color: white;
  font-size: 150%;
  margin: 1em;
  padding: 0.3em;
}
tr:last-of-type button.add-item {
  visibility: visible;
}
</style>
