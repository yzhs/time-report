# Time Report

The purpose of this programme is two-fold: data entry and report generation.


## Front end



## Back end
The data is stored in a SQLite database by the backend with the following
tables:

* `holidays`
  A list of dates on which no one works. This includes the school holidays as
  well as general holidays.

  Columns: date, title
  Primary key: date

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

  The necessary dates could be scraped from
  https://www.gymnasium-kreuztal.de/docs.php?doc=Wochen

  Columns:
  * year ∈ ℕ ∩ [2017, current year + 1]
  * week_of_year ∈ ℕ ∩ [1, 52] (week 1 is the week containing the 4th of January)
  * type_of_week ∈ ℕ ∩ [0, 3],

  Primary key: (year, week_of_year)

* `employees`
  | id  | name   |
  |-----|--------|
  | int | string |


* `reports`
  | id  | title  | start_date | end_date | was_pdf_generated |
  |-----|--------|------------|----------|-------------------|
  | int | string | date       | date     | boolean           |

  Constraints:
  * length of title > 7
  * start_date <= end_date and  now + 1 week >= end_date
  * was_pdf_generated is true for at most one row in reports

* `items`
  Basically, who worked when for how long.

  | id  | report_id   | employee_id | start_time | end_time | remark |
  |-----|-------------|-------------|------------|----------|--------|
  | int | foreign key | foreign key | time       | time     | string |

  NB: SQLite does not support foreign keys by default. They have to be enabled
  for each connection using `PRAGMA foreign_keys = ON`.

  Constraints:
  * date(start_datetime) is no earlier than 2017-08-01
  * time(start_datetime) is no earlier than 11:00
  * date(start_datetime) = date(end_datetime)
  * end_datetime is no earlier than 15 minutes *after* start_datetime
  * time(end_datetime) is no later than 16:00
