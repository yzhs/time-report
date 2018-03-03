# Time Report

The purpose of this programme is two-fold: data entry and report generation.


## Front end



## Back end
The data is stored in a SQLite database by the backend with the following
tables:

* `holidays`
  A list of dates on which no one works. This includes the school holidays as
  well as general holidays.

  ``` sql
  CREATE TABLE holidays (date integer NOT NULL, title varchar NOT NULL)
  ```

* `weeks`
  Maybe use year/month/day format?
  | year               | week_of_year | week_type |
  |--------------------|--------------|-----------|
  | 2017..current year | 1..52        | [A-D]     |

  Note that week 1 is the first week that *starts* in a year, i.e. the week
  starting with the first Monday in January.

  The week type alternates (A -> B -> C -> D -> A…) from one week to the next
  *which contains a school day*, i.e. the week-long holidays are treated like a
  single weekend.

  The necessary date could be scraped from
  https://www.gymnasium-kreuztal.de/sites/default/files/docs/Wochen/A-D-Wochen_2017_2.pdf
  or something similar.

  ``` sql
  CREATE TABLE weeks (
    year integer NOT NULL
      CHECK (2017 <= year AND year <= cast(strftime('%y', 'now') as integer) + 1),

    week_of_year integer NOT NULL
      CHECK (0 < week_of_year AND week_of_year <= 52),

    type_of_week integer NOT NULL CHECK (0 <= type_of_week AND type_of_week < 4),

    PRIMARY KEY (year, week_of_year)
  )
  ```

* `employees`
  | id  | name   |
  |-----|--------|
  | int | string |

  ``` sql
  CREATE TABLE employees (
    id integer PRIMARY KEY AUTOINCREMENT,
    name varchar NOT NULL
  )
  ```

* `reports`
  | id  | title  | start_date | end_date | was_pdf_generated |
  |-----|--------|------------|----------|-------------------|
  | int | string | date       | date     | boolean           |

  ``` sql
  CREATE TABLE reports (
    id integer PRIMARY KEY AUTOINCREMENT,
    title varchar NOT NULL CHECK (length(title) > 7),
    start_date integer NOT NULL,
    end_date integer NOT NULL CHECK (
      start_date <= end_date AND
      end_date <= cast(strftime('%s', 'now') as integer) + 7*24*60*60
    ),
    was_pdf_generated boolean NOT NULL DEFAULT false
  )
  ```

* `items`
  Basically, who worked when for how long.

  | id  | report_id   | employee_id | start_time | end_time | remark |
  |-----|-------------|-------------|------------|----------|--------|
  | int | foreign key | foreign key | time       | time     | string |

  NB: SQLite does not support foreign keys by default. They have to be enabled
  for each connection using `PRAGMA foreign_keys = ON`.

  ``` sql
  CREATE TABLE items (
    id integer PRIMARY KEY AUTOINCREMENT,
    report_id integer NOT NULL REFERENCES reports(id),
    employee_id integer NOT NULL REFERENCES employees(id),


    start_datetime integer NOT NULL CHECK(
      date(start_datetime) >= '2017-08-01' AND
      time(start_datetime) >= '11:00'
    ),
    end_datetime integer NOT NULL CHECK (
      date(start_datetime) = date(end_datetime) AND
      start_datetime <= end_datetime - 15*60 AND
      time(end_datetime) <= '16:00'
    ),

    remark varchar
  )
  ```
