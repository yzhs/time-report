import { Dictionary } from "vue-router/types/router";

export function formatDate (date: Date) {
  return date.toISOString().split('T')[0]
}

export const useJsonHeader = {
  headers: {
    'Content-Type': 'application/json'
  }
}

export class Item {
  public inDb: boolean = false
  public modified: Dictionary<boolean> = {
    'name': false,
    'date': false,
    'start': false,
    'end': false
  }
  constructor(public id: number, public name: string, public day: string,
              public type_of_week: number, public start: string,
              public end: string, public remark: string) {}

  noneModified () {
    ['name', 'date', 'start', 'end'].forEach(comp => this.modified[comp] = false)
  }

  allModified () {
    ['name', 'date', 'start', 'end'].forEach(comp => this.modified[comp] = true)
  }

  modify (component: string) {
    this.modified[component] = true
  }

  isModified (component: string): boolean {
    return this.modified[component]
  }
}

export function newItem ({id, name, day, type_of_week, start, end, remark}:
    {id: number, name: string, day: string, type_of_week: number, start: string, end: string, remark: string}): Item {
  // Strip trailing ':00' from time strings
  return new Item(id, name, day, type_of_week, start.substr(0, 5), end.substr(0, 5), remark)
}

export class Report {
  mindate: string = '2017-08-01'
  maxdate: string = formatDate(new Date())
  was_pdf_generated: boolean = false

  constructor(public id: number, public title: string, public start_date: string, public end_date: string) {}
}