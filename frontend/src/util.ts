export function formatDate (date: Date) {
  return date.toISOString().split('T')[0]
}

export const useJsonHeader = {
  headers: {
    'Content-Type': 'application/json'
  }
}

export class Item {
  inDb: boolean = false
  constructor(public id: number, public name: string, public day: string,
              public type_of_week: number, public start: string,
              public end: string, public remark: string) {}
}

export class Report {
  mindate: string = '2017-08-01'
  maxdate: string = formatDate(new Date())
  was_pdf_generated: boolean = false

  constructor(public id: number, public title: string, public start_date: string, public end_date: string) {}
}