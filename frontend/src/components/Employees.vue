<template>
  <div>
    <h1>Mitarbeiter</h1>

    <ul>
      <li v-for="(employee, index) in employees" :key="index">
        <input type="text" required minlength="2" title="Name" placeholder="Vorname Nachname"
               v-model="employee.name" @change="onChange(index)">
        <button class="delete" title="Löschen" @click="deleteEmployee(index)"></button>
      </li>
    </ul>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import axios,{ AxiosResponse } from 'axios'
import { useJsonHeader } from '../util'

export class Employee {
  id: number | null = 0
  inDb: boolean = false
  constructor (public name: string) {}
}

export default Vue.extend({
  data (): { employees: Employee[] } {
    return { employees: [] }
  },

  methods: {
    onChange (index: number) {
      if (index + 1 === this.employees.length) {
        // Last employee; add a new row when done with this one
        this.employees.push(new Employee(''))
      }

      let employee = this.employees[index]
      if (employee.inDb) {
        axios.put('employees/' + employee.id!, employee, useJsonHeader).then((response: AxiosResponse<any>) => {
          console.log('Updated employee #' + employee.id! + '; response:', response)
        }).catch((reason: any) => {
          console.error('Error updating employee record:', reason.response)
        })
      } else {
        axios.post('employees', employee, useJsonHeader).then((response: AxiosResponse<any>) => {
          console.log('Added employee', JSON.stringify(employee), 'to database. Response:', response)
          employee.id = response.data
          console.log('The id is', employee.id!)
        }).catch((reason: any) => {
          console.error('Error adding employee record:', reason.response)
        })
      }
    },

    deleteEmployee (index: number) {
      let employee = this.employees[index]
      if (employee.inDb) {
        axios.delete('employees/' + employee.id!, useJsonHeader).then((response: AxiosResponse<any>) => {
          console.log('Deleted employee from database')
        })
      }

      this.employees.splice(index)
    }
  },

  beforeMount () {
    axios.get('employees').then((response: any) => {
      response.data.forEach((element: any) => {
        element.inDb = true
        this.employees.push(element)
      })
      this.employees.push(new Employee(''))
    }).catch((reason: any) => {
      console.error('Error getting all reports:', reason.response.data.message)
    })
  }
})
</script>

<style>
body {
  text-align: center;
}

input {
  background-color: lightgray;
  border: 0;
}

@font-face {
  font-family: FontAwesome;
  src: url("../assets/fa-regular-400.woff2");
}

.delete {
  font-family: FontAwesome;
  text-decoration: none;
  white-space: nowrap;
  background: white;
  color: #369;
  font-size: 150%;
}
</style>

