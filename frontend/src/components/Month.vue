<template>
  <div id="vcom-calendar" class="vcom-calendar">
    <div class="header">
      <div class="head">{{monthNames[month - 1]}}</div>
      <div class="weekdays">
        <span v-for="weekday in weekdays" class="week" :key="weekday">{{weekday}}</span>
      </div>
    </div>
    <div class="days">
      <span class="day" v-for="day in days" track-by="$index" :key="day.year + '-' + day.month + '-' + day.day">
          <span v-if="day.isThisMonthDay" class="this-month-day">{{day.day}}</span>
          <span v-else></span>
      </span>
    </div>
  </div>
</template>

<style scoped>
  .vcom-calendar {
    width: 350px;
    height: auto;
    overflow: hidden;
    -webkit-box-shadow: 0 5px 10px 0 rgba(0, 0, 0, 0.2);
    box-shadow: 0 5px 10px 0 rgba(0, 0, 0, 0.2);
  }
  .header {
    width: 100%;
    padding: 10px 0;
    background-color: #369;
  }
  .head {
    text-align: center;
    font-size: 24px;
    padding: 10px 0;
    color: #ffffff;
    letter-spacing: 1px;
    text-shadow: 1px 1px 1px rgba(0, 0, 0, .1);
  }
  .weekdays {
    display: grid;
    width: 100%;
    padding: 10px 0;
    grid-template-columns: repeat(7, 1fr);
    text-align: center;
  }
  .weekdays .week {
    width: 50px;
    display: block;
    color: #ffffff;
    font-size: 16px;
    -webkit-box-sizing: border-box;
    -moz-box-sizing: border-box;
    box-sizing: border-box;
  }
  .days {
    background-color: #ffffff;
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    grid-auto-rows: minmax(50px, auto);
  }
  .days > .day {
    width: 50px;
    height: 50px;
    font-size: 12px;
    text-align: center;
    line-height: 50px;
    color: #333333;
    background-color: #fefefe;
    font-weight: bold;
    -webkit-box-sizing: border-box;
    -moz-box-sizing: border-box;
    box-sizing: border-box;
    border-right: 1px solid #f0f0f0;
    border-bottom: 1px solid #f0f0f0;
  }
  .days > .day > .this-month-day:hover {
    background-color: #e1e1e1;
    cursor: pointer;
    color: #ffffff;
  }
</style>

<script>
const MATRIX_MAX = 7 * 6
const WEEK_DAYS = ['Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So']
const MONTHS = ['Januar', 'Februar', 'März', 'April', 'Mai', 'Juni', 'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember']
let bMonthRe = /^1?$|3|5|7|8|10|12/

export default {
  props: [ 'year', 'month' ],
  data () {
    return {
      monthNames: MONTHS,
      time: new Date(),
      weekdays: WEEK_DAYS
    }
  },

  computed: {
    isLeapYear () {
      return ((this.year % 400 === 0) || ((this.year % 4 === 0) && (this.year % 100 !== 0)))
    },
    firstDayWeek () {
      return new Date(this.year, this.month - 1, 1).getDay()
    },
    days () {
      return this.getDaysList()
    }
  },

  methods: {
    _getPreMonthDays (month, offset) {
      if (offset === 0) {
        return []
      } else {
        return this.getDays(1).slice(-offset)
      }
    },
    _getNextMonthDays (month, offset) {
      if (offset % 7 === 6) {
        return []
      }
      return this.getDays(1).slice(0, offset % 7)
    },
    _getRangeList (range, start) {
      var i = start || 1
      var _list = []
      for (; i <= range; i++) {
        _list.push(i)
      }
      return _list
    },
    getDays (month) {
      if (bMonthRe.test(month)) {
        return this._getRangeList(31)
      } else if (month === 2) {
        return this._getRangeList(this.isLeapYear ? 29 : 28)
      } else {
        return this._getRangeList(30)
      }
    },
    getDaysList () {
      let _needConcatLength = this.getDays(this.month).length + this.firstDayWeek
      let _initList = this._getPreMonthDays(this.month, (this.firstDayWeek + 6) % 7)
        .map((preMonthday) => {
          return {
            'year': this.month === 1 ? this.year - 1 : this.year,
            'month': this.month === 1 ? 12 : this.month - 1,
            'day': preMonthday,
            'isPreMonth': true
          }
        })
      if (MATRIX_MAX <= _needConcatLength) {
        let _thisMonthDaysList = this.getDays(this.month).slice(0, this.getDays(this.month).length - (_needConcatLength - MATRIX_MAX))
        return _initList.concat(_thisMonthDaysList.map((day) => {
          let _dateObj = {
            'year': this.year,
            'month': this.month,
            'isThisMonthDay': true,
            'day': day
          }
          if (day === this.currentDay) {
            _dateObj.isToday = true
          }
          return _dateObj
        }))
      } else {
        return _initList.concat(this.getDays(this.month).map((day) => {
          let _dateObj = {
            'year': this.year,
            'month': this.month,
            'isThisMonthDay': true,
            'day': day
          }
          if (day === this.currentDay) {
            _dateObj.isToday = true
          }
          return _dateObj
        })).concat(
          this._getNextMonthDays(this.month, MATRIX_MAX - _needConcatLength).map((nextMonthDay) => {
            return {
              'year': this.year,
              'month': this.month + 1,
              'day': nextMonthDay,
              'isNextMonth': true
            }
          })
        )
      }
    }
  }
}
</script>
