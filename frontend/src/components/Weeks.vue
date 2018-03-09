<template>
    <div id="app" style="text-align:center">
        <h1>A-D Wochen</h1>

        <h2><input id="year" name="year" v-model="year"/></h2>

        <div class="wrapper">
          <div v-for="month in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]" :key="month">
            <month :year="year" :month="month"></month>
          </div>
        </div>
    </div>
</template>

<script>
// taken from https://stackoverflow.com/questions/6117814/get-week-of-year-in-javascript-like-in-php
function getWeekNumber (day) {
  var d = new Date(Date.UTC(day.getFullYear(), day.getMonth(), day.getDate()))
  var dayNum = d.getUTCDay() || 7
  d.setUTCDate(d.getUTCDate() + 4 - dayNum)
  var yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1))
  return Math.ceil((((d - yearStart) / 86400000) + 1) / 7)
}

export default {
  data () {
    let days = []
    let year = 2017
    let day = new Date(year, 0, 1)
    while (day.getFullYear() === year) {
      days.push(new Date(day.getFullYear(), day.getMonth(), day.getDate()))
      day.setDate(day.getDate() + 1)
    }

    let weeks = []
    for (var i = 0; i < 54; i++) {
      weeks.push({num: i, typ: i % 4, days: []})
    }

    days.forEach(day => {
      let i = getWeekNumber(day)
      if (i > 50 && day.getMonth() === 0) {
        weeks[0].days.push(day)
      } else {
        weeks[i].days.push(day)
      }
    })

    return {
      time: new Date(),
      year: year,
      days: days,
      weeks: weeks
    }
  },
  computed: {
    date () {
      return [this.year, this.month, this.day].join('-')
    },
    month () {
      return this.time.getMonth() + 1
    },
    day () {
      return this.time.getDate()
    }
  }
}
</script>

<style scoped>
  h2, input#year {
    color: #369;
    font-weight: 600;
    font-family: sans-serif;
    font-size: 140%;
    text-align: center;
    background-color: inherit;
    border: none;
    border-radius: 5px;
  }

  .wrapper {
    display: inline-block;
    margin: 0 auto;
    max-width: 69em;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    grid-gap: 2em;
  }

  .wrapper > div {
    display: inline-block;
  }
</style>
