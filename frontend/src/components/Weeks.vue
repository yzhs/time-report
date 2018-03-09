<template>
    <div id="app" style="text-align:center">
        <h1>A-D Wochen</h1>

        <h2><input id="year" name="year" v-model="year"/></h2>

        <div class="wrapper">
          <div v-for="month in months" :key="month">
            <month :year="year" :month="month"></month>
          </div>
        </div>
    </div>
</template>

<script>
export default {
  data () {
    let year = 2017
    return {
      year: year,
      months: [...Array(12)].map((_, i) => i + 1)
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
