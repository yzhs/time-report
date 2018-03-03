import { HttpClient } from '@angular/common/http';
import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';

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
  title: string;
  mindate: string;
  maxdate: string;
  mintime: string;
  maxtime: string;

  constructor(public titleService: Title,
    title = '', mindate = '2017-08-01',
    maxdate = '2018-07-31',
    mintime = '12:30',
    maxtime = '16:00') { }

  onChangeMe() {
    this.titleService.setTitle('Abrechnung BetreuerInnen ' + this.title);
  }
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  entries: Entry[] = [];
  globals: Globals = new Globals(this.titleService);

  constructor(private http: HttpClient, private titleService: Title) {}

  ngOnInit() {
    /*
    this.http.get<Entry[]>('/api/rows').subscribe(data => {
      this.entries = data;
      this.next();
    });
    */
   this.entries = [
     new Entry('Alice A.', '2018-01-07', 0, '13:30', '15:30'),
     new Entry('Bob b.', '2018-01-08', 0, '14:00', '15:30')
    ];
    this.http.get<Globals>('/api/globals').subscribe(data => {
      this.globals = data;
    });
  }

  next() {
    this.http.get<Entry>('/api/new_row').subscribe(data => {
      this.entries.push(data);
    });
  }
}
