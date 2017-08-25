import { Component } from '@angular/core';

enum Week {
  A, B, C, D
}

export class Row {
  name: string;
  date: string;
  week: Week;
  from: string;
  to: string;
  remark: string;
}

const ROWS = [
  { name: 'Foo Bar', date: '2017-08-24', week: Week.A, from: '', to: '', remark: '' },
  { name: 'Baz', date: '2017-08-25', week: Week.A, from: '', to: '', remark: '' },
  { name: 'Otto', date: '2017-08-30', week: Week.B, from: '', to: '', remark: '' },
  { name: '', date: '', week: Week.B, from: '', to: '', remark: '' },
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
