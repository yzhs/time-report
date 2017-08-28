import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';

export class Entry {
  name: string;
  date: string;
  week: number;
  start: string;
  end: string;
  remark: string;

  constructor(name = '', date = '', week = 0, start = '', end = '', remark = '') {
    this.name = name;
    this.date = date;
    this.week = week;
    this.start = start;
    this.end = end;
    this.remark = remark;
  }
}

export class Globals {
  timePeriod: string;
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
export class AppComponent implements OnInit {
  title = 'Abrechnung BetreuerInnen';
  entries: Entry[];
  globals: Globals;

  constructor(private http: HttpClient) {}

  ngOnInit() {
    this.http.get<Entry[]>('/api/rows').subscribe(data => {
      this.entries = data;
    });
    this.http.get<Globals>('/api/globals').subscribe(data => {
      this.globals = data;
    });
  }
}
