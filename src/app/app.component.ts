import { Component } from '@angular/core';

enum Week {
  A, B, C, D
}

export class Row {
  name: string;
  date: string;
  week: Week;
  start: string;
  end: string;
  remark: string;
}

const ROWS = [
  { name: 'Foo Bar', date: '2017-08-24', week: Week.A, start: '12:00', end: '15:30', remark: 'Vertretung für Otto' },
  { name: 'Baz', date: '2017-08-25', week: Week.A, start: '13:55', end: '15:00', remark: '' },
  { name: 'Otto', date: '2017-08-30', week: Week.B, start: '14:05', end: '15:20', remark: '' },
  { name: '', date: '', week: Week.B, start: '', end: '', remark: '' },
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
  rows: Row[] = ROWS;
  globals: Globals = {
    mindate: '2017-08-01',
    maxdate: '2017-12-31',
    mintime: '12:30',
    maxtime: '15:45',
  };
}
