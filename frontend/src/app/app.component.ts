import { Component } from '@angular/core';

enum Week {
  A, B, C, D
}

export class Entry {
  name: string;
  date: string;
  week: Week;
  start: string;
  end: string;
  remark: string;

  constructor(name = '', date = '', week = Week.A, start = '', end = '', remark = '') {
    this.name = name;
    this.date = date;
    this.week = week;
    this.start = start;
    this.end = end;
    this.remark = remark;
  }

  next() {
      return new Entry('', this.date, this.week, '', '', '');
  }
}

const ENTRIES: Entry[] = [
  new Entry('Alice A', '2017-08-24', Week.B, '14:45', '15:30', 'Vertretung für Bob'),
  new Entry('Baz', '2017-08-25', Week.A, '13:55', '15:00', ''),
  new Entry('Otto', '2017-08-30', Week.B, '14:05', '15:20', '')
];

export class Globals {
  mindate: string;
  maxdate: string;
  mintime: string;
  maxtime: string;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent {
  title = 'Abrechnung BetreuerInnen';
  entries = ENTRIES;
  timePeriod = '';
  globals: Globals = {
    mindate: '2017-08-01',
    maxdate: '2017-12-31',
    mintime: '12:30',
    maxtime: '16:00',
  };
}
